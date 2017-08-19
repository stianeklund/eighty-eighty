use std::io::Cursor;
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};
use std::fmt;
use minifb::{Key, Scale, WindowOptions, Window};
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
        let mut window = Window::new(
            "Eighty Eighty",
            WIDTH,
            HEIGHT,
            WindowOptions {
                resize: true,
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap();


        Display {
            raster: vec![0x00FFFFFF; WIDTH * HEIGHT],
            vblank: false,
            draw_flag: true,
            window: window,
        }
    }
    pub fn render(&mut self, mut memory: &mut Memory) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let image_y = 255 - y;
                let offset = (WIDTH * image_y) + x;
                let frame_offset = (WIDTH * y) + x;
                // self.raster[frame_offset] = memory.memory[offset] as u32;
                // self.raster[x + (y * WIDTH)] = ((x + (y * HEIGHT) & 0xFF) * 1) as u32;
            }
        }
    }
    pub fn render_vram(&mut self, mut memory: &mut Memory) {
        // 0x2400 is the beginning of VRAM
        let mut base: u16 = 0x2400;

        // Iterate over all the memory locations in the VRAM memory map $2400 - $3FFF.
        // We want to read this into a buffer & point to the byte (8 bits) of the current memory loc
        // The video hardware is 7168 bytes (1bpp bitmap), 32 bytes per scan line.
        // We simply iterate over the entirety of the size of the video hardware
        // & iterate over the 8 pixels per byte.

        for offset in 0...(256 * 244 / 8) {
            let mut x: usize = 0;
            let mut y: usize = 224;
            let image_y = 224 - y;

            // Inner loop should split the byte into bits (8 pixels per byte)
            for shift in 0 ... 8 {
                if memory.memory[base as usize + offset as usize] as u16 >> shift & 1 != 0 {
                    self.raster[x * WIDTH + image_y] = 0x00FFFFFF;
                } else {
                    self.raster[x * WIDTH + image_y] = 0x00000000;
                }
                y -= 1;
                if y <= 0 {
                    y = 255;
                }
                x += 1;
            }
        }
    }
}
