extern crate minifb;
extern crate byteorder;

use std::io::prelude;
use std::io::Read;
use std::io::Cursor;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::cmp::PartialEq;

use std::env;
use std::thread;
use minifb::{Window, WindowOptions, Scale, Key};
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use debugger::{HEIGHT, WIDTH};

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
        println!("[Please specify ROM as an argument]");
        return;
    }

    let bin = &args[1];

    // let mut inter = interconnect::Interconnect::new();
    let mut memory = memory::Memory::new();
    let mut registers = Registers::new();
    let mut display = display::Display::new();
    let mut debugger = debugger::Debugger::new();

    // load binary file
    memory.load_bin(bin);

    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        ExecutionContext::new(&mut memory, &mut registers).step(1);
        // Update registry values continuously
        debugger.draw_cpu_status(registers);
        debugger.draw_cpu_flags(registers);

        // display.draw(80, 80, &mut memory);
        display.render_vram(&mut memory);
        // Update window with our frame buffer here instead of within the rendering function
        //display.window.update_with_buffer(&display.raster);
        debugger.window.update_with_buffer(&debugger.fb);

    }
}
