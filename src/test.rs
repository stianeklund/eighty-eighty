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

        // Set return address;
        cpu.memory.memory[5] = 0xC9;
        println!("Setting return address: [5]: {:#04X}", cpu.memory.memory[5]);
        // Jump to 0x100 (CPUTEST requires us to start here);
        cpu.registers.pc = 0x100;


        loop {
            cpu.step(1);
            if cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", cpu.registers.pc);
                break
            }
            // println!("PC: {:#04X}", cpu.registers.pc);
            if cpu.registers.pc == 0x0005 {
                if cpu.registers.reg_c == 9 {
                    let addr: u16 = cpu.registers.pc;
                    let reg_de = vec![cpu.memory.read_word(addr)];
                    for i in reg_de {
                        cpu.memory.memory[i as usize];
                    }
                }
            }
            return
        }
            // assert_eq!(cpu.registers.pc, 0x100);


    }
}

