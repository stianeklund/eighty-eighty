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
                resize: false,
                scale: Scale::X2,
                ..WindowOptions::default()
            },
        ).unwrap();


        Display {
            raster: vec![0; WIDTH * HEIGHT * 4],
            vblank: false,
            draw_flag: true,
            window: window,
        }
    }
    pub fn create_fb(&mut self, memory: &mut Memory) -> Vec<u32> {
        // Create 32bit bitmap array that can be used for rendering
        let mut buffer: Vec<u32> = memory
            .memory
            .chunks(4)
            .map(|buf| {
                let buf = Cursor::new(buf).read_u32::<LittleEndian>().unwrap();
                buf
            })
            .collect();
        buffer
    }


    pub fn render_vram(&mut self, mut memory: &mut Memory) {
        // 0x2400 is the beginning of VRAM
        let mut base: u16 = 0x2400;
        let mut offset: u16 = 0;
        let mut x: u8 = 0;
        let mut y: u8 = 255;

        let mut counter: u8 = 0;

        // Iterate over all the memory locations in the VRAM memory map $2400 - $3FFF.
        // We want to read this into a buffer & point to the byte (8 bits) of the current memory loc

        // The video hardware is 7168 bytes (1bpp bitmap), 32 bytes per scan line.
        // We simply iterate over the entirety of the size of the video hardware
        // & iterate over the 8 pixels per byte.


        for offset in 0..(256 * 244 / 8) {
            for shift in 0..8 {
                // Inner loop should split the byte into bits (8 pixels per byte)
                if (memory.memory[base as usize + offset as usize] >> shift) & 1 != 0 {
                    self.raster[counter as usize] = 0x00000000;
                } else {
                    self.raster[counter as usize] = 0x0FFFFFFF;
                }
                y = y.wrapping_sub(1);
                if y < 0 {
                    y = 255;
                }
                x = x.wrapping_add(1);
            }
            counter = counter.wrapping_add(1);
           //  self.draw(x as usize, y as usize, memory);
        }

        self.window.update_with_buffer(&self.raster);
    }

    // TODO
    pub fn draw(&mut self, x: usize, y: usize, mut memory: &mut Memory) {
        let mut sprite_value = 0;

        let sprite_sheet = self.create_fb(memory);
        let sprite_w = 50;
        let sprite_h = 50;

        let index_x = sprite_w * (sprite_value % 50);
        let index_y = sprite_h * (sprite_value / 50);
        let tile_w = index_x + sprite_w;
        let tile_h = index_y + sprite_h;


        let mut offset = 0;
        let mut line = 0;
        for i in index_y..tile_h {
            for j in index_x..tile_w {
                // self.raster[x + WIDTH * y] // = memory.memory[j + (i * HEIGHT)] as u32;
                self.raster[x + line + WIDTH * offset] = sprite_sheet[j + (i * HEIGHT)] as u32;
                line += 1;
            }
            line = 0;
            offset += 1;
        }
        // self.window.update_with_buffer(&self.raster).unwrap();
    }
}
