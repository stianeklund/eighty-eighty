extern crate minifb;
extern crate byteorder;
use interconnect::Interconnect;
use minifb::Key;
use display::Display;
use std::io;

mod cpu;
mod opcode;
mod display;
mod interconnect;
mod debugger;
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

    let mut i = Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);

    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        i.execute_cpu();
        display.draw_pixel(&i);
        display.window.update_with_buffer(&display.raster).unwrap();

        if display.window.is_key_down(Key::D) {
            i.registers.debug = true;
        } else if display.window.is_key_down(Key::B) {
            i.registers.breakpoint = true;
            i.registers.debug = true;
        } else if display.window.is_key_down(Key::E) {
            i.registers.debug = false;
            i.registers.breakpoint = false;
        } else if display.window.is_key_down(Key::Escape) {
            i.registers.breakpoint = false;
            i.registers.debug = false;
            break;
        }
    }
}
