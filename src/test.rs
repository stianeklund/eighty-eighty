#[cfg(test)]
mod tests {
    use cpu::{Registers, ExecutionContext};
    use interconnect::Interconnect;
    use memory::Memory;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::{self, Write};
    use std::path::Path;
    use std::thread::sleep;
    use std::thread::sleep_ms;
    use std::ascii::AsciiExt;

    #[test]
    fn preliminary() {
        // Standup memory & registers
        let mut i = Interconnect::new();

        // 8080PRE
        let path = Path::new("8080PRE.COM");
        let mut file = File::open(&path).expect("Couldn't load binary");
        let bin: &str = "8080PRE.COM";
        i.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.registers.pc = 0x0100;

        'main: loop {
            i.execute_cpu();

            if i.registers.pc == 0x76 {
                println!("HALT at {:#04X}", i.registers.pc);
                #[should_panic]
                assert_ne!(i.registers.pc, 0x76);
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.registers.pc == 05 {
                if i.registers.reg_c == 9 {
                    // Create register pair
                    let mut de = (i.registers.reg_d as u16) << 8 | (i.registers.reg_e as u16);
                    let mut result = String::new();
                    'print: loop {
                        let output = i.memory.memory[de as usize];
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
            if i.registers.reg_c == 2 {
                print!("{}", i.registers.reg_e as char);
            }
            sleep_ms(5);

            if i.registers.pc == 0 {
                let stack = (i.memory.memory[i.registers.sp as usize + 1] as u16) << 8 |
                    i.memory.memory[i.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break
            }
            assert_ne!(i.registers.opcode, 0x00);
        }
    }
    #[test]
    fn cpu_test() {
        // Standup memory & registers
        let mut i = Interconnect::new();

        // CPUTEST
        let path = Path::new("CPUTEST.COM");
        let mut file = File::open(&path).expect("Couldn't load binary");
        let bin: &str = "CPUTEST.COM";
        i.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.registers.pc = 0x0100;

        let mut result = String::new();
        'main: loop {
            i.execute_cpu();

            if i.registers.pc == 0x76 {
                println!("HALT at {:#04X}", i.registers.pc);
                #[should_panic]
                assert_ne!(i.registers.pc, 0x76);
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.registers.pc == 05 {
                if i.registers.reg_c == 9 {
                    // Create register pair
                    let mut de = (i.registers.reg_d as u16) << 8 | (i.registers.reg_e as u16);
                    'print: loop {
                        let output = i.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print
                        } else if output as char != '$' {
                            de += 1;
                        }
                        result.push(output as char);
                    }
                    print!("{}", result)
                }
            }
            if i.registers.reg_c == 2 {
                print!("{}", i.registers.reg_e as char);
            }
            sleep_ms(5);

            if i.registers.pc == 0 {
                let stack = (i.memory.memory[i.registers.sp as usize + 1] as u16) << 8 |
                    i.memory.memory[i.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break
            }
            assert_ne!(i.registers.pc, 0);
            assert_ne!(i.registers.opcode, 0x00);
        }
    }
}

