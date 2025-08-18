pub struct Keyboard {
    pressed: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { pressed: [false; 16] }
    }

    /// Set pressed CHIP-8 keys (replace all)
    pub fn set_pressed(&mut self, pressed_keys: &[u8]) {
        self.pressed = [false; 16];
        for &key in pressed_keys {
            if key < 16 {
                self.pressed[key as usize] = true;
            }
        }
    }

    /// Is the CHIP-8 key pressed?
    pub fn is_key_pressed(&self, chip8_key: u8) -> bool {
        self.pressed.get(chip8_key as usize).copied().unwrap_or(false)
    }

    /// Get the first pressed CHIP-8 key, if any
    pub fn get_key_press(&self) -> Option<u8> {
        self.pressed.iter()
            .position(|&b| b)
            .map(|i| i as u8)
    }
}