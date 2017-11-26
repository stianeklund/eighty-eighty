#![feature(try_from)]

extern crate minifb;
extern crate byteorder;
use interconnect::Interconnect;
use cpu::Registers;
use minifb::Key;
use display::Display;
use std::thread::sleep_ms;
use std::time::{Instant, Duration};
use keypad::{State, Input};

mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;
mod keypad;
mod test;

fn poll_input(registers: &mut Registers, window: &minifb::Window) {

        window.get_keys().map(|keys| {
        for t in keys {
            match t {
                Key::D     => registers.debug = true,
                Key::E     => registers.debug = false,
                Key::C     => Input::handle_input(registers, Key::C),
                Key::Enter => Input::handle_input(registers, Key::Enter),
                Key::Key2  => Input::handle_input(registers, Key::Key2),
                Key::Space => Input::handle_input(registers, Key::Space),
                Key::Left  => Input::handle_input(registers, Key::Left),
                Key::Right => Input::handle_input(registers, Key::Right),
                _ => eprintln!("Input key not handled"),
            }
        }

    });
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[Please specify ROM as an argument]");
        return;
    }

    let bin = &args[1];

    let mut now = Instant::now();
    let i = &mut Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);

    loop {
        // Execute an instruction
        i.execute_cpu();
        sleep_ms(16);
        // Poll for input
        poll_input(&mut i.registers, &display.window);
        display.draw_pixel(&i);
        display.window.update_with_buffer(&display.raster).unwrap();
    }
}
