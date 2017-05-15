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
    bitmap: Vec<u8>,
}

impl DebugFont {
    pub fn new() -> DebugFont {
        let mut font = DebugFont { bitmap: Vec::<u8>::new() };

        // TODO find out if we can use a bitmap file with characters for rendering?
        // If so do we need to handle BMP header data or can this be omitted?
        // let mut file = File::open(&path).expect("File not found");
        // let mut file_data = Vec::<u8>::new();
        // let result = file.read_to_end(&mut file_data);

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

        Debugger {
            buffer: vec![0; WIDTH * HEIGHT],
            font: DebugFont::new(),
            window: window,
            memory_page: vec![0; 65536],
        }
    }
}
