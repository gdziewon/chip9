use std::{
    sync::{
        atomic::{AtomicU8, Ordering}, mpsc, Arc
    },
    thread::{self, JoinHandle},
    time::Duration
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
        let _ = self.value.fetch_update(
            Ordering::SeqCst,
            Ordering::Relaxed,
            |v| if v > 0 { Some(v - 1) } else { None },
        );
    }
}

enum TimerCmd { Pause, Resume, Shutdown }

pub struct TimerClock {
    timers: Vec<Arc<Timer>>,
    tx: Option<mpsc::Sender<TimerCmd>>,
    handle: Option<JoinHandle<()>>,
}

impl TimerClock {
    pub fn new() -> Self {
        Self {
            timers: Vec::new(),
            tx: None,
            handle: None,
        }
    }

    pub fn register(&mut self, timer: Arc<Timer>) {
        self.timers.push(timer);
    }

    pub fn start(&mut self) {
        if self.tx.is_some() {
            return; // already started
        }

        let (tx, rx) = mpsc::channel::<TimerCmd>();
        let timers = self.timers.clone();

        let tick = Duration::from_secs_f64(TIMER_FREQ);

        let handle = thread::spawn(move || {
            let mut paused = false;
            loop {
                match rx.recv_timeout(tick) {
                    Ok(TimerCmd::Pause) => { paused = true; }
                    Ok(TimerCmd::Resume) => { paused = false; }
                    Ok(TimerCmd::Shutdown) => break,
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        if !paused {
                            for t in &timers {
                                t.tick();
                            }
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        self.tx = Some(tx);
        self.handle = Some(handle);
    }

    pub fn pause(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(TimerCmd::Pause);
        }
    }

    pub fn resume(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(TimerCmd::Resume);
        }
    }

    pub fn shutdown(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(TimerCmd::Shutdown);
        }
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