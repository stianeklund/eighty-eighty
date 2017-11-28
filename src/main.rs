#![feature(try_from)]

extern crate minifb;
extern crate byteorder;

use display::Display;
use std::thread::sleep_ms;
use std::time::{Instant, Duration};
use interconnect::Interconnect;
use keypad::{State, Input, Keypad};

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
        sleep_ms(60);
        // Poll for input
        display.draw_pixel(&i);
        display.window.update_with_buffer(&display.raster).unwrap();
        Keypad::poll_input(&mut i.registers, &display.window);
    }
}
