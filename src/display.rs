extern crate sdl2;

use std::fmt;
use super::sdl2::Sdl;
use super::sdl2::pixels::Color;
use super::sdl2::rect::Rect;
use super::memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;


pub struct Display {
    renderer: sdl2::render::Renderer<'static>,
    raster: Box<([u8; 100000])>,
    vblank: bool,
    memory: memory::Memory,
    draw_flag: bool,
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:?}", val)
    }
}

impl Display {
    pub fn new(ctx: &sdl2::Sdl) -> Display {

        let memory = memory::Memory::new();
        // Initialize SDL2
        let video = ctx.video().unwrap();

        // Create window
        let window = video.window("Eighty Eighty", WIDTH as u32 * 2, HEIGHT as u32 * 2)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window.renderer()
            .accelerated()
            .build()
            .expect("Initialization of window renderer failed");

        Display {
            renderer: renderer,
            raster: Box::new([0; 100000]),
            memory: memory,
            vblank: false,
            draw_flag: true,
        }
    }


    pub fn render_vram(&mut self) {
        // 0x2400 is the beginning of VRAM
        let mut base: u16 = 0x2400;
        let mut offset: u16 = 0;

        let mut counter = 0;
        // Iterate over all the memory locations from addr: $2400 - $3FFF (offset) reading it into memory
        // and point to the byte of the current memory location
        for offset in 0..(256 * 244 / 8) {
            // println!("VRAM value: {:?}", self.memory);
            for shift in 0..8 {
                // Inner loop should split the byte into bits (8 pixels per byte)
                if (self.memory.read(base as usize + offset as usize) >> shift) & 1 != 1 {
                    self.raster[offset as usize] = 0x0000;

                } else {
                    self.raster[offset as usize] = 0xFFFF;
                }
            }
            counter += 1;
        }
    }

    pub fn draw(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.raster[y.wrapping_mul(WIDTH) + x] != 0 {

                    // Foreground
                    self.renderer.set_draw_color(Color::RGB(251, 241, 199));
                } else {
                    // Background
                    self.renderer.set_draw_color(Color::RGB(69, 133, 149));
                }
                self.renderer.fill_rect(Rect::new(x as i32 * 2, y as i32 *2, 15, 15)).unwrap();
            }
        }
        self.renderer.present();
        self.draw_flag = true;
    }
}
