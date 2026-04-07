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

    pub fn execute(&mut self, opcode: u16, ram: &mut Chip8Ram, keyboard: &mut Chip8Keyboard) {
        let current_pc: u16 = self.pc;
        self.pc = self.pc.wrapping_add(2);

        let x: usize = ((opcode & 0x0F00) >> 8) as usize;
        let y: usize = ((opcode & 0x00F0) >> 4) as usize;
        let n: u8 = (opcode & 0x000F) as u8;
        let kk: u8 = (opcode & 0x00FF) as u8;
        let nnn: u16 = opcode & 0x0FFF;

        match (opcode & 0xF000) >> 12 {
            0x0 => match kk {
                0xE0 => self.op_cls(),
                0xEE => self.op_ret(),
                _ => println!("Unknown 0x0 opcode: 0x{:04X}", opcode),
            },

            0x1 => self.op_jp_addr(nnn),
            0x2 => self.op_call_addr(nnn),
            0x3 => self.op_se_vx_byte(x, kk),
            0x4 => self.op_sne_vx_byte(x, kk),
            0x5 => self.op_se_vx_vy(x, y),
            0x6 => self.op_ld_vx_byte(x, kk),
            0x7 => self.op_add_vx_byte(x, kk),

            0x8 => match n {
                0x0 => self.op_ld_vx_vy(x, y),
                0x1 => self.op_or_vx_vy(x, y),
                0x2 => self.op_and_vx_vy(x, y),
                0x3 => self.op_xor_vx_vy(x, y),
                0x4 => self.op_add_vx_vy(x, y),
                0x5 => self.op_sub_vx_vy(x, y),
                0x6 => self.op_shr_vx(x),
                0x7 => self.op_subn_vx_vy(x, y),
                0xE => self.op_shl_vx(x),
                _ => println!("Unknown 0x8 opcode: 0x{:04X}", opcode),
            },

            0x9 => self.op_sne_vx_vy(x, y),
            0xA => self.op_ld_i_addr(nnn),
            0xB => self.op_jp_v0_addr(nnn),
            0xC => self.op_rnd_vx_byte(x, kk),
            0xD => self.op_drw_vx_vy_nibble(x, y, n, ram),

            0xE => match kk {
                0x9E => self.op_skp_vx(x, keyboard),
                0xA1 => self.op_sknp_vx(x, keyboard),
                _ => println!("Unknown 0xE opcode: 0x{:04X}", opcode),
            },

            0xF => match kk {
                0x07 => self.op_ld_vx_dt(x),
                0x0A => self.op_ld_vx_key(x, keyboard, current_pc),
                0x15 => self.op_ld_dt_vx(x),
                0x18 => self.op_ld_st_vx(x),
                0x1E => self.op_add_i_vx(x),
                0x29 => self.op_ld_f_vx(x),
                0x33 => self.op_ld_b_vx(x, ram),
                0x55 => self.op_ld_i_vx(x, ram),
                0x65 => self.op_ld_vx_i(x, ram),
                _ => println!("Unknown 0xF opcode: 0x{:04X}", opcode),
            },

            _ => println!("Unknown opcode: 0x{:04X}", opcode),
        }
    }

    // 00E0
    fn op_cls(&mut self) {
        self.display.clear();
    }

    // 00EE
    fn op_ret(&mut self) {
        if let Some(addr) = self.stack.pop() {
            self.pc = addr;
        } else {
            println!("Stack underflow on RET");
        }
    }

    // 1nnn
    fn op_jp_addr(&mut self, addr: u16) {
        self.pc = addr;
    }

    // 2nnn
    fn op_call_addr(&mut self, addr: u16) {
        self.stack.push(self.pc);
        self.pc = addr;
    }

    // 3xkk
    fn op_se_vx_byte(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // 4xkk
    fn op_sne_vx_byte(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // 5xy0
    fn op_se_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // 6xkk
    fn op_ld_vx_byte(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
    }

    // 7xkk
    fn op_add_vx_byte(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
    }

    // 8xy0
    fn op_ld_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }

    // 8xy1
    fn op_or_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] |= self.v[y];
    }

    // 8xy2
    fn op_and_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] &= self.v[y];
    }

    // 8xy3
    fn op_xor_vx_vy(&mut self, x: usize, y: usize) {
        self.v[x] ^= self.v[y];
    }

    // 8xy4
    fn op_add_vx_vy(&mut self, x: usize, y: usize) {
        let (result, carry) = self.v[x].overflowing_add(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = carry as u8;
    }

    // 8xy5
    fn op_sub_vx_vy(&mut self, x: usize, y: usize) {
        self.v[0xF] = (self.v[x] >= self.v[y]) as u8;
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
    }

    // 8xy6
    fn op_shr_vx(&mut self, x: usize) {
        self.v[0xF] = self.v[x] & 0x01;
        self.v[x] >>= 1;
    }

    // 8xy7
    fn op_subn_vx_vy(&mut self, x: usize, y: usize) {
        self.v[0xF] = (self.v[y] >= self.v[x]) as u8;
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
    }

    // 8xyE
    fn op_shl_vx(&mut self, x: usize) {
        self.v[0xF] = (self.v[x] >> 7) & 0x01;
        self.v[x] <<= 1;
    }

    // 9xy0
    fn op_sne_vx_vy(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // Annn
    fn op_ld_i_addr(&mut self, addr: u16) {
        self.i = addr;
    }

    // Bnnn
    fn op_jp_v0_addr(&mut self, addr: u16) {
        self.pc = addr.wrapping_add(self.v[0] as u16);
    }

    // Cxkk
    fn op_rnd_vx_byte(&mut self, x: usize, kk: u8) {
        let rand_byte: u8 = get_rand_byte();
        self.v[x] = rand_byte & kk;
    }

    // Dxyn
    fn op_drw_vx_vy_nibble(&mut self, x: usize, y: usize, n: u8, ram: &Chip8Ram) {
        self.draw_sprite(x, y, n, ram);
    }

    // Ex9E
    fn op_skp_vx(&mut self, x: usize, keyboard: &mut Chip8Keyboard) {
        if keyboard.is_pressed(self.v[x]) {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // ExA1
    fn op_sknp_vx(&mut self, x: usize, keyboard: &mut Chip8Keyboard) {
        if !keyboard.is_pressed(self.v[x]) {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    // Fx07
    fn op_ld_vx_dt(&mut self, x: usize) {
        self.v[x] = self.delay_timer;
    }

    // Fx0A
    fn op_ld_vx_key(&mut self, x: usize, keyboard: &mut Chip8Keyboard, current_pc: u16) {
        if let Some(key) = keyboard.is_n_key_pressed() {
            self.v[x] = key;
        } else {
            self.pc = current_pc;
        }
    }

    // Fx15
    fn op_ld_dt_vx(&mut self, x: usize) {
        self.delay_timer = self.v[x];
    }

    // Fx18
    fn op_ld_st_vx(&mut self, x: usize) {
        self.sound_timer = self.v[x];
    }

    // Fx1E
    fn op_add_i_vx(&mut self, x: usize) {
        self.i = self.i.wrapping_add(self.v[x] as u16);
    }

    // Fx29
    fn op_ld_f_vx(&mut self, x: usize) {
        self.i = (self.v[x] as u16) * 5;
    }

    // Fx33
    fn op_ld_b_vx(&mut self, x: usize, ram: &mut Chip8Ram) {
        let value = self.v[x];
        ram.write(self.i as usize, value / 100);
        ram.write((self.i as usize) + 1, (value / 10) % 10);
        ram.write((self.i as usize) + 2, value % 10);
    }

    // Fx55
    fn op_ld_i_vx(&mut self, x: usize, ram: &mut Chip8Ram) {
        for offset in 0..=x {
            ram.write(self.i as usize + offset, self.v[offset]);
        }
    }

    // Fx65
    fn op_ld_vx_i(&mut self, x: usize, ram: &mut Chip8Ram) {
        for offset in 0..=x {
            self.v[offset] = ram.read(self.i as usize + offset);
        }
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
            let sprite_byte: u8 = ram.read(self.i as usize + row);

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
