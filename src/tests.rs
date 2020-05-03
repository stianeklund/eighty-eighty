#[cfg(test)]
mod tests {
    use crate::interconnect::Interconnect;
    use std::time::Duration;

    #[test]
    fn preliminary() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let _duration = Duration::new(0, 2000);

        // 8080PRE
        let bin: &str = "tests/8080PRE.COM";
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.cpu.registers.pc = 0x0100;

        i.cpu.registers.debug = false;
        'main: loop {
            i.run_tests();
            if i.cpu.registers.pc == 0x76 {
                println!("HALT at {:#04X}", i.cpu.registers.pc);
                #[should_panic]
                assert_ne!(i.cpu.registers.pc, 0x76);
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.cpu.registers.pc == 05 {
                if i.cpu.registers.reg_c == 9 {
                    let mut de =
                        (i.cpu.registers.reg_d as u16) << 8 | (i.cpu.registers.reg_e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.registers.reg_c == 2 {
                    print!("{}", i.cpu.registers.reg_e as char);
                }
            }

            if i.cpu.registers.pc == 0 {
                println!("\nJump to 0 from {:04X}", i.cpu.registers.prev_pc);
                break;
            }
            // sleep(duration);
            // assert_ne!(i.cpu.registers.opcode, 0x00);
        }
    }

    #[test]
    fn test_com() {
        let mut i = Interconnect::new();

        let bin: &str = "tests/TEST.COM";
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5h
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.cpu.registers.pc = 0x0100;

        i.cpu.registers.debug = false;
        let _cycles = 0;

        'main: loop {
            // i.step_cpu();
            i.run_tests();

            if i.cpu.registers.pc == 0x76 {
                assert_ne!(i.cpu.registers.pc, 0x76);
            }
            if i.cpu.registers.opcode == 0x0000 {
                eprintln!("NOP?");
                // panic!();
            }

            // If PC is 5 we're at the return address we set earlier.
            if i.cpu.registers.pc == 05 {
                if i.cpu.registers.reg_c == 9 {
                    let mut de =
                        (i.cpu.registers.reg_d as u16) << 8 | (i.cpu.registers.reg_e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.registers.reg_c == 2 {
                    print!("{}", i.cpu.registers.reg_e as char);
                }
            }

            if i.cpu.registers.pc == 0 {
                println!("\nJump to 0 from {:04X}", i.cpu.registers.prev_pc);
                break;
            }
        }
    }
    #[test]
    fn cpu_test() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let bin: &str = "tests/CPUTEST.COM";
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.cpu.registers.pc = 0x0100;
        i.cpu.registers.debug = false;

        'main: loop {
            i.run_tests();

            if i.cpu.registers.pc == 0x76 {
                panic!("Halting");
            }

            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.cpu.registers.pc == 05 {
                if i.cpu.registers.reg_c == 9 {
                    // Create register pair
                    let mut de =
                        (i.cpu.registers.reg_d as u16) << 8 | (i.cpu.registers.reg_e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.registers.reg_c == 2 {
                    print!("{}", i.cpu.registers.reg_e as char);
                }
            }
            if i.cpu.registers.pc == 0 {
                println!("\nJump to 0 from {:04X}", i.cpu.registers.prev_pc);
                break;
            }
        }
    }

    #[test]
    #[ignore]
    fn cpu_exer() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let bin: &str = "tests/8080EXM.COM";
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.cpu.registers.pc = 0x0100;
        i.cpu.registers.debug = false;

        'main: loop {
            i.run_tests();

            if i.cpu.registers.pc == 0x76 {
                panic!("Halting");
            }
            if i.cpu.registers.opcode == 0x0000 {
                // panic!();
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.cpu.registers.pc == 05 {
                if i.cpu.registers.reg_c == 9 {
                    // Create register pair
                    let mut de =
                        (i.cpu.registers.reg_d as u16) << 8 | (i.cpu.registers.reg_e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.registers.reg_c == 2 {
                    print!("{}", i.cpu.registers.reg_e as char);
                }
            }
            if i.cpu.registers.pc == 0 {
                println!("\nJump to 0 from {:04X}", i.cpu.registers.prev_pc);
                break;
            }
        }
    }
    #[test]
    #[ignore]
    fn cpu_ex1() {
        // Standup memory & registers
        let mut i = Interconnect::new();
        let bin: &str = "tests/8080EX1.COM";
        i.cpu.memory.load_tests(bin);

        // Inject RET (0xC9) at 0x0005 to handle CALL 5
        // CALL 5 is the last subroutine call in the test.
        // If successful it should return to 0x0005.
        i.cpu.memory.memory[5] = 0xC9;

        // i8080core sets this before init, not sure why.
        i.cpu.registers.pc = 0xF800;

        // All test binaries start at 0x0100.
        i.cpu.registers.pc = 0x0100;
        i.cpu.registers.debug = false;

        'main: loop {
            i.run_tests();

            if i.cpu.registers.pc == 0x76 {
                panic!("Halting");
            }
            if i.cpu.registers.opcode == 0x0000 {
                // panic!();
            }
            // If PC is 5 we're at the return address we set earlier.
            // Print out characters from rom
            if i.cpu.registers.pc == 05 {
                if i.cpu.registers.reg_c == 9 {
                    // Create register pair
                    let mut de =
                        (i.cpu.registers.reg_d as u16) << 8 | (i.cpu.registers.reg_e as u16);
                    'print: loop {
                        let output = i.cpu.memory.memory[de as usize];
                        if output as char == '$' {
                            break 'print;
                        } else if output as char != '$' {
                            de += 1;
                        }
                        print!("{}", output as char);
                    }
                }
                if i.cpu.registers.reg_c == 2 {
                    print!("{}", i.cpu.registers.reg_e as char);
                }
            }
            if i.cpu.registers.pc == 0 {
                println!("\nJump to 0 from {:04X}", i.cpu.registers.prev_pc);
                break;
            }
        }
    }
}
