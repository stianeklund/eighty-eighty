extern crate sdl2;
use std::env;


mod cpu;
mod opcode;
mod display;
mod interconnect;
mod memory;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];
    let mut cpu = cpu::Cpu::new();
    let mut inter = interconnect::Interconnect::new();
    cpu.load_bin(bin);


    loop {
        cpu.execute_instruction();
    }
}
