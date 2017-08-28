#![feature(inclusive_range,inclusive_range_syntax)]

extern crate minifb;
extern crate byteorder;

use debugger::{HEIGHT, WIDTH};
use cpu::{ExecutionContext, Registers};
use minifb::Key;
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

    let mut interconnect = Interconnect::new();

    // load binary file
    interconnect.memory.load_bin(bin);

    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        ExecutionContext::new(&mut interconnect.memory, &mut interconnect.registers).step(1);

        interconnect.display.render_vram(&mut interconnect.memory);
        // display.window.update_with_buffer(&display.raster).unwrap();

        //if display.window.is_key_down(Key::Escape) {
         //   break
        //}
        sleep_ms(1);


    }
}
