extern crate minifb;

use std::env;
use std::thread;
use minifb::Key;
mod cpu;
mod opcode;
mod display;
mod debugger;
// mod interconnect;
mod memory;
mod keypad;

use cpu::{ExecutionContext, Registers};

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];
    // let mut inter = interconnect::Interconnect::new();
    let mut memory = memory::Memory::new();
    let mut registers = Registers::new();
    let mut display = display::Display::new();
    let mut debugger = debugger::Debugger::new();
    let mut bitmap = debugger.font.bitmap;

    memory.load_bin(bin);

    // TODO Implement break & step keyboard actions
    loop {
        ExecutionContext::new(&mut memory, &mut registers).step(1);
        display.render_vram(&mut memory);
        // thread::sleep_ms(3);

    }
}
