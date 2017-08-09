#[cfg(test)]
mod tests {
    use cpu::{Registers, ExecutionContext};
    use memory::Memory;

    #[test]
    fn cpu_test() {
        // Standup CPU, memory & registers
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // Load CPUTEST from root directory
        memory.load_bin("8080PRE.COM");

        let mut cpu = ExecutionContext::new(&mut memory, &mut registers);

        // RET Set return address in memory
        // Per i8080core return should be 032F
        cpu.memory.memory[5] = 0xC9;
        // println!("Setting return address: [5]: {:#04X}", cpu.memory.memory[5]);

        // i8080core sets this for i8080_init().. Why?
        // cpu.registers.pc = 0xF800;

        // TODO Look at what start addresses the other CPU tests need.
        // Commented out as we're using 8080PRE as our test
        // For CPUTEST Only
        // cpu.registers.pc = 0x100;
        // println!("Jumping to: {:#04X}", cpu.registers.pc);

        let mut success: bool = false;

        for i in 0..10 {
            cpu.step(1);
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

