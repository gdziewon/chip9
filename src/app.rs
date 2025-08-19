use minifb::Key;
use minifb::{Window, WindowOptions, ScaleMode, Scale};

use crate::Chip9;
use crate::errors::Chip9Error;
use crate::chip9::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use crate::chip9::Keyboard;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;

const WINDOW_NAME: &str = "Chip9";
const CPU_FREQ: f64 = 1.0 / 700.0;

pub struct Emulator {
    window: Option<Window>,
    buffer: Vec<u32>,
    colors: Colors,
    bindings: Bindings
}

// todo: refactor from the ground up, maybe pixels + winit?
impl Emulator {
    pub fn new() -> Self {
        let buffer: Vec<u32> = vec![0; 64 * 32];
        let colors = Colors {
            filled: Color::from((0xFF, 0xFF, 0xFF)),
            empty: Color::from((0, 0, 0))
        };

        Self { window: None, buffer, colors, bindings: Bindings::default() }
    }

    pub fn run(&mut self, mut chip9: Chip9) -> Result<(), Chip9Error> {
        let window = Window::new(
            WINDOW_NAME,
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X16,
                scale_mode: ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .map_err(Chip9Error::WindowCreationError)?;

        self.window = Some(window);

        let tick = Duration::from_secs_f64(CPU_FREQ);
        let mut next = Instant::now() + tick;

        while self.window.as_ref().unwrap().is_open() {
            self.update_keyboard(&mut chip9.keyboard);
            self.render(&chip9.display)?;
            let now = Instant::now();
            if now >= next {
                chip9.execute()?; // todo: add audio
                next += tick
            } else {
                thread::sleep(next - now);
            }
        }

        Ok(())
    }

    fn render(&mut self, display: &Display) -> Result<(), Chip9Error> {
        let grid = display.grid();
        for j in 0..DISPLAY_HEIGHT {
            for i in 0..DISPLAY_WIDTH {
                let color = if grid[i][j] { &self.colors.filled } else { &self.colors.empty };
                self.buffer[i + j * DISPLAY_WIDTH] = color.value();
            }
        }
        self.window.as_mut().unwrap()
            .update_with_buffer(&self.buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .map_err(Chip9Error::WindowUpdateError)
    }

    fn update_keyboard(&self, keyboard: &mut Keyboard) {
        let window = self.window.as_ref().unwrap();
        let pressed_keys: Vec<u8> = window.get_keys()
            .iter()
            .filter_map(|key| self.bindings.get_chip9_key(key))
            .collect();
        keyboard.set_pressed(&pressed_keys);
    }
}

const DEFAULT_BINDINGS: [(Key, u8); 16] = [
    (Key::Key1, 0x1),
    (Key::Key2, 0x2),
    (Key::Key3, 0x3),
    (Key::Key4, 0xC),
    (Key::Q,    0x4),
    (Key::W,    0x5),
    (Key::E,    0x6),
    (Key::R,    0xD),
    (Key::A,    0x7),
    (Key::S,    0x8),
    (Key::D,    0x9),
    (Key::F,    0xE),
    (Key::Z,    0xA),
    (Key::X,    0x0),
    (Key::C,    0xB),
    (Key::V,    0xF),
];

// todo: refactor this from the ground up
struct Bindings(HashMap<Key, u8>);

impl Default for Bindings {
    fn default() -> Self {
        Self(HashMap::from(DEFAULT_BINDINGS))
    }
}

impl Bindings {
    pub fn get_chip9_key(&self, key: &Key) -> Option<u8> {
        self.0.get(key).copied()
    }
}

struct Colors {
    filled: Color,
    empty: Color
}

pub struct Color {
    value: u32,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { value: ((r as u32) << 16) | ((g as u32) << 8) | b as u32 }
    }
}

impl Color {
    fn value(&self) -> u32 {
        self.value
    }
}