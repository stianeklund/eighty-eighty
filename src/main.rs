extern crate byteorder;
extern crate minifb;

use crate::interconnect::Interconnect;
use crate::keypad::Input;
use std::thread::sleep;
use std::time::Duration;

mod cpu;
mod display;
mod interconnect;
mod keypad;
mod memory;
mod opcode;
mod tests;

fn main() {
    let i = &mut Interconnect::new();
    let args: Vec<String> = std::env::args().collect();
    let mut display = display::Display::new();
    i.cpu.memory.load_bin(args);

    loop {
        // For debugging (executing one instruction at a time)
        // std::io::stdin().read_line(&mut String::new()).unwrap();
        i.execute_cpu(); // <-- handles interrupts fos us. One execution == 1 frame
        i.keypad.key_down(&mut i.cpu.registers, &display.window);

        display.draw_pixel(&i.cpu.memory);

        display
            .window
            .update_with_buffer(&display.raster)
            .unwrap();

        // Reset I/O port values every 5 frames
        // TODO: Implement better timing
        sleep(Duration::from_micros(16));
        if i.frame_count % 5 == 1 {
            // i.keypad.key_up(&mut i.cpu.registers,&i.display.window);
            i.keypad.reset_ports(&mut i.cpu.registers);
        }
    }
}
