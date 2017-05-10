
use std::fmt;
use super::minifb::{Key, Scale, WindowOptions, Window};
use super::memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;

pub struct Display {
    pub raster: Vec<(u32)>,
    vblank: bool,
    memory: memory::Memory,
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

        let memory = memory::Memory::new();
        let mut window = Window::new("Eighty Eighty", WIDTH, HEIGHT,
                                     WindowOptions {
                                         resize: false,
                                         scale: Scale::X2,
                                         ..WindowOptions::default()
                                     }).unwrap();


        Display {
            raster: vec![0; WIDTH * HEIGHT],
            memory: memory,
            vblank: false,
            draw_flag: true,
            window: window,
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
        self.window.update_with_buffer(&self.raster.as_slice());
    }
}
