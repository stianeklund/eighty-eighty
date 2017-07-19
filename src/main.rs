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
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];

    // let mut inter = interconnect::Interconnect::new();
    let mut memory = memory::Memory::new();
    let mut registers = Registers::new();
    // let mut display = display::Display::new();
    let mut debugger = debugger::Debugger::new();

    // Load binary file
    memory.load_bin(bin);


    // TODO Implement break & step keyboard actions

    loop {
        // CPU Execution
        ExecutionContext::new(&mut memory, &mut registers).step(1);
        // display.render_vram(&mut memory);

        // Update the debug frame buffer
        // debugger.render_fb(30, 30);

        debugger.draw_sprite(10,40,'H');
        debugger.draw_sprite(20,40,'E');
        debugger.draw_sprite(30,40,'L');
        debugger.draw_sprite(40,40,'L');
        debugger.draw_sprite(50,40,'O');



    }
}
