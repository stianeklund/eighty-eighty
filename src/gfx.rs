extern crate sdl2;

use sdl2::Sdl;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub const WIDTH: usize = 128;
pub const HEIGHT: usize = 64;


pub struct Display<'a> {
    pub renderer: sdl2::render::Renderer<'a>,
    pub pixels: [[bool; WIDTH]; HEIGHT],
    pub memory: Box<[u8; 65536]>,
    pub draw_flag: bool,
}

impl<'a> Display<'a> {
    pub fn new(sdl_ctx: &Sdl) -> Display<'a> {

        // Initialize SDL2
        let video = sdl_ctx.video().expect("SDL2 initialization failed");

        // Create window
        let window = video.window("Eighty Eighty", WIDTH as u32 * 10, HEIGHT as u32 * 10)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window.renderer()
            .accelerated()
            .build()
            .expect("Initialization of window renderer failed");

        Display {
            renderer: renderer,
            pixels: [[false; WIDTH]; HEIGHT],
            memory: Box::new([0; 65536]),
            draw_flag: true,
        }
    }

    pub fn render(&mut self, x: usize, y: usize) {

        let xi = x;
        let x = HEIGHT - y;
        let y = xi;
        let byte = self.memory[0x2400 + (y * (HEIGHT / 8) + (x / 8))];
        if byte & (1 << (x % 8)) > 0 & 0xFFFFFF {
            self.renderer.set_draw_color(Color::RGB(251, 241, 199));
        } else {
            self.renderer.set_draw_color(Color::RGB(69, 133, 149));
        }
        self.renderer.fill_rect(Rect::new(x as i32, y as i32, 10, 10)).expect("Fill rect failed");

        // for y in 0..HEIGHT {
          //  for x in 0..WIDTH {
           //     if [y][x] {
                    // Foreground
          //          self.renderer.set_draw_color(Color::RGB(251, 241, 199));
           //     } else {
                    // Background
           //         self.renderer.set_draw_color(Color::RGB(69, 133, 149));
            //    }
                // x, y, w, h
        self.renderer.present();
        self.draw_flag = true;
    }
}
