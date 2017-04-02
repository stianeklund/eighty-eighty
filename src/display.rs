extern crate sdl2;

use super::sdl2::Sdl;
use super::sdl2::pixels::Color;
use super::sdl2::rect::Rect;
use interconnect;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;


pub struct Display {
    renderer: sdl2::render::Renderer<'static>,
    vram: Box<[u8; 8000]>,
    pixels: [[bool; WIDTH]; HEIGHT],
    vblank: bool,
    draw_flag: bool,
}

impl Display {
    pub fn new(sdl_ctx: &Sdl) -> Display{

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
            vram: Box::new([0; 8000]),
            pixels: [[false; WIDTH]; HEIGHT],
            vblank: false,
            draw_flag: true,
        }
    }


    pub fn draw(&mut self) {

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.pixels[y][x] {
                    // Foreground
                    self.renderer.set_draw_color(Color::RGB(251, 241, 199));
                } else {
                    // Background
                    self.renderer.set_draw_color(Color::RGB(69, 133, 149));
                }

                self.renderer.fill_rect(Rect::new(20, 20, 20, 20)).unwrap();

            }
            self.renderer.present();
            self.draw_flag = true;
        }
    }

    // TODO Rendering & draw pixels
    // Each byte of video ram represents 8 pixels.
    // In other words, each byte needs to be "decoded" to 8 pixels.
    // Then do XOR drawing for pixel 1 / 0.

}
