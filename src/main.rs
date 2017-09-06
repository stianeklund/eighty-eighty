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

#[derive(Debug, Clone, Copy)]
pub struct Debug {
    state: bool
}
impl Debug {
    pub fn new() -> Debug {
        Debug {
            state: false
        }
    }
}

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

    let mut debug = Debug::new();
    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        i.execute_cpu();
        display.draw_pixel(&i);
        // sleep_ms(5);
        display.window.update_with_buffer(&display.raster).unwrap();
        if display.window.is_key_down(Key::D) {
            debug.state = true;

        } else if display.window.is_key_down(Key::E) {
            debug.state = false;
        } else if display.window.is_key_down(Key::Escape) {
            break
        }
    }
}



