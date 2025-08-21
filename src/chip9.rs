pub mod cpu;
mod display;
mod keyboard;

use std::fs::File;

use crate::errors::Chip9Error;
use cpu::CPU;
pub use display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
pub use keyboard::Keyboard;

pub struct Chip9 {
    cpu: CPU,
    pub display: Display, // fixme
    pub keyboard: Keyboard,
}

impl Chip9 {
    pub fn new() -> Self {
        let cpu = CPU::new();
        let display = Display::new();
        let keyboard = Keyboard::new();

        Self {
            cpu,
            display,
            keyboard,
        }
    }

    pub fn tick(&mut self) -> Result<(), Chip9Error> {
        self.cpu.execute(&mut self.display, &mut self.keyboard)
    }

    pub fn load_program(&mut self, file: &File) -> Result<(), Box<dyn std::error::Error>> {
        self.cpu.load_program(file)
    }

    pub fn shutdown(&mut self) { // should be called only on started
        self.cpu.shutdown();
    }
}