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

    // debugger.draw_text("Opcode:", 10, 20);
    // debugger.draw_text("Register A:", 10, 40);
    // debugger.draw_text("PC:", 10, 60);
    // debugger.draw_text("carry::", 10, 80);

    // load binary file
    memory.load_bin(bin);


    // TODO implement break & step keyboard actions
    loop {
        // CPU execution
        ExecutionContext::new(&mut memory, &mut registers).step(1);
        // debugger.draw_register("A", registers.reg_a);
        // debugger.draw_register("B", registers.reg_b);
        // debugger.draw_num(registers.reg_a, 130, 20);
        // debugger.draw_num(registers.opcode, 130, 40);
        // debugger.draw_num(registers.pc as u8, 130, 60);
        // debugger.draw_bool(registers.carry, 130, 80);
        // display.render_vram(&mut memory);

    }
}
