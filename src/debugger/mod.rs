use super::minifb::{Key, Scale, WindowOptions, Window};
use byteorder::{ByteOrder, LittleEndian, BigEndian, ReadBytesExt};

use std::char;
use std::io::prelude;
use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use display::Display;
use cpu::{ExecutionContext, Registers};
use memory::Memory;
use std::thread;

mod font;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 256;
#[derive(Debug, Copy, Clone)]
pub enum DebugType {
    Bool(bool),
    U8(u8),
    U16(u16),
}

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
            fb: vec![0; WIDTH * HEIGHT],
            memory_page: vec![0; 65536],
        }
    }

    // Cursor wraps another type & provides it with a Seek implementation
    // which calls that closure on each element.
    // We need a chunk size of 3 because 8 * 3 = 24 and we need a 24bit integer to present
    // Create a temporary buffer & convert our bitmap values to be presented
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

    pub fn draw_register_text(&mut self) {
        self.draw_text("Opcode:", 10, 20);
        self.draw_text("Register A:", 10, 40);
        self.draw_text("PC:", 10, 60);
        self.draw_text("Carry:", 10, 80);
    }
    pub fn render_reg_values(&mut self, registers: Registers) {
        self.draw_num(registers.opcode, 130, 20);
        self.draw_num(registers.reg_a, 130, 40);
        self.draw_num(registers.pc as u8, 130, 60);
        self.draw_bool(registers.carry, 130, 80);
    }
        pub fn render_fb(&mut self) {
        let mut sprite_sheet = self.create_fb();
        // let mut frame_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let image_y = 255 - y;
                let offset = (WIDTH * image_y) + x;
                let frame_offset = (WIDTH * y) + x;
                self.fb[frame_offset] = sprite_sheet[offset];
            }
        }
    }

    // Takes a `char` type as input along with x & y positions.
    pub fn draw_sprite(&mut self, x: usize, y: usize, character: char) {
        let mut sprite_sheet = self.create_fb();
        // let mut frame_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        // Perform a lookup of the character in `lookup_char()`.
        // Returns a integer value to be used with the frame buffer.
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
                self.fb[x + line + WIDTH * (y + offset)] = sprite_sheet[j + ((255 - i) * HEIGHT)];
                line += 1;
            }
            line = 0;
            offset += 1;
            self.window.update_with_buffer(&self.fb);
        }
    }

    pub fn draw_bool(&mut self, value: bool, mut x: usize, mut y: usize) {

        let value = format!("{}", value);
        self.draw_text(&value, x, y);
    }
    pub fn draw_num(&mut self, num: u8, mut x: usize, mut y: usize) {

        let value = format!("{:04X}", num);
        self.draw_text(&value, x, y);
    }

    pub fn draw_text(&mut self, text: &str, mut x: usize, mut y: usize) {

        for ch in text.to_uppercase().chars() {
            self.draw_sprite(x, y, ch);
            // TODO Look into X & Y padding values
            x += 10;
        }
    }
    // Looks up charcter from char & provides us with a corresponding value
    fn lookup_char(&self, character: char) -> usize {
        match character {
            ' ' => 0,
            '!' => 1,
            '"' => 2,
            '#' => 3,
            '$' => 4,
            '%' => 5,
            '&' => 6,
            '\'' => 7,
            '(' => 40,
            ')' => 41,
            '*' => 42,
            '+' => 43,
            ',' => 44,
            '-' => 45,
            '.' => 46,
            '/' => 47,
            '0' => 64,
            '1' => 65,
            '2' => 66,
            '3' => 67,
            '4' => 68,
            '5' => 69,
            '6' => 70,
            '7' => 71,
            '8' => 96,
            '9' => 97,
            ':' => 98,
            ';' => 99,
            '<' => 100,
            '=' => 101,
            '>' => 102,
            '?' => 103,
            '@' => 128,
            'A' => 129,
            'B' => 130,
            'C' => 131,
            'D' => 132,
            'E' => 133,
            'F' => 134,
            'G' => 135,
            'H' => 160,
            'I' => 161,
            'J' => 162,
            'K' => 163,
            'L' => 164,
            'M' => 165,
            'N' => 166,
            'O' => 167,
            'P' => 192,
            'Q' => 193,
            'R' => 194,
            'S' => 195,
            'T' => 196,
            'U' => 197,
            'V' => 198,
            'W' => 199,
            'X' => 224,
            'Y' => 225,
            'Z' => 226,
            '[' => 227,
            '\\' => 228,
            ']' => 229,
            '^' => 230,
            '_' => 231,
            _ => 0,
        }
    }
}
