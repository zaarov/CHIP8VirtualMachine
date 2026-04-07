use crate::display::Chip8Display;
use crate::keyboard::Chip8Keyboard;
use crate::ram::Chip8Ram;

use crate::utils::get_rand_byte;

pub struct Chip8CPU {
    pub pc: u16,
    i: u16,
    v: [u8; 16],
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    pub display: Chip8Display,
}

impl Chip8CPU {
    pub fn new(display: Chip8Display) -> Self {
        Self {
            pc: 0x200,
            v: [0; 16],
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            display,
        }
    }

    pub fn cycle(&mut self, ram: &mut Chip8Ram, keyboard: &mut Chip8Keyboard) {
        let opcode: u16 =
            ((ram.read(self.pc as usize) as u16) << 8) | ram.read((self.pc + 1) as usize) as u16;

        self.execute(opcode, ram, keyboard);
    }

    pub fn execute(&mut self, opcode: u16, ram: &mut Chip8Ram, _keyboard: &mut Chip8Keyboard) {
        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
        let n: u8 = (opcode & 0x000F) as u8;
        let kk: u8 = (opcode & 0x00FF) as u8;
        let nnn: u16 = opcode & 0x0FFF;

        let mut new_pc: u16 = self.pc + 2;

        match (opcode & 0xF000) >> 12 {
            0x0 => match kk {
                0xE0 => self.display.clear(),
                0xEE => {
                    new_pc = self.stack.pop().unwrap();
                }
                _ => println!("Unknown 0x0 opcode: 0x{:04X}", opcode),
            },
            0x1 => new_pc = nnn,
            0x2 => {
                self.stack.push(self.pc + 2);
                new_pc = nnn;
            }
            0x3 => {
                if self.v[x] == kk {
                    new_pc = self.pc + 4;
                }
            }
            0x4 => {
                if self.v[x] != kk {
                    new_pc = self.pc + 4;
                }
            }
            0x5 => {
                if self.v[x] == self.v[y] {
                    new_pc = self.pc + 4;
                }
            }
            0x6 => self.v[x] = kk,
            0x7 => self.v[x] = self.v[x].wrapping_add(kk),
            0x8 => match n {
                0x0 => self.v[x] = self.v[y],
                0x1 => self.v[x] |= self.v[y],
                0x2 => self.v[x] &= self.v[y],
                0x3 => self.v[x] ^= self.v[y],
                0x4 => {
                    self.v[0xF] = 0;
                    if self.v[x] as i32 + self.v[y] as i32 > 255 {
                        self.v[0xF] = 1;
                    }
                    self.v[x] = self.v[x].wrapping_add(self.v[y])
                }
                0x5 => {
                    self.v[0xF] = 0;
                    if self.v[x] > self.v[y] {
                        self.v[0xF] = 1;
                    }
                    self.v[x] = self.v[x].wrapping_sub(self.v[y])
                }
                0x6 => {
                    self.v[0xF] = self.v[x] & 0x01;
                    self.v[x] = self.v[x] >> 1;
                }
                0x7 => {
                    self.v[0xF] = 0;
                    if self.v[y] > self.v[x] {
                        self.v[0xF] = 1;
                    }
                    self.v[x] = self.v[y].wrapping_sub(self.v[x])
                }
                0xE => {
                    self.v[0xF] = self.v[x] & 0x80;
                    self.v[x] = self.v[x] << 1;
                }

                _ => println!("Unknown 0x8 opcode: 0x{:04X}", opcode),
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    new_pc = self.pc + 4;
                }
            }
            0xA => self.i = nnn,
            0xB => new_pc = nnn + self.v[0] as u16,
            0xC => {
                let rand_byte: u8 = get_rand_byte();
                self.v[x] = rand_byte & kk;
            }
            0xD => self.draw_sprite(x, y, n, ram),
            0xE => match kk {
                0x9E => {
                    if _keyboard.is_pressed(self.v[x]) {
                        new_pc = self.pc + 4;
                    }
                }
                0xA1 => {
                    if !_keyboard.is_pressed(self.v[x]) {
                        new_pc = self.pc + 4;
                    }
                }
                _ => println!("Unknown 0xE opcode: 0x{:04X}", opcode),
            },
            0xF => match kk {
                0x07 => self.v[x] = self.delay_timer,
                0x0A => {
                    if let Some(key) = _keyboard.is_n_key_pressed() {
                        self.v[x] = key;
                    } else {
                        new_pc = self.pc
                    }
                }
                0x15 => self.delay_timer = self.v[x],
                0x18 => self.sound_timer = self.v[x],
                0x1E => self.i += self.v[x] as u16,
                0x29 => self.i = (self.v[x] as u16 & 0x0F) * 5,
                0x33 => {
                    ram.write(self.i as usize, self.v[x] / 100);
                    ram.write((self.i as usize) + 1, (self.v[x] / 10) % 10);
                    ram.write((self.i as usize) + 2, self.v[x] % 10);
                }
                0x55 => {
                    for offset in 0..x + 1 {
                        ram.write(self.i as usize + offset, self.v[offset]);
                    }
                }
                0x65 => {
                    for offset in 0..x + 1 {
                        self.v[offset] = ram.read(self.i as usize + offset);
                    }
                }

                _ => println!("Unknown 0xF opcode: 0x{:04X}", opcode),
            },
            _ => println!("Unknown opcode: 0x{:04X}", opcode),
        }

        self.pc = new_pc;
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn draw_sprite(&mut self, x: usize, y: usize, height: u8, ram: &Chip8Ram) {
        let start_x: i32 = self.v[x] as i32;
        let start_y: i32 = self.v[y] as i32;

        self.v[0xF] = 0;

        for row in 0..height as usize {
            let sprite_byte: u8 = ram.read((self.i as usize + row) as usize);
            for col in 0..8 {
                if (sprite_byte & (0x80 >> col)) != 0 {
                    let px: usize = ((start_x + col as i32) % 64) as usize;
                    let py: usize = ((start_y + row as i32) % 32) as usize;
                    if self.display.xor_pixel(px, py) {
                        self.v[0xF] = 1;
                    }
                }
            }
        }
    }
}
