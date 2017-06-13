use super::minifb::{Key, Scale, WindowOptions, Window};
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};


use std::io::prelude;
use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use display::Display;
use memory::Memory;

mod font;

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
        let path = Path::new("/home/stian/dev/projects/eighty-eighty/assets/font.tga");
        let mut file = File::open(&path).expect("File not found");
        let mut file_data = Vec::<u8>::new();

        // Skip BMP header & DIB for now.
        // file.seek(SeekFrom::Start(1)).expect("Seek error");
        let result = file.read_to_end(&mut file_data);


        match result {
            Ok(result) => println!("Read {:?}: Bitmap {} bytes", &path, result),
            Err(e) => panic!("IO Error:: {}", e),
        }
        // This may not be entirely correct, but for now lets just
        // read the bitmap data and assign it to the bitmap vec.
        font.bitmap = file_data;
        // println!("Targa bitmap: {:?}", font.bitmap);
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
    pub bitmap: font::Bitmap,
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

        Debugger {
            buffer: vec![0; WIDTH * HEIGHT],
            font: DebugFont::new(),
            bitmap: font::Bitmap::new(),
            window: window,
            fb: vec![0; 65536],
            memory_page: vec![0; 65536],
        }
    }

    // Cursor wraps another type & provides it with a Seek implementation
    // Chunks is a iterator that provides us with a slice
    //
    // Map provides a closure for this & creates an iterator
    // which calls that closure on each element.

    // We need a chunk size of 4 because 8 * 4 = 32 and we need a u32 to present

    // Create a temporary buffer & convert our bitmap values
    pub fn update_fb(&mut self) -> Vec<u32> {
            let mut buffer: Vec<u32> = self.font
                .bitmap
                .chunks(4)
                .map(|buf| {
                    let buf = Cursor::new(buf).read_u32::<LittleEndian>().unwrap();
                    buf
                })
                .collect::<Vec<u32>>();
                buffer
    }

    pub fn render_char(&mut self) {
        let sprite_sheet = self.update_fb();
        let mut frame_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let rect_width = 50;
        let rect_height = 30;

        let rect_x = 2;
        let rect_y = 3;

            for y in 0..rect_height {
                 for x in 0..rect_width {
                let frame_x = rect_x + x;
                let frame_y = rect_y + y;

                let buf_pos = frame_y * (WIDTH) + frame_x;
                frame_buffer[buf_pos] = 0x00FFFFFF;

            }
        }

        // Our X offset in the bitmap array
        /* for offset in 0..(WIDTH - 1) * (HEIGHT - 1) / 8) {
            // println!("Offset: {}", offset);

            // TODO: Find out character height & width; or check bitmap generator.
            // We need to know this in order to render the correct area of the "sprite sheet"
            // 8 Pixels per byte
            for y_line in 0..255 {
                // This panics, WHYYY?
                if self.font.bitmap[y * (WIDTH - 1) + x] != 0  {
                    frame_buffer[counter] = 0x00FFFFF;
                } else {
                    frame_buffer[counter] = 0;
                }

                y = y.wrapping_sub(1);
                if y <= 0 {
                    y = 255;
                }
                x = x.wrapping_add(1);
            }
                counter = counter + 1;
        }*/

        self.window.update_with_buffer(&frame_buffer);
    }
}
