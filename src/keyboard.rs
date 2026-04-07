use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Chip8Keyboard {
    keys: [bool; 16],
}

impl Chip8Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    fn key_down(&mut self, chip8_key: u8) {
        if chip8_key < 16 {
            self.keys[chip8_key as usize] = true;
        }
    }

    fn key_up(&mut self, chip8_key: u8) {
        if chip8_key < 16 {
            self.keys[chip8_key as usize] = false;
        }
    }

    pub fn is_n_key_pressed(&mut self) -> Option<u8> {
        for (i, &k) in self.keys.iter().enumerate() {
            if k {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn is_pressed(&mut self, chip8_key: u8) -> bool {
        self.keys[chip8_key as usize]
    }

    pub fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                if let Some(k) = Self::sdl_to_chip8(*key) {
                    self.key_down(k);
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                if let Some(k) = Self::sdl_to_chip8(*key) {
                    self.key_up(k);
                }
            }
            _ => {}
        }
    }

    pub fn sdl_to_chip8(key: Keycode) -> Option<u8> {
        match key {
            Keycode::Num1 => Some(0x1),
            Keycode::Num2 => Some(0x2),
            Keycode::Num3 => Some(0x3),
            Keycode::Num4 => Some(0xC),
            Keycode::Q => Some(0x4),
            Keycode::W => Some(0x5),
            Keycode::E => Some(0x6),
            Keycode::R => Some(0xD),
            Keycode::A => Some(0x7),
            Keycode::S => Some(0x8),
            Keycode::D => Some(0x9),
            Keycode::F => Some(0xE),
            Keycode::Z => Some(0xA),
            Keycode::X => Some(0x0),
            Keycode::C => Some(0xB),
            Keycode::V => Some(0xF),
            _ => None,
        }
    }
}
