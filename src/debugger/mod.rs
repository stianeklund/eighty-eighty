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
use std::thread;

mod font;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 256;

pub struct DebugFont {
    pub bitmap: Vec<u8>,
}

impl DebugFont {
    pub fn new() -> DebugFont {
        let mut font = DebugFont { bitmap: Vec::<u8>::new() };

        // TODO: Improve path handling.
        let path = Path::new("/home/stian/dev/projects/eighty-eighty/assets/ExportedFont.tga");
        let mut file = File::open(&path).expect("File not found");

        // This is the exact number of bytes of image data we want to read
        // By doing this we exclude the pesky footer data.
        // The header is 18 bytes long, and therefor we skip it entirely.
        let mut file_data = vec![0; WIDTH * HEIGHT * 3];
        file.seek(SeekFrom::Start(18)).expect("IO Seek error");
        let result = file.read_exact(&mut file_data);

        match result {
            Ok(result) => println!("Read {:?}: Bitmap {:?} bytes", &path, result),
            Err(e) => panic!("IO Error:: {}", e),
        }
        // This may not be entirely correct, but for now lets just
        // read the bitmap data and assign it to the bitmap vec.
        font.bitmap = file_data;
        font
    }
}

// TODO: Implement a way to display Cpu register values & memory pages.
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
        let mut window =
            Window::new("Debugger",
                        WIDTH,
                        HEIGHT,
                        WindowOptions { scale: Scale::X2, ..WindowOptions::default() })
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
    // which calls that closure on each element.
    // We need a chunk size of 3 because 8 * 3 = 24 and we need a 24bit integer to present
    // Create a temporary buffer & convert our bitmap values
    pub fn create_fb(&mut self) -> Vec<u32> {
        let mut buffer: Vec<u32> = self.font
            .bitmap
            .chunks(3)
            .map(|buf| {
                let buf = Cursor::new(buf)
                    .read_u24::<LittleEndian>()
                    .unwrap();
                buf
            })
            .collect();
        buffer
    }

    pub fn render_fb(&mut self) {
        let mut sprite_sheet = self.create_fb();
        let mut frame_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let image_y = 255 - y;
                let offset = (WIDTH * image_y) + x;
                let frame_offset = (WIDTH * y) + x;
                frame_buffer[frame_offset] = sprite_sheet[offset];
            }
        }
    }

    pub fn draw_sprite(&mut self, x: usize, y: usize, character: char) {
        let mut sprite_sheet = self.create_fb();
        let mut frame_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        // Lookup charcter in `lookup_char()` and return an integer value.
        let sprite_value = self.lookup_char(character);

        let sprite_w = 32;
        let sprite_h = 32;

        let index_x = sprite_w * (sprite_value % 32);
        let index_y = sprite_h * (sprite_value / 32);
        let tile_w = index_x + sprite_w;
        let tile_h = index_y + sprite_h;

        let mut offset = 0;
        let mut line = 0;

        for i in index_y..tile_h {
            for j in index_x..tile_w {
                // Subtract 255 from the y index to flip the coordinates.
                frame_buffer[x + line + WIDTH * (y + offset)] = sprite_sheet[j +
                                                                             ((255 - i) * HEIGHT)];
                line += 1;
            }
            line = 0;
            offset += 1;
            self.window.update_with_buffer(&frame_buffer);
        }
    }

    pub fn draw_text(&mut self, text: &str) {
        let collection: Vec<char> = text.chars().collect();

        let mut x = 10;
        for ch in text.chars() {
            thread::sleep_ms(2);
            self.draw_sprite(x, 10, ch);
            x += 10;
        }
    }
    // Looks up charcter from char & provides us with a corresponding value
    fn lookup_char(&self, character: char) -> usize {
        match character {
            '!' => 0,
            '"' => 1,
            '#' => 2,
            '$' => 3,
            '%' => 4,
            '&' => 5,
            '\'' => 6,
            '(' => 7,
            ')' => 40,
            '*' => 41,
            '+' => 42,
            ',' => 43,
            '-' => 44,
            '.' => 45,
            '/' => 46,
            '0' => 47,
            '1' => 64,
            '2' => 65,
            '3' => 66,
            '4' => 67,
            '5' => 68,
            '6' => 69,
            '7' => 70,
            '8' => 71,
            '9' => 96,
            ':' => 97,
            ';' => 98,
            '<' => 99,
            '=' => 100,
            '>' => 101,
            '?' => 102,
            '@' => 103,
            'A' => 128,
            'B' => 129,
            'C' => 130,
            'D' => 131,
            'E' => 132,
            'F' => 133,
            'G' => 134,
            'H' => 135,
            'I' => 160,
            'J' => 161,
            'K' => 162,
            'L' => 163,
            'M' => 164,
            'N' => 165,
            'O' => 166,
            'P' => 167,
            'Q' => 192,
            'R' => 193,
            'S' => 194,
            'T' => 195,
            'U' => 196,
            'V' => 197,
            'W' => 198,
            'X' => 199,
            'Y' => 224,
            'Z' => 225,
            '[' => 226,
            '\\' => 227,
            ']' => 228,
            '^' => 229,
            '_' => 230,
            '`' => 231,
            _ => 250,
        }
    }
}
