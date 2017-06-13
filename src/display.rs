
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

impl fmt::UpperHex for Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:02X}", val)
    }
}


impl Display {
    pub fn new() -> Display {
        let mut window = Window::new("Eighty Eighty",
                                     WIDTH,
                                     HEIGHT,
                                     WindowOptions {
                                         resize: false,
                                         scale: Scale::X2,
                                         ..WindowOptions::default()
                                     }).unwrap();


        Display {
            raster: vec![0; WIDTH * HEIGHT],
            vblank: false,
            draw_flag: true,
            window: window,
        }
    }


    pub fn render_vram(&mut self, memory: &mut Memory) {

        // 0x2400 is the beginning of VRAM
        let mut base: u16 = 0x2400;
        let mut offset: u16 = 0;
        let mut x: u8 = 0;
        let mut y: u8 = 255;

        let mut counter: u8 = 0;

        // Iterate over all the memory locations in the VRAM memory map $2400 - $3FFF.
        // We want to read this into a buffer & point to the byte (8 bits) of the current memory loc

        // The video hardware is 7168 bytes (1bpp bitmap), 32 bytes per scanline.
        // We simply iterate over the entirity of the size of the video hardware
        // & iterater over the 8 pixels per byte.

        let memory = &mut memory.memory;

        for offset in 0..(256 * 244 / 8) {

            for shift in 0..8 {
                // Inner loop should split the byte into bits (8 pixels per byte)
                if (memory[base as usize + offset as usize] >> shift) & 1 != 0 {
                    self.raster[counter as usize] = 0x0000000;
                } else {
                    self.raster[counter as usize] = 0x0FFFFFF;
                }
                y = y.wrapping_sub(1);
                if y < 0 {
                    y = 255;
                }
                x = x.wrapping_add(1);
            }
            // println!("X: {}, Y: {}", x, y);
            counter = counter.wrapping_add(1);
        }

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                self.raster[y as usize * 224 + x as usize];
            }
        }
        // We essentially are presenting the already iterated frame buffer
        // at this point.
        self.window.update_with_buffer(&self.raster);
    }
    pub fn update_screen(&mut self, x: usize, y: usize) {

        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                self.raster[y.wrapping_mul(224).wrapping_add(x)];
            }
        }
    }
}
