use colored::Colorize;
use std::fs;

#[derive(Debug)]
pub struct Chip8Ram {
    memory: [u8; 4096],
}

impl Chip8Ram {
    pub fn new() -> Self {
        let mut ram: Chip8Ram = Self { memory: [0; 4096] };
        ram.load_fontset();
        ram
    }

    pub fn read(&self, addr: usize) -> u8 {
        self.memory[addr % 4096]
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.memory[addr % 4096] = value;
    }

    fn load_fontset(&mut self) {
        let fontset: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        let start: usize = 0x00;
        for (i, byte) in fontset.iter().enumerate() {
            self.memory[start + i] = *byte;
        }
    }

    fn load_rom(&mut self, data: &[u8]) {
        let start: usize = 0x200;

        if data.len() > (self.memory.len() - start) {
            panic!("ROM too large");
        }

        for (i, byte) in data.iter().enumerate() {
            self.memory[start + i] = *byte;
        }
    }

    pub fn get_rom_file(&mut self, path: &str) {
        let buffer: Vec<u8> = fs::read(path).expect("failed to read");
        self.load_rom(&buffer);
    }

    pub fn dump_memory(&self) {
        println!();
        for i in (0..self.memory.len()).step_by(16) {
            print!("{:03X}: ", i);

            for j in 0..16 {
                if self.memory[i + j] == 0 {
                    print!("{} ", format!("{:02X}", self.memory[i + j]).red());
                } else {
                    print!("{} ", format!("{:02X}", self.memory[i + j]).green());
                }
            }
            println!();
        }
        println!();
    }
}
