use std::fmt;
use minifb::{Scale, WindowOptions, Window};
use interconnect::Interconnect;

// Actual raster size
pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 224;

// Frame buffer size (temporary size TODO fix shearing issue)
const FB_WIDTH: u32 = 256;
const FB_HEIGHT: u32 = 256;

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
            FB_WIDTH as usize,
            FB_HEIGHT as usize,
            WindowOptions {
                resize: false,
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap();


        Display {
            raster: vec![0x00FF_FFFF; WIDTH as usize * HEIGHT as usize * 2],
            window,
        }
    }
    pub fn draw_pixel(&mut self, interconnect: &Interconnect) {
        let memory = &interconnect.memory.memory;

        for (i, byte) in (memory[0x2400..0x4000]).iter().enumerate() {
            let y = i * 8 / (WIDTH as usize);

            for shift in 0..(7 + 1) {
                let x = ((i * 8) % WIDTH as usize + shift as usize) as isize;

                let new_x = y as isize;
                let new_y = -x as isize + 256;

                let pixel = if byte.wrapping_shr(shift) & 1 == 0 {
                    0xFF00_0000 // Alpha
                } else {
                    0xFFFF_FFFF // Black
                };
                self.raster[WIDTH as usize * new_y as usize + new_x as usize] = pixel;
            }
        }
    }
}
