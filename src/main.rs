use std::env;

mod cpu;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("[Rom path]");
    }

    let bin = &args[1];

    let mut cpu = cpu::Cpu::new();

    cpu.load_bin(bin);

    loop {
        cpu.execute_instruction();
    }
}
