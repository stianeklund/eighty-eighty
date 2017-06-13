extern crate sdl2;

use std::fmt;
use super::sdl2::Sdl;
use super::sdl2::pixels::Color;
use super::sdl2::rect::Rect;
use super::memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;


pub struct Display {
    pub renderer: sdl2::render::Renderer<'static>,
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
        let mut x: u8 = 0;
        let mut y: u8 = 255;

        let mut counter = 0;
        // Iterate over all the memory locations in the VRAM memory map $2400 - $3FFF.
        // We want to read this into a buffer & point to the byte (8 bits) of the current memory loc

        // The video hardware is 7168 bytes (1bpp bitmap), 32 bytes per scanline.
        // We simply iterate over the entirity of the size of the video hardware
        // & iterater over the 8 pixels per byte.

        for offset in 0..(256 * 244 / 8) {

            for shift in 0..8 {
                // Inner loop should split the byte into bits (8 pixels per byte)

                if (self.memory.memory[base as usize + offset as usize] >> shift) & 1 != 0 {
                    self.raster[counter as usize] = 0x00;

                } else {
                    self.raster[counter as usize] = 0xFF;
                    // self.raster[y as usize * 224 + x as usize];
                }

                // When x is bigger than 256, reset x & increase y by one.
            }
            y = y.wrapping_sub(1);
            if y <= 0 {
                let mut y = 255;
            }
            x = x.wrapping_add(1);
            counter += 1;
        }
        self.renderer.set_draw_color(Color::RGB(251, 241, 199));
        self.renderer.fill_rect(Rect::new(x as i32, y as i32, 15, 15)).unwrap();
        // println!("X:{}, Y:{}", x, y);
        self.renderer.present();
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.raster[y * WIDTH + x] != 0 {
                    // Foreground
                    self.renderer.set_draw_color(Color::RGB(251, 241, 199));
                } else {
                    // Background
                    self.renderer.set_draw_color(Color::RGB(69, 133, 149));
                }
            }
        }
        self.renderer.fill_rect(Rect::new(x as i32 * 2, y as i32 * 2, 15, 15)).unwrap();
    }

    pub fn draw(&mut self) {
        // Foreground
        // Background
        self.renderer.clear();
        self.renderer.present();
    }
}
