pub mod memory;
pub mod cpu;
pub mod io;

use std::{collections::HashMap, fs::File, thread, time::{Duration, Instant}};
use minifb::{Key, Scale};

use crate::errors::Chip8Error;
use io::{IO, Color};
use memory::Memory;
use cpu::CPU;

pub const CPU_FREQ: f64 = 1.0 / 700.0;

pub struct Chip8 {
    cpu: CPU,
    mem: Memory,
    io: IO
}

impl Chip8 {
    pub fn new() -> Self {
        let cpu = CPU::new();
        let mem = Memory::new();
        let io = IO::new();
        Chip8 { cpu, mem, io }
    }

    pub fn load_program(&mut self, file: &File) -> Result<(), Box<dyn std::error::Error>> {
        self.mem.load_from_file(file)
    }

    pub fn run(&mut self) -> Result<(), Chip8Error> {
        self.io.display_init()?;

        let tick = Duration::from_secs_f64(CPU_FREQ);
        let mut next = Instant::now() + tick;

        while self.io.display_is_open() {
            let now = Instant::now();
            if now >= next {
                self.cpu.execute(&mut self.mem, &mut self.io)?;

                self.io.display_update()?;

                if self.cpu.sound_timer() > 0 {
                    self.io.audio_play();
                } else {
                    self.io.audio_pause();
                }

                next += tick
            } else {
                thread::sleep(next - now);
            }
        }

        self.cpu.shutdown();

        Ok(())
    }

    pub fn set_colors(&mut self, filled: Color, empty: Color) {
        self.io.display_set_colors(filled, empty);
    }

    pub fn set_keyboard_bindings(&mut self, bindings: HashMap<u8, Key>) {
        self.io.keyboard_set_bindings(bindings);
    }
}

