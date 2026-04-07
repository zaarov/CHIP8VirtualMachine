use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl, VideoSubsystem};

const WINDOW_TITLE: &str = "CHIP8";
const WINDOW_WIDTH: u32 = 64;
const WINDOW_HEIGHT: u32 = 32;
const WINDOW_SCALE: u32 = 20;

pub struct Chip8Display {
    canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub pixels: [[bool; 64]; 32],
    black: Color,
    white: Color,
}

impl Chip8Display {
    pub fn new() -> Self {
        let sdl_context: Sdl = sdl2::init().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

        let window: Window = video_subsystem
            .window(
                WINDOW_TITLE,
                WINDOW_WIDTH * WINDOW_SCALE,
                WINDOW_HEIGHT * WINDOW_SCALE,
            )
            .position_centered()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        canvas
            .set_scale(WINDOW_SCALE as f32, WINDOW_SCALE as f32)
            .unwrap();

        let event_pump: EventPump = sdl_context.event_pump().unwrap();
        canvas.present();

        Self {
            canvas,
            event_pump,
            pixels: [[false; 64]; 32],
            black: Color::RGB(0, 0, 0),
            white: Color::RGB(255, 255, 255),
        }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(self.black);
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn xor_pixel(&mut self, x: usize, y: usize) -> bool {
        if x >= 64 || y >= 32 {
            return false;
        }

        let was_on: bool = self.pixels[y][x];
        self.pixels[y][x] = !was_on;
        was_on
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(self.black);
        self.canvas.clear();

        for y in 0..32 {
            for x in 0..64 {
                if self.pixels[y][x] {
                    self.canvas.set_draw_color(self.white);
                    let _ = self.canvas.draw_point((x as i32, y as i32));
                }
            }
        }
    }
}
