#[cfg(test)]
mod tests {
    use cpu::{Registers, ExecutionContext};
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

        // RET Set return address in memory
        // Per i8080core return should be 032F
        cpu.memory.memory[5] = 0xC9;
        // println!("Setting return address: [5]: {:#04X}", cpu.memory.memory[5]);

        // INIT
        // i8080core sets this for i8080_init().. Why?
        cpu.registers.pc = 0xF800;

        // TODO Look at what start addresses the other CPU tests need.
        // Commented out as we're using 8080PRE as our test
        // For CPUTEST Only
        cpu.registers.pc = 0x0100;
        // println!("Jumping to: {:#04X}", cpu.registers.pc);

        let mut success: bool = false;
        let mut instruction = cpu.memory.read(cpu.registers.pc as usize);
        for i in 0..10 {
            cpu.execute_instruction(instruction);
            // cpu.step(1);
           if cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", cpu.registers.pc);
                break
            }
            if cpu.registers.pc == 0x0005 {
                if cpu.registers.reg_c == 9 {
                    let addr: u16 = cpu.registers.pc;
                    // Create register pair
                    let reg_de = vec![cpu.memory.read_word(addr)];
                    for i in reg_de {
                        println!("{:?}", cpu.memory.memory[i as usize]);
                        success = true;
                    }
                }
                if cpu.registers.reg_c == 2 {
                    println!("{}", cpu.registers.reg_e);
                }
            }
            // 8080PRE Resets PC to 0 if there has been an error
            if cpu.registers.pc == 0 {
                println!("Jump to 0");
                assert_eq!(cpu.registers.pc, 1);
            }
        }
    }
}

