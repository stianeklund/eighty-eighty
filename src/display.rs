extern crate sdl2;

use super::sdl2::Sdl;
use super::sdl2::pixels::Color;
use super::sdl2::rect::Rect;
use interconnect::Interconnect;

pub const WIDTH: usize = 256;
pub const HEIGHT: usize = 224;


pub struct Display {
    renderer: sdl2::render::Renderer<'static>,
    vram: Box<[u8; 65536]>,
    pixels: [[bool; WIDTH]; HEIGHT],
    vblank: bool,
    draw_flag: bool,
}

impl Display {
    pub fn new(ctx: &sdl2::Sdl) -> Display {

        // Initialize SDL2
        let video = ctx.video().unwrap();

        // Create window
        let window = video.window("Eighty Eighty", WIDTH as u32, HEIGHT as u32)
            .position_centered()
            .build()
            .expect("Window creation failed");
        let renderer = window.renderer()
            .accelerated()
            .build()
            .expect("Initialization of window renderer failed");

        Display {
            renderer: renderer,
            vram: Box::new([0; 65536]),
            pixels: [[true; WIDTH]; HEIGHT],
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

                self.renderer.fill_rect(Rect::new(10, 10, 10, 10)).unwrap();

            }
            self.renderer.present();
            self.draw_flag = true;
        }
    }
}

    /* pub fn render_vram(&mut self) {

        let mut base: u8 = 0x2400;
        let mut base_addr = base.wrapping_add(base);

        for j in 0..HEIGHT {
            let src = 0x2400 + (j << 5);
            for i in 0..32 {
                if self.vram[i] & 1 << j != 0 {

                    self.vram[src] = 0xFFFF;
                } else {
                    self.vram[src] = 0x0000;
                }
            }
        }
        // base as u8
    }*/
    // TODO Rendering & draw pixels
    // Each byte of video ram represents 8 pixels.
    // In other words, each byte needs to be "decoded" to 8 pixels.
    // Then do XOR drawing for pixel 1 / 0.
    // When the last line is drawn on the screen, we should generate
    // a Vertical Blank Interrupt
    // Ref: http://www.emulator101.com/displays-and-refresh.html
    // http://computerarcheology.com/Arcade/SpaceInvaders/Hardware.html


