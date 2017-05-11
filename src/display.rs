
use std::fmt;
use super::minifb::{Key, Scale, WindowOptions, Window};
// use super::interconnect;
use std::borrow::BorrowMut;
use std::iter::Enumerate;

use cpu::ExecutionContext;
use memory::Memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;

pub struct Display {
    pub raster: Vec<(u32)>,
    vblank: bool,
    draw_flag: bool,
    pub window: Window,
}

impl fmt::Debug for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:?}", val)
    }
}

impl Display {
    pub fn new() -> Display {
    let mut window = Window::new("Eighty Eighty", WIDTH, HEIGHT, WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()}).unwrap();


        Display {
            raster: vec![0; WIDTH * HEIGHT],
            vblank: false,
            draw_flag: true,
            window: window,
        }
    }


    pub fn render_vram(&mut self) {

        let m = Memory::new();
        let memory = m.memory;
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

                if (memory[base as usize + offset as usize] >> shift) & 1 != 0 {
                    self.raster[counter as usize] = 0x00;

                } else {
                    self.raster[counter as usize] = 0xFF;
                }
            }

            y -= 1;
            if y <= 0 {
                y = 255;
                x += 1;
            }
            counter += 1;
        }
    }
    pub fn update_screen(&mut self) {
        
        let m = Memory::new();
        let memory = m.memory;
        let mut iter = memory.iter();
        // println!("Current memory value: {:?}", iter);

        let sprite_x = self.raster[0] as usize;
        let sprite_y = self.raster[1] as usize;

        // Pixels can either be on or off
        let mut flipped = false;

        for j in 0..HEIGHT {
            let row = memory[(0x2400 as usize + j as usize)];
            for i in 0..WIDTH {
                let xi = ((sprite_x + i as usize) % WIDTH) as usize;
                let yj = ((sprite_y + j as usize) % HEIGHT) as usize;

                if row & 0x80u8.wrapping_shl(i as u32) != 0 {
                        self.raster[row as usize];
                    }
                }
            self.window.update_with_buffer(&self.raster.as_slice());
        }
    }
}
