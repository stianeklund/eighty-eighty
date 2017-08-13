#[cfg(test)]
mod tests {
    use cpu::{Registers, ExecutionContext};
    use opcode::Instruction;
    use memory::Memory;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn cpu_test() {
        // Standup memory & registers
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // 8080PRE
        let path = Path::new("8080PRE.COM");
        let mut file = File::open(&path).expect("Couldn't load binary");
        let mut buf = Vec::new();


        file.read_to_end(&mut buf).expect("Failed to read binary");
        let buf_len = buf.len();
        for i in 0..buf_len {
            memory.memory[i + 0x0100] = buf[i];
        }
        println!("Loaded: {:?} Bytes: {:?}", path, buf_len);


        let mut cpu = ExecutionContext::new(&mut memory, &mut registers);


        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // Per i8080core return should be 032F at the end of the test
        cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        cpu.registers.pc = 0x0100;
        // println!("Jumping to: {:#04X}", cpu.registers.pc);

        let mut success: bool = false;
        let instruction = cpu.memory.read(cpu.registers.pc as usize);
        // for _ in 0..70 {
        loop {
            cpu.step(1);
            // cpu.step(1);
            if cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", cpu.registers.pc);
                break
            }
            if cpu.registers.pc == 0x0005 {
                if cpu.registers.reg_c == 9 {
                    let addr: u16 = cpu.registers.pc;
                    // Create register pair
                    let reg_de = vec![cpu.memory.read_word(addr as u8)];
                    for i in reg_de {
                        println!("{:?}", cpu.memory.memory[i as usize]);
                        success = true;
                    }
                }
                if cpu.registers.reg_c == 2 {
                    println!("{}", cpu.registers.reg_e);
                }
            }
        }
    }
}
