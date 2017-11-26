#![feature(try_from)]

extern crate minifb;
extern crate byteorder;
use interconnect::Interconnect;
use minifb::Key;
use display::Display;
use std::thread::sleep_ms;
use std::time::{Instant, Duration};
use keypad::{State, Input};

mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;
mod keypad;
mod test;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[Please specify ROM as an argument]");
        return;
    }

    let bin = &args[1];

    let mut now = Instant::now();
    let i = &mut Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);

    loop {
        // Execute an instruction
        i.execute_cpu();
        sleep_ms(16);
        // Iterate over VRAM & only VRAM and update the local raster
        display.draw_pixel(&i);
        // Present raster to window

        // TODO Better input handling...
        if display.window.is_key_down(Key::D) {
            i.registers.debug = true;
        } else if display.window.is_key_down(Key::E) {
            i.registers.debug = false;
        } else if display.window.is_key_down(Key::Escape) {
            Input::handle_input(&mut i.registers, Key::Enter);
        } else if display.window.is_key_down(Key::C) {
            Input::handle_input(&mut i.registers, Key::C);

        } else if display.window.is_key_down(Key::Enter) {
            Input::handle_input(&mut i.registers, Key::Enter);
        } else if display.window.is_key_down(Key::Space) {
            Input::handle_input(&mut i.registers, Key::Space);
        }
        display.window.update_with_buffer(&display.raster).unwrap();
    }
}
