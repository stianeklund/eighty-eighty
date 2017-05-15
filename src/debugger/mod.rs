use super::minifb::{Key, Scale, WindowOptions, Window};

use std::io::prelude;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use memory::Memory;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;

#[derive(Debug, PartialEq)]
pub struct DebugFont {
    pub bitmap: Vec<u8>,
}

impl DebugFont {
    pub fn new() -> DebugFont {
        let mut font = DebugFont { bitmap: Vec::<u8>::new() };

        // TODO: Figure out how many bytes we need to skip to omit the header
        // on the bitmap asset.

        // The block of bytes at the start of the file is the header.
        // The first 2 bytes of the BMP file format are the character "B"
        // then the character "M" in ASCII encoding.
        // All of the integer values are stored in little-endian (LSB first)

        // In our case we can likely omit the header all together.
        // Alternatively, we can check whether or not there is a match for
        // either "B" or "M" (in ASCII) to validate the file then skip the rest.

        // TODO: Improve path handling.
        let mut path = Path::new("/home/stian/dev/projects/eighty-eighty/assets/font.bmp");
        let mut file = File::open(&path).expect("File not found");
        let mut file_data = Vec::<u8>::new();
        let result = file.read_to_end(&mut file_data);

        // This may not be entirely correct, but for now lets just
        // read the bitmap data and assign it to the bitmap vec.
        font.bitmap = file_data;
        font
    }


    // The char type represents a single character.
    pub fn render_text(&mut self, text: &char) {

        // let s = String::from("love: ❤️");
        // Turn str type into a Vec the char primitive type & create a iterator.
        let mut chars: Vec<char> = vec![*text];

        // let chars = chars.into_iter();

        for i in 0..chars.len() {
            self.render_char(10, 10, &chars[i]);
        }
    }

    // Render a character
    pub fn render_char(&mut self, x: usize, y: usize, text: &char) {
        self.render_text(text);

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
                                     }).unwrap();

        window.set_position(1250, 340);

        Debugger {
            buffer: vec![0; WIDTH * HEIGHT],
            font: DebugFont::new(),
            window: window,
            memory_page: vec![0; 65536],
        }
    }
}
