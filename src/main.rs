extern crate sdl2;
use std::env;

mod cpu;
mod opcode;
mod gfx;
use gfx::Display;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
        return;
    }

    let bin = &args[1];

    let mut cpu = cpu::Cpu::new();

    let sdl_ctx = sdl2::init().expect("sdl2 init failed");
    let timer = sdl_ctx.timer().expect("timer failed");

    cpu.load_bin(bin);
    // let mut display = Display::new(&sdl_ctx);

    loop {
        cpu.execute_instruction();
    }
}
