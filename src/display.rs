use crate::memory::Memory;
use minifb::{Scale, Window, WindowOptions};
use std::fmt;

pub const WIDTH: u32 = 224;
pub const HEIGHT: u32 = 256;

pub struct Display {
    pub raster: Vec<u32>,
    pub vblank: bool,
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

const SCALE: usize = 8;
impl Display {
    pub fn new() -> Self {
        let mut window = Window::new(
            "Eighty Eighty",
            WIDTH as usize,
            HEIGHT as usize,
            WindowOptions {
                resize: true,
                scale: Scale::X4,
                ..WindowOptions::default()
            },
        )
        .unwrap();

        window.set_position(500, 500);
        Display {
            // TODO: Is there a better way to handle resize / different scaling?
            raster: vec![0x00FF_FFFF; WIDTH as usize * HEIGHT as usize * SCALE],
            vblank: false,
            window,
        }
    }

    pub fn draw_pixel(&mut self, memory: &Memory) {
        let memory = &memory.memory;

        // Iterate over VRAM
        for (i, byte) in (memory[0x2400..0x4000]).iter().enumerate() {
            let y = i as isize * 8 / 256;

            for shift in 0..(7 + 1) {
                let x = ((i * 8) % 256 as usize + shift as usize) as isize;
                // Rotate frame buffer 90 deg
                let new_x = y as isize;
                let new_y = (-x as isize + 256) - 1;

                let pixel = if byte.wrapping_shr(shift) & 1 == 0 {
                    0xFF00_0000 // Alpha
                } else if x <= 63 && (x >= 15 || x <= 15 && y >= 20 && y <= 120) {
                    0xFF00_FF00 // Green
                } else if x >= 200 && x <= 220 {
                    0xFF00_00FF // Red
                } else {
                    0xFFFF_FFFF // Black
                };
                self.raster[WIDTH as usize * new_y as usize + new_x as usize] = pixel;
            }
        }
    }
}
