extern crate minifb;
extern crate byteorder;

use debugger::{HEIGHT, WIDTH};
use cpu::{ExecutionContext, Registers};
use interconnect::Interconnect;
use minifb::Key;
use display::Display;
use std::thread::sleep;
use std::time::Duration;

mod cpu;
mod opcode;
mod display;
mod interconnect;
mod debugger;
mod memory;
mod keypad;
mod test;

const DEBUG: bool = false;

fn main() {
    let args: Vec<String> = std::env::args().collect();
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
