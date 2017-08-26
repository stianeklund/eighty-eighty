#[cfg(test)]
mod tests {
    use cpu::{Registers, ExecutionContext};
    use memory::Memory;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, Write};
    use std::path::Path;
    use std::thread::sleep;
    use std::thread::sleep_ms;
    use std::ascii::AsciiExt;

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

        let mut success: bool = false;

        'main: loop {
            cpu.step(1);

            if cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", cpu.registers.pc);
                #[should_panic]
                assert_ne!(cpu.registers.pc, 0x76);
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if cpu.registers.pc == 05 {
                if cpu.registers.reg_c == 9 {
                    // Create register pair
                    let mut de: u16 = (cpu.memory.memory[cpu.registers.reg_d as usize] as u16) << 8
                        | (cpu.memory.memory[cpu.registers.reg_e as usize] as u16) as u16;

                    'print: loop {
                        let mut output = cpu.memory.memory[de as usize];
                        output & 0x7F;

                        //output.escape_unicode()
                        if output as char == '$' {
                            break 'print
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if cpu.registers.reg_c == 2 {
                    print!("{}", cpu.registers.reg_e as char);
                }
            }
            sleep_ms(5);

            if cpu.registers.pc == 0 {
                let stack = (cpu.memory.memory[cpu.registers.sp as usize + 1] as u16) << 8 |
                cpu.memory.memory[cpu.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break
            }
            assert_ne!(cpu.registers.opcode, 0x00);
        }
    }

    #[test]
    fn hl_mem_test() {
        // Standup memory & registers
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        // 8080PRE's Test access to memory through HL
        let path = Path::new("hl_mem_access.bin");
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
        // cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        cpu.registers.pc = 0x0100;

        let mut success: bool = false;

        loop {
            cpu.step(1);
            // cpu.step(1);
            if cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", cpu.registers.pc);
                break;
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
            sleep_ms(50);
            // Last instruction is to halt emulator, lets panic here.
            if cpu.registers.opcode == 0x76 {
                break;
            }
        }
    }
}
