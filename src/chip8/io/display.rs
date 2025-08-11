use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};

use crate::errors::Chip8Error;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const DISPLAY_SCALE: Scale = Scale::X16;
const WINDOW_NAME: &str = "Chip8 Emulator";

// todo: refactor from the ground up, maybe pixels + winit?
pub struct Display {
    grid: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH],
    window: Option<Window>,
    buffer: Vec<u32>,
    colors: Colors,
    scale: Scale
}

impl Display {
    pub fn new() -> Self {
        let grid = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
        let buffer: Vec<u32> = vec![0; DISPLAY_WIDTH * DISPLAY_HEIGHT];
        let colors = Colors {
            filled: Color::from((0xFF, 0xFF, 0xFF)),
            empty: Color::from((0, 0, 0))
        };

        Display { grid, buffer, window: None, colors, scale: DISPLAY_SCALE }
    }

    pub(super) fn init(&mut self) -> Result<(), Chip8Error> {
        let window = Window::new(
            WINDOW_NAME,
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WindowOptions {
                resize: true,
                scale: self.scale,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )
        .map_err(Chip8Error::WindowCreationError)?;

        self.window = Some(window);
        Ok(())
    }

    pub fn get_key_press(&self, keyboard: &super::Keys) -> Option<u8> {
        self.window.as_ref().unwrap().get_keys_pressed(KeyRepeat::No)
        .iter()
        .find_map(|&k| keyboard.get_chip8_key(&k))
        .copied()
    }

    pub(super) fn is_key_down(&self, key: Key) -> bool {
        self.window.as_ref().unwrap().is_key_down(key)
    }

    pub(super) fn is_open(&self) -> bool {
        match self.window.as_ref() {
            Some(window) => window.is_open(),
            None => false,
        }
    }

    pub(super) fn set_colors(&mut self, filled: Color, empty: Color) {
        self.colors.filled = filled;
        self.colors.empty = empty;
    }

    // Update the display
    pub(super) fn update(&mut self) -> Result<(), Chip8Error>{
        // Draw a grid
        self.update_buffer();

        // Update the window with buffer
        self.window.as_mut().unwrap()
            .update_with_buffer(&self.buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .map_err(Chip8Error::WindowUpdateError)

    }

    pub(super) fn clear(&mut self) {
        self.grid = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
        self.update_buffer();
    }

    pub(super) fn draw(&mut self, horizontal_pos: usize, vertical_pos: usize, sprite: impl Iterator<Item = u8>) -> bool {
        let mut collision = false;
        for (j, byte) in sprite.enumerate() {
            for i in 0..8 {
                let xi = (horizontal_pos + i) % DISPLAY_WIDTH;
                let yj = (vertical_pos + j) % DISPLAY_HEIGHT;
                let old = self.grid[xi][yj];
                let new = (byte & (0x80 >> i)) != 0;
                self.grid[xi][yj] ^= new;
                collision |= old && !self.grid[xi][yj];
            }
        }
        collision
    }

    // Update buffer with grid
    fn update_buffer(&mut self) {
        for j in 0..DISPLAY_HEIGHT {
            for i in 0..DISPLAY_WIDTH {
                let color = if self.grid[i][j] { &self.colors.filled } else { &self.colors.empty };
                self.buffer[i + j * DISPLAY_WIDTH] = color.value();
            }
        }
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