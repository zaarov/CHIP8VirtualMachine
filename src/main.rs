use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::time::Duration;

mod utils;

mod display;
use display::Chip8Display;

mod keyboard;
use keyboard::Chip8Keyboard;

mod ram;
use ram::Chip8Ram;

mod cpu;
use cpu::Chip8CPU;

/*
struct Chip8Motherboard {
    cpu: Chip8CPU,
    ram: Chip8Ram,
    display: Chip8Display,
    keyboard: Chip8Keyboard,
}
*/

pub fn main() {
    let mut ram: Chip8Ram = Chip8Ram::new();
    let display: Chip8Display = Chip8Display::new();
    let mut keyboard: Chip8Keyboard = Chip8Keyboard::new();
    let mut cpu: Chip8CPU = Chip8CPU::new(display);

    let argument: String = env::args().nth(1).expect("Please provide an argument");
    let path: String = format!("../test_roms/roms/{}", argument);
    ram.get_rom_file(&path);

    ram.dump_memory();
    'running: loop {
        let events: Vec<Event> = cpu.display.event_pump.poll_iter().collect();

        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    let opcode: u16 = ((ram.read(cpu.pc as usize) as u16) << 8)
                        | ram.read((cpu.pc + 1) as usize) as u16;

                    println!("PC: 0x{:04X} | Opcode: 0x{:04X}", cpu.pc, opcode);

                    cpu.cycle(&mut ram, &mut keyboard);
                    cpu.display.present();
                }

                _ => {
                    keyboard.handle_sdl_event(&event);
                }
            }
        }
        for _ in 0..16 {
            cpu.cycle(&mut ram, &mut keyboard);
        }
        cpu.update_timers();
        cpu.display.render();
        cpu.display.present();
        ram.dump_memory();
        std::thread::sleep(Duration::from_millis(1000 / 60));
    }
    println!();
    println!("Emulator stopped.");
}
