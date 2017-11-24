extern crate minifb;
extern crate byteorder;
use interconnect::Interconnect;
use minifb::Key;
use display::Display;
use std::thread::sleep_ms;
use std::time::{Instant, Duration};

mod cpu;
mod opcode;
mod display;
mod interconnect;
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

    let mut now = Instant::now();
    let i = &mut Interconnect::new();
    let mut display = Display::new();

    // load binary file
    i.memory.load_bin(bin);
    // let fps = Duration::new(0,1_660_000_000);
    // let ms = 16;

    loop {
        // Execute an instruction
        i.execute_cpu();
        // Iterate over VRAM & only VRAM and update the local raster
        sleep_ms(100);
        display.draw_pixel(&i);
        // Present raster to window
        display.window.update_with_buffer(&display.raster);


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
