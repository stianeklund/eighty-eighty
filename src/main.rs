#![feature(inclusive_range,inclusive_range_syntax)]

extern crate minifb;
extern crate byteorder;

use debugger::{HEIGHT, WIDTH};
use cpu::{ExecutionContext, Registers};
use minifb::Key;
use display::Display;
use std::env;
use std::thread::sleep_ms;
use interconnect::Interconnect;
mod cpu;
mod opcode;
mod display;
mod interconnect;
mod debugger;
mod memory;
mod keypad;
mod test;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Please specify ROM as an argument]");
        return;
    }


    let bin = &args[1];

    let mut i = Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);

    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        i.execute_cpu();
        display.draw_pixel(&i);
        // sleep_ms(5);
        display.window.update_with_buffer(&display.raster).unwrap();
    }
}


            // if interconnect.display.window.is_key_down(Key::Escape) {
            // break
            //}
