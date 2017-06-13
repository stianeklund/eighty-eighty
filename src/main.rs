extern crate sdl2;

use std::env;
use std::thread;
mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;
mod keypad;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];
    let mut inter = interconnect::Interconnect::new();
    inter.cpu.load_bin(bin);
    inter.display.draw();
    let mut keypad = inter.keypad;

    // TODO Implement break & step keyboard actions
    loop {
        match keypad.key_press() {
            keypad::State::Exit => break,
            keypad::State::Step => {
                // cpu.step(1);
            },
            keypad::State::Break => {
                // TODO We want to pause here.
            },
            keypad::State::Continue => {}
        }
        inter.cpu.run();
        inter.display.render_vram();
        // thread::sleep_ms(3);
    }
}
