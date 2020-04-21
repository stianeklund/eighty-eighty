extern crate minifb;
extern crate byteorder;

use crate::display::Display;
use std::thread::sleep;
use std::time::{Instant, Duration};
use crate::interconnect::Interconnect;
use crate::keypad::{Keypad};

mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;
mod keypad;
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[Please specify ROM as an argument]");
        return;
    }

    let bin = &args[1];

    let _now = Instant::now();
    let i = &mut Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);

    loop {
        // Execute an instruction
        i.execute_cpu();
        sleep(Duration::from_micros(16));
        display.draw_pixel(&i);
        // Poll for input
        Keypad::poll_input(&mut i.registers, &display.window);
        display.window.update_with_buffer(&display.raster).unwrap();
    }
}
