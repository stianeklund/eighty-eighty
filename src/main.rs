extern crate sdl2;
extern crate minifb;

use std::env;
use std::thread;
use minifb::Key;
mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;
mod keypad;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];
    let mut inter = interconnect::Interconnect::new();
    inter.cpu.load_bin(bin);
    // inter.display.draw();

    // TODO Implement break & step keyboard actions
    loop {
        inter.cpu.run();
        inter.display.render_vram();
        inter.display.update_screen();
        if inter.display.window.is_key_down(Key::Escape) || inter.display.window.is_key_down(Key::X) {
            break;
            } else {
            continue;
        }
        thread::sleep_ms(3);
    }
}
