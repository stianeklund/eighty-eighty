#[cfg(test)]
mod tests {
    use interconnect::Interconnect;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn preliminary() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let duration = Duration::new(0, 2000);

        // 8080PRE
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

        i.registers.debug = false;
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
                    let mut de = (i.registers.reg_d as u16) << 8 | (i.registers.reg_e as u16);
                    'print: loop {
                        let output = i.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.registers.reg_c == 2 {
                    print!("{}", i.registers.reg_e as char);
                }
            }

            if i.registers.pc == 0 {
                let stack = (i.memory.memory[i.registers.sp as usize + 1] as u16) << 8 | i.memory.memory[i.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break;
            }
            // sleep(duration);
            assert_ne!(i.registers.opcode, 0x00);
        }
    }

    #[test]
    fn cpu_test() {
        let mut i = Interconnect::new();

        let duration = Duration::new(0, 10);
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

        // i.registers.debug = true;
        let mut cycles = 0;
        let hl = (i.registers.reg_h as u16) << 8 | (i.registers.reg_l as u16);

        'main: loop {
            // i.step_cpu();
            i.execute_cpu();

            if i.registers.pc == 0x76 {
                assert_ne!(i.registers.pc, 0x76);
            }

            /* if i.registers.prev_pc == 0x3589 && i.registers.reg_a == 0x02 && i.registers.reg_c == 0x05 {
                println!("B register: {:02X}, Mem location value: {:04X}", i.registers.reg_b, i.memory.memory[hl as usize]);
                panic!();
            }*/

            /* if i.registers.reg_h == 0xFF && i.registers.reg_l == 0xFF {
                i.registers.debug = true;
                if i.registers.reg_c == 19 {
                    break;
                }
            }*/

            // If PC is 5 we're at the return address we set earlier.
            if i.registers.pc == 05 {
                if i.registers.reg_c == 9 {
                    let mut de = (i.registers.reg_d as u16) << 8 | (i.registers.reg_e as u16);
                    'print: loop {
                        let output = i.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.registers.reg_c == 2 {
                    print!("{}", i.registers.reg_e as char);
                }
            }

            if i.registers.pc == 0 {
                let stack = (i.memory.memory[i.registers.sp as usize + 1] as u16) << 8 |
                    i.memory.memory[i.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}", stack);
                break;
            }
            // sleep(duration);
        }
    }
    #[test]
    fn cpu_ex1() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let duration = Duration::new(0, 0);
        let bin: &str = "TEST.COM";
        i.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.registers.pc = 0x0100;
        i.registers.debug = true;


        'main: loop {
            i.execute_cpu();

            if i.registers.pc == 0x76 {
                panic!("Halting");

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
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.registers.reg_c == 2 {
                    print!("{}", i.registers.reg_e as char);
                }
            }
            // sleep(duration);
            if i.registers.pc == 0 {
                let sp = i.memory.read_imm(i.registers.sp);
                i.registers.sp += 2;
                let stack = (i.memory.memory[i.registers.sp as usize + 1] as u16) << 8 |
                    i.memory.memory[i.registers.sp as usize] as u16;
                println!("\nJump to 0 from {:04X}, {:04X}", stack, sp);
                break;
            }
        }
    }
}
