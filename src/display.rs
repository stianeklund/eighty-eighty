use std::io::Cursor;
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};
use std::fmt;
use minifb::{Key, Scale, WindowOptions, Window};
use std::iter::Enumerate;
use std::thread::sleep;
use std::time::Duration;


use cpu::ExecutionContext;
use interconnect::Interconnect;
use memory::Memory;

pub const WIDTH: u32 = 224;
pub const HEIGHT: u32 = 256;

pub struct Display {
    pub raster: Vec<u32>,
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
            WIDTH as usize,
            HEIGHT as usize,
            WindowOptions {
                resize: true,
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap();


        Display {
            // 0x00FFFFFF;
            raster: vec![0x00FFFFFF; (HEIGHT as usize * WIDTH as usize) * 4],
            window: window,
        }
    }
    pub fn render(&mut self) {
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let image_y = 255 - y;
                let offset = (WIDTH * image_y) + x;
                let frame_offset = (WIDTH * y) + x;
                // self.raster[frame_offset] = sprite_sheet[offset] as u32;
                // self.raster[x + (y * WIDTH)] = ((x + (y * HEIGHT) & 0xFF) * 1) as u32;
            }
        }
    }

    pub fn draw_pixel(&mut self, interconnect: &Interconnect) {
        let memory = &interconnect.memory.memory;

        for (i, byte) in (memory[0x2400..0x4000]).iter().enumerate() {
            let y = i * 8 / WIDTH as usize + 1;
            for shift in 0..(8 + 1) {
                let x = ((i * 8) % WIDTH as usize) + shift as usize;

                let pixel = if (byte >> shift as usize) & 1 == 0 {
                    0xFF000000 // Alpha
                } else if x <= 63 && (x >= 15 || x <= 15 && y >= 20 && y <= 120) {
                    0xFF00FF00 // Green
                } else if x >= 200 && x <= 220 {
                    0x00FF0000 // Red
                } else {
                    0xFFFFFFFF // Black
                };
                self.raster[WIDTH as usize * y + x] = pixel;
            }
        }
    }
}


