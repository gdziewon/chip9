use std::{
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant}
};

const TIMER_FREQ: f64 = 1.0 / 60.0; // 60Hz - shouldn't be changed

pub struct Timer {
    value: AtomicU8,
}

impl Timer {
    pub fn new() -> Self {
        Self { value: AtomicU8::new(0) }
    }
    pub fn load(&self, val: u8) { self.value.store(val, Ordering::Relaxed); }
    pub fn get(&self) -> u8 { self.value.load(Ordering::Relaxed) }
    pub fn tick(&self) {
        let v = self.value.load(Ordering::Relaxed);
        if v > 0 {
            self.value.fetch_sub(1, Ordering::Relaxed);
        }
    }
}

pub struct TimerClock {
    paused: Arc<AtomicBool>,
    shutdown: Arc<AtomicBool>,
    timers: Vec<Arc<Timer>>,
    handle: Option<JoinHandle<()>>,
}

impl TimerClock {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
            paused: Arc::new(AtomicBool::new(false)),
            shutdown: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    pub fn register(&mut self, timer: Arc<Timer>) {
        self.timers.push(timer);
    }

    pub fn start(&mut self) {
        let timers = self.timers.clone();
        let paused = self.paused.clone();
        let shutdown = self.shutdown.clone();

        let handle = thread::spawn(move || {
            let tick = Duration::from_secs_f64(TIMER_FREQ);
            let mut next = Instant::now() + tick;
            while !shutdown.load(Ordering::Relaxed) {
                let now = Instant::now();

                if paused.load(Ordering::Relaxed) {
                    thread::sleep(tick);
                    continue;
                }

                if now >= next {
                    for t in &timers {
                        t.tick();
                    }
                next += tick;
                } else {
                    thread::sleep(next - now);
                }
            }
        });
        self.handle = Some(handle);
    }

    pub fn pause(&self) { self.paused.store(true, Ordering::Relaxed); }
    pub fn resume(&self) { self.paused.store(false, Ordering::Relaxed); }
    pub fn shutdown(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for TimerClock {
    fn drop(&mut self) {
        self.shutdown();
    }
}