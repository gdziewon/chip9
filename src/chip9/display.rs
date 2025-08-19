const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;


pub struct Display {
    pub grid: [[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH], // todo: refactor
}

impl Display {
    pub fn new() -> Self {
        let grid = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];

        Self { grid }
    }

    pub(super) fn clear(&mut self) {
        self.grid = [[false; DISPLAY_HEIGHT]; DISPLAY_WIDTH];
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

    pub fn grid(&self) -> &[[bool; DISPLAY_HEIGHT]; DISPLAY_WIDTH] {
        &self.grid
    }
}