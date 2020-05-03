extern crate byteorder;
extern crate minifb;

use crate::interconnect::Interconnect;
use crate::keypad::Input;
use std::thread::sleep;
use std::time::{Duration, Instant};

mod cpu;
mod display;
mod interconnect;
mod keypad;
mod memory;
mod opcode;
mod tests;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("[Please specify ROM as an argument]");
        return;
    }

    let bin = &args[1];
    let i = &mut Interconnect::new();
    i.cpu.memory.load_bin(bin);

    loop {
        // TODO: Proper timing
        // Should be approx 1000 cycles per tick?
        let prev_tick = Instant::now();
        let tick = prev_tick.elapsed().as_millis();

        // For debugging (executing one instruction at a time)
        // std::io::stdin().read_line(&mut String::new()).unwrap();

        i.execute_cpu(); // <-- handles interrupts fos us. One execution == 1 frame
        i.keypad.key_down(&mut i.cpu.registers, &i.display.window);

        sleep(Duration::from_millis(16));

        i.display.draw_pixel(&i.cpu.memory);

        i.display
            .window
            .update_with_buffer(&i.display.raster)
            .unwrap();

        // Reset I/O port values every 5 frames
        if i.frame_count % 5 == 1 {
            // i.keypad.key_up(&mut i.cpu.registers,&i.display.window);
            i.keypad.reset_ports(&mut i.cpu.registers);
        }
    }
}
