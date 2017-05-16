use super::minifb::{Key, Scale, WindowOptions, Window};

use std::io::prelude;
use std::io::Read;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use display::Display;
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
        let path = Path::new("/home/stian/dev/projects/eighty-eighty/assets/font.bmp");
        let mut file = File::open(&path).expect("File not found");
        let mut file_data = Vec::<u8>::new();

        // Skip BMP header & DIB for now.
        let file_offset = file.seek(SeekFrom::Start(54));
        let result = file.read_to_end(&mut file_data);


        match result {
            Ok(result) => println!("Read {:?}: {} bitmap bytes", &path, result),
            Err(e) => panic!("IO Error:: {}", e),
        }
        // This may not be entirely correct, but for now lets just
        // read the bitmap data and assign it to the bitmap vec.
        font.bitmap = file_data;
        // println!("Bitmap: {:?}", font.bitmap);
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

        let memory = Memory::new();
        let display = Display::new();
        let mut raster = display.raster;

        // Sprite sheet info (based off the initial PNG, this may be wrong)

        // info face="Arial" size=32 bold=0 italic=0 charset=""
        // unicode=1 stretchH=100 smooth=1 aa=1 padding=0,0,0,0 spacing=1,1 outline=0
        // common lineHeight=32 base=26 scaleW=256 scaleH=256
        // pages=1 packed=0 alphaChnl=1 redChnl=0 greenChnl=0 blueChnl=0
        // page id=0 file="font_bitmap_0.png"

        // Example from sprite sheet file:
        // char id=43 x=187 y=84 width=14 height=12 xoffset=1 yoffset=10 xadvance=16 page=0 chnl=15

        let mut base: u16 = 0x2400;
        let mut offset: u16 = 0;
        let mut counter = 0;
        let mut y: u16 = 256;
        let mut x: u16 = 0;

        for offset in 0..(256 * 256) {

            // TODO: Find out character height & width; or check bitmap generator.
            // We need to know this in order to render the correct area of the "sprite sheet"

            for shift in 0..32 {
                // Lets assume 32 in shift width for now
                if (memory.memory[base as usize + offset as usize] >> shift) & 1 != 0 {
                    raster[counter as usize] = 0x0000000;
                } else {
                    raster[counter as usize] = 0x0FFFFFF;
                }
                y = y.wrapping_sub(1);
                if y < 0 {
                    y = 255;
                }
                x = x.wrapping_add(1);
            }
        }
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
