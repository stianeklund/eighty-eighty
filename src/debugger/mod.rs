use super::minifb::{Key, Scale, WindowOptions, Window};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

mod font;

use std::io::prelude;
use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use display::Display;
use memory::Memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 256;

pub struct DebugFont {
    pub bitmap: Vec<u8>,
}

impl DebugFont {
    pub fn new() -> DebugFont {
        let mut font = DebugFont { bitmap: Vec::<u8>::new() };

        // The block of bytes at the start of the file is the header.
        // The first 2 bytes of the BMP file format are the character "B"
        // then the character "M" in ASCII encoding.
        // All of the integer values are stored in little-endian (LSB first)

        // In our case we can likely omit the header all together.
        // Alternatively, we can check whether or not there is a match for
        // either "B" or "M" (in ASCII) to validate the file then skip the rest.

        // TODO: Improve path handling.
        let path = Path::new("/home/stian/dev/projects/eighty-eighty/assets/font.bmp");
        let mut file = File::open(&path).expect("File not found");
        let mut file_data = Vec::<u8>::new();

        // Skip BMP header & DIB for now.
        file.seek(SeekFrom::Start(54)).expect("Seek error");
        let result = file.read_to_end(&mut file_data);


        match result {
            Ok(result) => println!("Read {:?}: Bitmap {} bytes", &path, result),
            Err(e) => panic!("IO Error:: {}", e),
        }
        // This may not be entirely correct, but for now lets just
        // read the bitmap data and assign it to the bitmap vec.
        font.bitmap = file_data;
        // println!("Bitmap: {:?}", font.bitmap);
        font
    }
}

// TODO: Implement a way to display Cpu register values & memory pages.
//
// I.e displaying VRAM page values & main memory values.
// We also want to be able to peek at Cpu register values.
// Displaying it all in one nice window vs printing a ton of text.
// Whether or not that is possible with the current infrastructure I don't know
pub struct Debugger {
    pub font: DebugFont,
    pub window: Window,
    pub buffer: Vec<u32>,
    pub fb: Vec<u32>,
    pub memory_page: Vec<u32>,
}



impl Debugger {
    pub fn new() -> Debugger {
        let mut window = Window::new("Debugger",
                                     WIDTH,
                                     HEIGHT,
                                     WindowOptions {
                                         scale: Scale::X2,
                                         ..WindowOptions::default()
                                     })
                .unwrap();

        window.set_position(1250, 340);

        Debugger {
            buffer: vec![0; WIDTH * HEIGHT],
            font: DebugFont::new(),
            window: window,
            fb: vec![0; 65536],
            memory_page: vec![0; 65536],
        }
    }

    // Cursor wraps another type & provides it with a Seek implementation
    // Chunks is a iterator that provides us with a slice
    // Map provides a closure for this & creates an iterator
    // which calls that closure on each element.
    // We need a chunk size of 4 because 8 * 4 = 32 and we need a u32 to present
    // Hopefully we're presented with the bitmap without the header here?

    pub fn update_fb(&mut self) {
        if self.window.is_open() {
            //     let fb2: Vec<u32> = vec![0x00FFFFFF; WIDTH * HEIGHT];
            let mut buffer: Vec<u32> = self.font
                .bitmap
                .chunks(4)
                .map(|n| Cursor::new(n).read_u32::<LittleEndian>().unwrap())
                .collect();
            println!("Framebuffer: {:?}", buffer);

            self.window.update_with_buffer(&buffer);
        }
    }

    // Render a character
    pub fn render_char(&mut self) {

        let mut offset: u16 = 0;
        let mut counter = 0;
        let mut y: u16 = 256;
        let mut x: u16 = 0;
        // This is our line width, for now this is the same
        // as the raster width.
        let mut test_bitmap: Vec<u32> = vec![0x00FFFFFF; 63553];
        const LINE: usize = 256;

        // Our X offset in the bitmap array
        for offset in 0..(WIDTH.wrapping_mul(HEIGHT as usize)) {

            // TODO: Find out character height & width; or check bitmap generator.
            // We need to know this in order to render the correct area of the "sprite sheet"

            // Our Y line
            for y_line in 0..LINE {
                // for i in &test_bitmap {
                // We use wrapping here as our array is too small
                if test_bitmap[offset.wrapping_mul(WIDTH as usize).wrapping_add(y_line)] != 0 {
                    self.window.update_with_buffer(&test_bitmap);
                }

            }
        }
    }
}
