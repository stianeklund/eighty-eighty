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
    use std::str;

    #[test]
    fn preliminary() {
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
        // If successful it should return to 0x0005.
        cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        cpu.registers.pc = 0x0100;

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
                    let mut de = (cpu.registers.reg_d as u16) << 8 | (cpu.registers.reg_e as u16);
                    let mut result = String::new();
                    'print: loop {
                        let output = cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print
                        } else if output as char != '$' {
                            de += 1;
                        }
                        result.push(output as char);
                    }
                    println!("{}", result)
                }
            }
            if cpu.registers.reg_c == 2 {
                print!("{}", cpu.registers.reg_e as char);
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
    fn instruction_set_exerciser() {
        // Standup memory & registers
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        let path = Path::new("8080EX1.COM");
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
        // If successful it should return to 0x0005.
        cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        cpu.registers.pc = 0x0100;

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
                    let mut de = (cpu.registers.reg_d as u16) << 8 | (cpu.registers.reg_e as u16);
                    let mut result = String::new();
                    'print: loop {
                        let output = cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print
                        } else if output as char != '$' {
                            de += 1;
                        }
                        result.push(output as char);
                    }
                    println!("{}", result)
                }
            }
            if cpu.registers.reg_c == 2 {
                print!("{}", cpu.registers.reg_e as char);
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
    fn cpu_test() {
        // Standup memory & registers
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        let path = Path::new("8080EX1.COM");
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
        // If successful it should return to 0x0005.
        cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        cpu.registers.pc = 0x0100;

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
                    let mut de = (cpu.registers.reg_d as u16) << 8 | (cpu.registers.reg_e as u16);
                    let mut result = String::new();
                    'print: loop {
                        let output = cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print
                        } else if output as char != '$' {
                            de += 1;
                        }
                        result.push(output as char);
                    }
                    println!("{}", result)
                }
            }
            if cpu.registers.reg_c == 2 {
                print!("{}", cpu.registers.reg_e as char);
            }
            sleep_ms(5);

            if cpu.registers.pc == 0 {
                let stack = (cpu.memory.memory[cpu.registers.sp as usize + 1] as u16) << 8 |
                    cpu.memory.memory[cpu.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break
            }
            if cpu.registers.opcode == 0x00 {
               panic!();
            }
        }
    }
}
