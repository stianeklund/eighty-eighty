use opcode::{Instruction, Register, RegisterPair};
use memory::Memory;
use interconnect::Interconnect;

/// Intel 8080 Notes:
///
/// The Intel 8080 has 7 8-bit registers (A,B,C,D,E,H and L).
/// The A register is the primary 8-bit accumulator.
/// The other 6 registers can be used as individual registers, or as 3 16-bit
/// register pairs, (BC, DE and HL).
///
/// Some instructions enable the HL register pair as a 16-bit accumulator & a
/// pseudo reg, M.
///
/// The M register can be used almost anywhere that any other registers can use,
/// referring to the memory address pointed to by the HL pair.
/// BC, DE, or HL, (referred to as B, D, H in Intel documents)
/// or SP can be loaded with an immediate 16-bit value (using LXI).
/// Incremented or decremented (using INX and DCX) or added to HL (using DAD).
/// The 8080 has a 16-bit stack pointer, and a 16-bit program counter
#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub opcode: u8,
    pub debug: bool,

    pub pc: u16,
    pub sp: u16,

    // 8-bit Registers
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_h: u8,
    pub reg_l: u8,
    pub reg_m: u8, // pseudo-register

    // 16-bit Register pairs
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,

    pub reg_psw: u16,

    // Status Register (Flags)
    pub sign: bool,
    pub zero: bool, // If the zero bit is one = true
    pub parity: bool,

    pub carry: bool,
    pub half_carry: bool,

    pub cycles: usize,
    pub interrupt: bool,
    pub interrupt_addr: u8,
    shift_offset: u8,
    shift_0: u8,
    shift_1: u8,

    // I/O Read port
    port_0_in: u8, // Input port 0
    port_1_in: u8, // Input port 1
    port_2_in: u8, // Input port 2
    port_3_in: u8, // Bit shift register read / shift in

    // I/O Write port
    port_2_out: u8, // Shift amount (3 bits)
    port_3_out: u8, // Sound bits
    port_4_out: u8, // Shift data
    port_5_out: u8, // Sound bits
    port_6_out: u8, // Watchdog
}

impl Registers {
    pub fn new() -> Registers {

        Registers {
            opcode: 0,
            debug: false,

            pc: 0,
            sp: 0,

            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_m: 0,

            reg_bc: 0,
            reg_de: 0,
            reg_hl: 0,

            reg_psw: 0,

            sign: false,
            zero: false,
            parity: false,

            carry: false,
            half_carry: false,

            cycles: 0,
            interrupt: false,
            interrupt_addr: 0x10,

            shift_offset: 0,
            shift_0: 0,
            shift_1: 0,

            port_0_in: 0x0E,
            port_1_in: 0x08,
            port_2_in: 0x00,
            port_3_in: 0,

            port_2_out: 0,
            port_3_out: 0,
            port_4_out: 0,
            port_5_out: 0,
            port_6_out: 0,
        }
    }
}

#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub registers: &'a mut Registers,
    pub memory: &'a mut Memory,
}
impl<'a> ExecutionContext<'a> {
    pub fn new(interconnect: &'a mut Interconnect) -> ExecutionContext<'a> {
        ExecutionContext {
            registers: &mut interconnect.registers,
            memory: &mut interconnect.memory,
        }
    }

    fn set_sp(&mut self, byte: u16) {
        self.registers.sp = byte & 0xFFFF;
    }

    fn read_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => self.registers.reg_m,
        }
    }

    // Write register value with little endianness.
    fn write_reg(&mut self, reg: Register, value: u8) {
        // Convert byte to little endian.
        match reg {
            Register::A => self.registers.reg_a = value,
            Register::B => self.registers.reg_b = value,
            Register::C => self.registers.reg_c = value,
            Register::D => self.registers.reg_d = value,
            Register::E => self.registers.reg_e = value,
            Register::H => self.registers.reg_h = value,
            Register::L => self.registers.reg_l = value,
            Register::M => self.registers.reg_m = value,
        }
    }

    fn write_rp(&mut self, reg: RegisterPair, value: u8) {
        match reg {
            RegisterPair::BC => self.registers.reg_bc = u16::from(value),
            RegisterPair::DE => self.registers.reg_de = u16::from(value),
            RegisterPair::HL => self.registers.reg_hl = u16::from(value),
            RegisterPair::SP => self.registers.sp = u16::from(value),
        }
    }

    fn adv_pc(&mut self, t: u16) {
        self.registers.pc = self.registers.pc.wrapping_add(t);
    }

    fn adv_cycles(&mut self, t: usize) {
        self.registers.cycles += t;
    }

    fn adc(&mut self, reg: Register) {
        let mut value = 0;
        let w16 = (self.registers.reg_a) + value + (self.registers.carry as u8);

        match reg {
            Register::A => {
                value = self.registers.reg_a + self.registers.reg_a + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::B => {
                value = self.registers.reg_a + self.registers.reg_b + (self.registers.carry as u8);
            },
            Register::C => {
                value = self.registers.reg_a + self.registers.reg_c + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::D => {
                value = self.registers.reg_a + self.registers.reg_d + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::E => {
                value = self.registers.reg_a + self.registers.reg_e + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::H => {
                value = self.registers.reg_a + self.registers.reg_h + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::L => {
                value = self.registers.reg_a + self.registers.reg_l + (self.registers.carry as u8);
                self.registers.reg_a = value as u8 & 0xFF;
            }
            Register::M => {
                let hl: u16 = (u16::from(self.registers.reg_h)) << 8 |
                    (u16::from(self.registers.reg_l)) + (self.registers.carry as u16);
                value = hl as u8;
                self.registers.reg_a = value as u8 & 0xFF;

                self.adv_cycles(3);
            }
        }
        self.registers.zero = value & 0xFF == 0;
        self.registers.sign = value & 0x80 != 0;
        self.registers.carry = (w16 & 0x0_1000) != 0;
        self.registers.parity = self.parity(value as u8 & 0xFF);
        self.registers.half_carry = self.half_carry_add(u16::from(value)) == 0;
    }

    fn add(&mut self, reg: Register) {

        match reg {
            Register::A => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_a;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                // TODO Check if this is correct
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::B => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_b;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::C => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_c;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::D => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_d;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::E => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_e;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::H => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_h;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::L => {
                let mut value = self.registers.reg_a;
                value += self.registers.reg_l;

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value & 0xFF);
            },
            Register::M => {
                let mut value = self.registers.reg_a as u16;
                value += (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);

                self.registers.zero = value & 0xFF == 0;
                self.registers.sign = value & 0x80 != 0;
                self.registers.half_carry = self.half_carry_add(u16::from(value)) != 0;
                self.registers.carry = value > 0xFF;
                self.registers.parity = self.parity(value as u8 & 0xFF);
            },
        }

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ana(&mut self, reg: Register) {
        // Check if the 4th bit is set on all registers
        match reg {
            Register::A => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_a) & 0x08 != 0;
                self.registers.reg_a &= self.registers.reg_a;
            }

            Register::B => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_b) & 0x08 != 0;
                if self.registers.debug {
                    println!(
                        "Setting half carry flag for ANA: {}",
                        self.registers.half_carry
                    );
                }
                self.registers.reg_a &= self.registers.reg_b;
            }

            Register::C => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_c) & 0x08 != 0;
                if self.registers.debug {
                    println!(
                        "Setting half carry flag for ANA: {}",
                        self.registers.half_carry
                    );
                }
                self.registers.reg_a &= self.registers.reg_c;
            }

            Register::D => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_d) & 0x08 != 0;
                if self.registers.debug {
                    println!(
                        "Setting half carry flag for ANA: {}",
                        self.registers.half_carry
                    );
                }
                self.registers.reg_a &= self.registers.reg_d;
            }

            Register::E => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_e) & 0x08 != 0;
                self.registers.reg_a &= self.registers.reg_e;
            }

            Register::H => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_h) & 0x08 != 0;
                self.registers.reg_a &= self.registers.reg_h;
            }

            Register::L => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_l) & 0x08 != 0;
                self.registers.reg_a &= self.registers.reg_l;
            }

            Register::M => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_m) & 0x08 != 0;
                self.registers.reg_a &= self.registers.reg_m;
            }
        }

        self.adv_pc(1);
        self.adv_cycles(4);
    }

    fn ani(&mut self) {
        // The byte of immediate data is ANDed with the contents of the accumulator
        // (reg_a).
        // The Carry bit is reset to zero.
        // Set half carry if the accumulator or opcode and the lower 4 bits are 1.

        self.registers.half_carry = (self.registers.pc | self.registers.pc) & 0x08 != 0;
        self.registers.reg_a &= self.memory.read_imm(self.registers.pc) as u8;

        self.registers.carry = false;
        self.registers.zero = false;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn aci(&mut self) {
        // Add Immediate to Accumulator with Carry
        self.registers.reg_a = self.memory.read_imm(self.registers.pc) as u8;
        self.registers.carry = true;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn adi(&mut self) {
        // Add Immediate to Accumulator
        self.registers.reg_a = self.memory.read_imm(self.registers.pc) as u8;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn jmp(&mut self) {
        self.registers.pc = self.memory.read_imm(self.registers.pc);
        if self.registers.debug {
            println!("Jumping to address: {:04X}", self.registers.pc);
            if self.registers.pc == 0x08F3 {
                println!("Jumping to PrintMessage");
            }
        }
        self.adv_cycles(10);
    }

    // Jump if carry
    fn jc(&mut self) {
        if self.registers.carry {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if no carry
    fn jnc(&mut self) {
        if !self.registers.carry {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Zero (If zero bit is 1)
    // If zero = 1 jump to address
    fn jz(&mut self) {
        if self.registers.zero == true {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Not Zero (if zero bit is 0 jump)
    fn jnz(&mut self) {
        if self.registers.zero == false {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Minus (If sign bit is one)
    fn jm(&mut self) {
        if self.registers.sign {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Positive (If sign bit is zero)
    fn jp(&mut self) {
        if !self.registers.sign {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity even (If parity bit is 1)
    fn jpe(&mut self) {
        if self.registers.parity {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity odd (If parity bit is 0)
    fn jpo(&mut self) {
        if !self.registers.parity {
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump to address in H:L
    fn pchl(&mut self) {
        let hl = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
        self.registers.pc = hl;
        if self.registers.debug {
            println!("Jumping to address: {:0X}", self.registers.pc);
        };
        self.adv_cycles(5);
    }

    fn lxi_sp(&mut self) {
        self.registers.sp = self.memory.read_imm(self.registers.pc);

        self.adv_pc(3);
        self.adv_cycles(10);
    }

    // Load Register Pair Immediate
    // E.g: LXI H, 2000H (2000H is stored in the HL reg pair and acts as as memory
    // pointer)
    // TODO Investigate possible problem here with CPUTEST & 8080EXER
    fn lxi(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                let low = self.memory.read_low(self.registers.pc);
                let high = self.memory.read_high(self.registers.pc);


                self.registers.reg_b = high;
                self.registers.reg_c = low;
            }

            RegisterPair::DE => {
                let low = self.memory.read_low(self.registers.pc);
                let high = self.memory.read_high(self.registers.pc);

                self.registers.reg_d = high;
                self.registers.reg_e = low;
            }

            RegisterPair::HL => {
                let low = self.memory.read_low(self.registers.pc);
                let high = self.memory.read_high(self.registers.pc);

                self.registers.reg_h = high;
                self.registers.reg_l = low;
            }
            RegisterPair::SP => self.registers.sp = self.memory.read_imm(self.registers.pc),
        }
        self.adv_cycles(10);
        self.adv_pc(3);
    }

    // Store Accumulator direct
    fn sta(&mut self) {
        let addr = self.memory.read_imm(self.registers.pc);

        self.memory.memory[addr as usize] = self.registers.reg_a;
        self.adv_pc(3);
        self.adv_cycles(13);
    }

    fn call(&mut self, addr: u16) {
        // CALL instructions occupy three bytes. (See page 34 of the 8080 Programmers
        // Manual)
        // CALL is just like JMP but also pushes a return address to stack.
        let ret: u16 = self.registers.pc + 3;

        match addr {
            0xCC | 0xCD | 0xC4 | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
                // High order byte
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = (ret >> 8 & 0xFF) as u8;
                // Low order byte
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = ret as u8 & 0xFF;

                // Push return address to stack
                self.registers.sp = self.registers.sp.wrapping_sub(2);
            }
            _ => println!("Unknown call address: {:04X}", self.registers.opcode),
        };

        if self.registers.debug {
            // For debugging
            // Match call addr with dissasembled Space Invaders function names
            /* match self.registers.pc {
                0x1E6 => println!("{:02X}, Load DE, prepare for BlockCopy", self.registers.pc),
                0x1EC => println!("Call BlockCopy"),
                0x01D => println!("Call CheckHandleTilt"),
                0x03B => println!("Call DrawnumCredits"),
                0x18DC => println!("Call DrawStatus"),
                0x18D9 => println!("RAM Mirror in ROM"),
                0x1928 => println!("DrawScore"),
                0x1956 => println!("Call ClearScreen"),
                0x1959 => println!("Call DrawScoreHead"),
                0x8F5 => println!("Call DrawChar"),
                0x214F => println!("Call $214F"),
                0x187 => println!("Call Error"),

                _ => println!(),
            }; */
        }

        self.registers.pc = self.memory.read_imm(self.registers.pc);
        self.adv_cycles(17);
    }

    // Call if Minus (if sign bit is 1, indicating a negative result)
    fn cm(&mut self, addr: u16) {
        if self.registers.sign {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(2);
        }
    }

    fn cc(&mut self, addr: u16) {
        if self.registers.carry {
            self.call(addr)
        } else {
            self.adv_cycles(11);
            self.adv_pc(2);
        }
    }

    // Call if Zero
    fn cz(&mut self, addr: u16) {
        if self.registers.zero {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cnz(&mut self, addr: u16) {
        if !self.registers.zero {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // Call If No Carry
    fn cnc(&mut self, addr: u16) {
        if !self.registers.carry {
            self.call(addr);
        } else {
            self.adv_cycles(11)
        }
        self.adv_pc(3);
    }

    fn cma(&mut self) {
        self.registers.reg_a ^= 0xFF;
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    fn cmc(&mut self) {
        self.registers.half_carry = !self.registers.half_carry;
        self.adv_pc(1);
        self.adv_cycles(4);
    }
    fn cmp(&mut self, reg: Register) {
        // Compare Register or Memory With Accumulator

        let result = self.registers.reg_a;
        match reg {

        Register::A => {
                if result < self.registers.reg_a {
                    self.registers.carry = true;
                } else if result == self.registers.reg_a {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_a {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::B => {
                if result < self.registers.reg_b {
                    self.registers.carry = true;
                } else if result == self.registers.reg_b {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_b {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::C => {
               if result < self.registers.reg_c {
                    self.registers.carry = true;
                } else if result == self.registers.reg_c {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_c {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::D => {
                if result < self.registers.reg_d {
                    self.registers.carry = true;
                } else if result == self.registers.reg_d {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_d {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::E => {
                if result < self.registers.reg_e {
                    self.registers.carry = true;
                } else if result == self.registers.reg_e {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_e {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::H => {
                if result < self.registers.reg_h {
                    self.registers.carry = true;
                } else if result == self.registers.reg_h {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_h {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::L => {
               if result < self.registers.reg_l {
                    self.registers.carry = true;
                } else if result == self.registers.reg_l {
                    self.registers.zero = true;
                }
                else if result > self.registers.reg_l {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::M => {
                let hl =  (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                if result < hl as u8 {
                    self.registers.carry = true;
                } else if result == hl as u8 {
                    self.registers.zero = true;
                }
                else if result > hl as u8 {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
                self.adv_cycles(4);
            }
        };
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Compare Immediate with Accumulator
    fn cpi(&mut self) {
        // Fetch byte out of memory which we will use to compare & set flags with.
        let value = self.memory.read(self.registers.pc + 1);

        // Compare is done with subtraction.
        // Compare the result of the accumulator with the immediate address.
        if self.registers.debug {
            println!("Value: {:02X}", value);
            println!("A reg: {:02X}", self.registers.reg_a);
        }

        let result = self.registers.reg_a.wrapping_sub(value);
        if self.registers.debug {
            println!("Result: {:X}", result);
            println!("Zero result: {:X}", result & 0xFF);
        }
        self.registers.sign = result & 0x80 != 0;
        self.registers.zero = result & 0xFF == 0;
        self.registers.half_carry = !self.half_carry_sub(value as u16) != 0;
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.parity = self.parity(result as u8);

        self.adv_pc(2);
        self.adv_cycles(7);
    }


    // Compare Parity Even
    fn cpe(&mut self, addr: u16) {
        if self.registers.parity {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }
    // TODO Consolidate
    fn cpo(&mut self, addr: u16) {
        if !self.registers.parity {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // CALL if plus (if sign bit is zero, indicating a positive result)
    fn cp(&mut self, addr: u16) {
        if !self.registers.sign {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn dad(&mut self, reg: RegisterPair) {
        // Double precision ADD.
        // For these instructions, HL functions as an accumulator.
        // DAD B means BC + HL --> HL. DAD D means DE + HL -- HL.


        match reg {
            RegisterPair::BC => {
                let mut value = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                value += (self.registers.reg_b as u16) << 8 | (self.registers.reg_c as u16);
                self.registers.carry = 0 < value as u16 & 0xFFFF_0000;
            }

            RegisterPair::DE => {
                let mut value = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                value += (self.registers.reg_d as u16) << 8 | (self.registers.reg_e as u16);
                self.registers.carry = 0 < value as u16 & 0xFFFF_0000;
            }

            RegisterPair::HL => {
                let mut value = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                value.wrapping_add((self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16));
                self.registers.carry = 0 < value as u16 & 0xFFFF_0000;
            }
            // DAD SP
            _ => {
                let mut value = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                value += self.registers.sp as u16;
                self.registers.carry = 0 < value as u16 & 0xFFFF_0000;
            }
        }
        self.adv_cycles(10);
        self.adv_pc(1);
    }

    // Decrement memory or register
    fn dcr(&mut self, reg: Register) {
        // Example:
        // If the H register contains 3AH, and the L register contains 7CH
        // and memory location 3A7CH contains 40H, the instruction:
        // DCR M will cause memory location 3A7CH to contain 3FH.

        match reg {
            Register::A => {
                let value = self.registers.reg_a.wrapping_sub(1);
                self.registers.reg_a = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::B => {
                let value = self.registers.reg_b.wrapping_sub(1);
                self.registers.reg_b = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::C => {
                let value = self.registers.reg_c.wrapping_sub(1);
                self.registers.reg_c = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::D => {
                let value = self.registers.reg_d.wrapping_sub(1);
                self.registers.reg_d = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::E => {
                let value = self.registers.reg_e.wrapping_sub(1);
                self.registers.reg_e = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::H => {
                let value = self.registers.reg_h.wrapping_sub(1);
                self.registers.reg_h = value & 0xFF;
                self.registers.half_carry = value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::L => {
                let value = self.registers.reg_l.wrapping_sub(1);
                self.registers.reg_l = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(5);
            }

            // TODO Investigate if this should change or read from H or L registers
            Register::M => {
                let value = self.registers.reg_m.wrapping_sub(1);

                self.registers.reg_m = value & 0xFF;
                self.registers.half_carry = !value & 0x0F == 0x0F;
                self.registers.zero = value & 0xFF == 0;
                self.registers.parity = self.parity(value & 0xFF);
                self.registers.sign = value & 0x80 != 0;
                self.adv_cycles(6);
            }
        }
        self.adv_pc(1);
    }

    fn dcx(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                let mut bc = self.registers.reg_b.wrapping_shl(8) | self.registers.reg_c;
                bc.wrapping_sub(1);
                self.registers.reg_b = bc.wrapping_shl(8) & 0xFF;
                self.registers.reg_c = bc.wrapping_shl(0) & 0xFF;
            }
            RegisterPair::DE => {
                let mut de = self.registers.reg_d.wrapping_shl(8) | self.registers.reg_e;
                de.wrapping_sub(1);
                self.registers.reg_d = de.wrapping_shl(8) & 0xFF;
                self.registers.reg_e = de.wrapping_shl(0) & 0xFF;
            }
            RegisterPair::HL => {
                let mut hl = self.registers.reg_h.wrapping_shl(8) | self.registers.reg_l;
                hl.wrapping_sub(1);
                self.registers.reg_h = hl.wrapping_shl(8) & 0xFF;
                self.registers.reg_l = hl.wrapping_shl(0) & 0xFF;
            }
            RegisterPair::SP => self.registers.sp = self.registers.sp.wrapping_sub(1),
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // TODO
    fn daa(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
        panic!("DAA");
    }
    fn di(&mut self) {
        if self.registers.debug {
            println!("Disabling interrupts");
        }
        self.registers.interrupt = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ei(&mut self) {
        if self.registers.debug {
            println!("Enabling interrupts");
        }
        self.registers.interrupt = true;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {
        // The contents of the accumulator are rotated one bit position to the left.
        // The high-order bit of the accumulator replaces the carry bit
        // while the carry bit replaces the high-order bit of the accumulator
        // Conditional flags affected: Carry
        self.registers.reg_a = self.registers.reg_a << 1;
        self.registers.carry = (self.registers.reg_a << 1) | ((self.registers.reg_a) & 0x40) != 1;

        self.adv_pc(1);
        self.adv_cycles(4);
    }
    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        // If last bit is 1 bit shift one up so that the accumulator is 1
        self.registers.reg_a = (self.registers.reg_a >> 1) | (self.registers.reg_a as u8) << 7;
        // self.registers.carry = self.registers.reg_a & 0x08 != 0;
        self.registers.carry = self.registers.reg_a & 0x1 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 higher bits are 1 we set the carry flag.
        self.registers.reg_a = self.registers.reg_a.rotate_left(1);
        self.registers.carry = self.registers.reg_a & 0x08 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn rrc(&mut self) {
        // The Carry bit is set equal to the low-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        self.registers.carry = self.registers.reg_a & 0x01 != 0;
        self.registers.reg_a = (self.registers.reg_a >> 1) | ((self.registers.reg_a & 0x1) << 7);
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Return if no carry
    fn rnc(&mut self) {
        if !self.registers.carry {
            self.ret();
        } else {
            self.adv_pc(1);
        }
    }
    // Return if Parity Even
    fn rpe(&mut self) {
        if self.registers.parity {
            self.ret()
        } else {
            self.adv_pc(1);
        }
    }
    // Return if Parity Odd
    fn rpo(&mut self) {
        if !self.registers.parity {
            self.ret()
        } else {
            self.adv_pc(1);
        }
    }
    // Return if Carry
    fn rc(&mut self) {
        // If Carry flag is set, return from subroutine
        if self.registers.carry {
            self.adv_cycles(11);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }

    // TODO
    fn rnz(&mut self) {
        if !self.registers.zero {
            self.adv_cycles(11);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if minus
    fn rm(&mut self) {
        if self.registers.sign {
            self.ret();
            self.adv_cycles(11)
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if positive (if sign bit is 0)
    fn rp(&mut self) {
        if !self.registers.sign {
            self.ret();
            self.adv_cycles(11)
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    fn rz(&mut self) {
        if self.registers.zero {
            self.ret();
            self.adv_cycles(11)
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }

    // Move Immediate Data
    fn mvi(&mut self, reg: Register) {
        // The MVI instruction uses a 8-bit data quantity, as opposed to
        // LXI which uses a 16-bit data quantity.

        // First MVI instruction is:
        // Address  Bytes    Operand  Exp       Comment
        // 18D7:    06 00    LD       B,$00    ; Count 256 bytes
        // In our case we get 0006 with self.memory.read, but it's stored as 06.
        let mut value = self.memory.read(self.registers.pc + 1);
        if self.registers.debug {
            println!("MVI value: {:04X}", value);
        }
        match reg {
            Register::A => self.registers.reg_a = value,
            Register::B => self.registers.reg_b = value,
            Register::C => self.registers.reg_c = value,
            Register::D => self.registers.reg_d = value,
            Register::E => self.registers.reg_e = value,
            Register::H => self.registers.reg_h = value,
            Register::L => self.registers.reg_l = value,
            Register::M => {
                self.registers.reg_m = value;
                self.adv_cycles(3);
            }
        }
        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // TODO Investigate which addr value is correct
    fn lda(&mut self) {
        let addr = self.memory.read_imm(self.registers.pc + 3) as u16;
        self.registers.reg_a = addr as u8;
        self.adv_cycles(13);
        self.adv_pc(3);
    }

    fn ldax(&mut self, reg: RegisterPair) {
        // LDAX(Load accumulator indirect):
        // The contents of the designated register pair point to a memory location.
        // This instruction copies the contents of that memory location into the
        // accumulator. The contents of either the register pair or the
        // memory location are not altered.

        match reg {
            RegisterPair::BC => {
                let addr = (self.registers.reg_b as u16) << 8 | (self.registers.reg_c as u16);
                self.registers.reg_a = self.memory.memory[addr as usize];
            }

            RegisterPair::DE => {
                let addr = (self.registers.reg_d as u16) << 8 | (self.registers.reg_e as u16);
                self.registers.reg_a = self.memory.memory[addr as usize];
            }

            _ => println!("LDAX on invalid register"),
        };

        self.adv_cycles(7);
        self.adv_pc(1);
    }

    fn lhld(&mut self) {
        // Load the HL register with 16 bits found at addr & addr + 1
        // The byte at the memory address formed by concatenating HI ADD with LOW ADD
        // replaces the contents of the L register.
        // The byte at the next higher memory address replaces the contents of the H
        // register.
        // L <- (adr); H<-(adr+1)

        self.registers.reg_l = self.memory.read_imm(self.registers.pc) as u8;
        self.registers.reg_h = self.memory.read_imm(self.registers.pc + 1) as u8;

        self.adv_cycles(16);
        self.adv_pc(3);
    }

    // Read one byte from input device #0 into the accumulator
    // Instructions in this class occupy 2 bytes.
    fn input(&mut self) {
        self.registers.reg_a = self.memory.read_next_byte(self.registers.pc);
        self.adv_cycles(10);
        self.adv_pc(2);
    }

    // TODO Variable value is unused (figure out how to handle flags or handle flags per register?)
    fn inr(&mut self, reg: Register) {
        match reg {
            Register::A => {
                let value = self.registers.reg_a.wrapping_add(1);
                self.registers.reg_a = value & 0xFF;

                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::B => {
                let value = self.registers.reg_b.wrapping_add(1);
                self.registers.reg_b = value & 0xFF;

                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::C => {
                let value = self.registers.reg_c.wrapping_add(1);
                self.registers.reg_c = value & 0xFF;
                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::D => {
                let value = self.registers.reg_d.wrapping_add(1);
                self.registers.reg_d = value & 0xFF;
                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::E => {
                let value = self.registers.reg_e.wrapping_add(1);
                self.registers.reg_e = value & 0xFF;
                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::H => {
                let value = self.registers.reg_h.wrapping_add(1);
                self.registers.reg_h = value & 0xFF;
                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::L => {
                let value = self.registers.reg_l.wrapping_add(1);
                self.registers.reg_l = value & 0xFF;
                self.registers.sign = value & 0x80 != 0;
                self.registers.zero = value == 0;
                self.registers.half_carry = !value & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
            Register::M => {
                let value = (self.registers.reg_h.wrapping_add(1) as u16) << 8 | (self.registers.reg_l.wrapping_add(1) as u16);
                self.registers.reg_h = value as u8 & 0xFF;

                self.registers.sign = value as u8 & 0x80 != 0;
                self.registers.zero = value as u8 == 0;
                self.registers.half_carry = !value as u8 & 0x0F == 0;
                self.registers.parity = self.parity(value as u8);
            }
        };

        if reg == Register::M {
            self.adv_cycles(10);
        } else {
            self.adv_cycles(5);
        }
        self.adv_pc(1);
    }

    fn inx(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                self.registers.reg_c = self.registers.reg_c.wrapping_add(1);
                if self.registers.reg_c == 0 {
                    self.registers.reg_b += 1;
                }
            }

            RegisterPair::DE => {
                self.registers.reg_e = self.registers.reg_e.wrapping_add(1);
                if self.registers.reg_e == 0 {
                    self.registers.reg_d = self.registers.reg_d.wrapping_add(1);
                }
            }

            RegisterPair::HL => {
                self.registers.reg_l = self.registers.reg_l.wrapping_add(1);
                if self.registers.reg_l == 0 {
                    self.registers.reg_h += 1;
                }
            }
            RegisterPair::SP => {
                self.registers.sp += 1;
                self.adv_pc(1);
            }
        };
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn push(&mut self, reg: Register) {
        match reg {
            Register::B => {
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = self.registers.reg_b;
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = self.registers.reg_c;
                self.registers.sp = self.registers.sp.wrapping_sub(2);
            }

            Register::D => {
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = self.registers.reg_d;
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = self.registers.reg_e;
                self.registers.sp = self.registers.sp.wrapping_sub(2);
            }

            Register::H => {
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = self.registers.reg_h;
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = self.registers.reg_l;
                self.registers.sp = self.registers.sp.wrapping_sub(2);
            }

            _ => println!("Unknown push instruction"),
        }
        self.adv_cycles(11);
        self.adv_pc(1);
    }

    fn push_psw(&mut self) {
        self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_a;
        self.memory.memory[self.registers.sp as usize - 2] =
           if self.registers.zero { 0x40 } else { 0x0 } |
           if self.registers.sign { 0x80 } else { 0x0 } |
           if self.registers.parity { 0x04 } else { 0x0 } |
           if self.registers.carry { 0x01 } else { 0x0 } |
           if self.registers.half_carry { 0x10 } else { 0x0 } |
           0x02;

        self.registers.sp = self.registers.sp.wrapping_sub(2);

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Store the contents of the accumulator addressed by registers B, C
    // or by registers D and E.
    fn stax(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                let bc = self.registers.reg_b.wrapping_shl(8) | self.registers.reg_c;
                self.memory.memory[bc as usize] = self.registers.reg_a;
            }
            RegisterPair::DE => {
                let de = self.registers.reg_d.wrapping_shl(8) | self.registers.reg_e;
                self.memory.memory[de as usize] = self.registers.reg_a;
            }
            RegisterPair::HL => {
                let hl = self.registers.reg_h.wrapping_shl(8) | self.registers.reg_l;
                self.memory.memory[hl as usize] = self.registers.reg_a;
            }
            RegisterPair::SP => eprintln!("STAX should not run on SP register"),
        }

        self.adv_cycles(7);
        self.adv_pc(1);
    }

    // SBB Subtract Register or Memory from Accumulator with borrow
    fn sbb(&mut self, reg: Register) {
        let mut reg_a = self.registers.reg_a;
        let carry = if self.registers.carry { 0x01 } else { 0x00 };

        match reg {
            Register::A => reg_a -= self.registers.reg_a - carry,
            Register::B => reg_a -= self.registers.reg_b - carry,
            Register::C => reg_a -= self.registers.reg_c - carry,
            Register::D => reg_a -= self.registers.reg_d - carry,
            Register::E => reg_a -= self.registers.reg_e - carry,
            Register::H => reg_a -= self.registers.reg_h - carry,
            Register::L => reg_a -= self.registers.reg_l - carry,
            Register::M => reg_a -= self.registers.reg_m - carry,
        };

        if reg == Register::M {
            self.adv_cycles(4);
        }

        self.registers.reg_a = reg_a;
        self.registers.half_carry = self.half_carry_sub(reg_a as u16 & 0xFF) != 0;
        self.registers.parity = self.parity(reg_a);
        // If the result from the subtraction is 1 the zero bit is set
        self.registers.zero = reg_a == 0;
        self.registers.sign = reg_a & 0x80 != 0;
        // Check if the carry bit is set if our result
        self.registers.carry = reg_a & 0x0100 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // SUB Subtract Register or Memory From Accumulator
    fn sub(&mut self, reg: Register) {
        let mut reg_a = self.registers.reg_a;

        match reg {
            Register::A => reg_a -= self.registers.reg_a,
            Register::B => reg_a -= self.registers.reg_b,
            Register::C => reg_a -= self.registers.reg_c,
            Register::D => reg_a -= self.registers.reg_d,
            Register::E => reg_a -= self.registers.reg_e,
            Register::H => reg_a -= self.registers.reg_h,
            Register::L => reg_a -= self.registers.reg_l,
            Register::M => reg_a -= self.registers.reg_m,
        };

        if reg == Register::M {
            self.adv_cycles(4);
        }

        self.registers.reg_a = reg_a;
        self.registers.half_carry = self.half_carry_sub(reg_a as u16 & 0xFF) != 0;
        self.registers.parity = self.parity(reg_a);
        self.registers.zero = reg_a == 0;
        self.registers.sign = reg_a & 0x80 != 0;
        self.registers.carry = reg_a & 0x0100 != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Set Carry (set carry bit to 0)
    fn stc(&mut self) {
        self.registers.carry = false;
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    fn xra(&mut self, reg: Register) {
        match reg {
            Register::A => self.registers.reg_a ^= self.registers.reg_a,
            Register::B => self.registers.reg_a ^= self.registers.reg_b,
            Register::C => self.registers.reg_a ^= self.registers.reg_c,
            Register::D => self.registers.reg_a ^= self.registers.reg_d,
            Register::E => self.registers.reg_a ^= self.registers.reg_e,
            Register::H => self.registers.reg_a ^= self.registers.reg_h,
            Register::L => self.registers.reg_a ^= self.registers.reg_l,
            Register::M => {
                let value = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                self.registers.reg_a ^= value as u8;
                self.adv_cycles(3);
            }
        }

        self.registers.half_carry = false;
        self.registers.carry = false;
        self.registers.zero = self.registers.reg_a == 0;
        self.registers.sign = self.registers.reg_a & 0x80 != 0;
        self.registers.parity = self.parity(self.registers.reg_a);

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRI Exclusive-Or Immediate with Accumulator
    fn xri(&mut self) {
        let value = self.memory.read_byte(self.registers.pc + 1);
        self.registers.reg_a ^= value;

        self.registers.half_carry = false;
        self.registers.carry = false;
        self.registers.zero = self.registers.reg_a == 0;
        self.registers.sign = self.registers.reg_a & 0x80 != 0;
        self.registers.parity = self.parity(self.registers.reg_a);

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn xchg(&mut self) {
        let h = self.registers.reg_h;
        let l = self.registers.reg_l;

        self.registers.reg_h = self.registers.reg_d;
        self.registers.reg_l = self.registers.reg_e;

        self.registers.reg_d = h;
        self.registers.reg_e = l;
        self.adv_pc(1);
        self.adv_cycles(5);
    }

    fn xthl(&mut self) {
        // Swap H:L with top word on stack
        let h = self.registers.reg_h;
        let l = self.registers.reg_l;

        self.registers.reg_l = self.memory.memory[self.registers.sp as usize + 0];
        self.registers.reg_h = self.memory.memory[self.registers.sp as usize + 1];
        self.memory.memory[self.registers.sp as usize + 0] = l;
        self.memory.memory[self.registers.sp as usize + 1] = h;

        self.adv_cycles(18);
        self.adv_pc(1);
    }


    fn pop(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                self.registers.reg_c = self.memory.memory[(self.registers.sp as usize + 0) & 0xFFFF];
                self.registers.reg_b = self.memory.memory[(self.registers.sp as usize + 1) & 0xFFFF];
            }

            RegisterPair::DE => {
                self.registers.reg_e = self.memory.memory[(self.registers.sp as usize + 0) & 0xFFFF];
                self.registers.reg_d = self.memory.memory[(self.registers.sp as usize + 1) & 0xFFFF];
            }

            RegisterPair::HL => {
                self.registers.reg_l = self.memory.memory[(self.registers.sp as usize + 0) & 0xFFFF];
                self.registers.reg_h = self.memory.memory[(self.registers.sp as usize + 1) & 0xFFFF];
            }
            RegisterPair::SP => println!("POP SP called at POP instruction, please fix"),
        }
        self.registers.sp = self.registers.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_psw(&mut self) {
        self.registers.reg_a = self.memory.memory[self.registers.sp as usize + 1];
        self.registers.zero = self.memory.memory[self.registers.sp as usize] & 0x40 != 0;

        self.registers.sign = self.memory.memory[self.registers.sp as usize] & 0x80 != 0;
        self.registers.parity = self.memory.memory[self.registers.sp as usize] & 0x04 != 0;
        self.registers.carry = self.memory.memory[self.registers.sp as usize] & 0x01 != 0;
        self.registers.half_carry = self.memory.memory[self.registers.sp as usize] & 0x10 != 0;

        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.adv_cycles(10);
        self.adv_pc(1);
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.memory.read_word(self.registers.sp);
        if self.registers.debug {
            println!("Popping stack. SP value: {:04X}", sp);
        }
        self.registers.sp += 2;
        sp
    }

    fn ret(&mut self) {
        let low = self.memory.memory[self.registers.sp as usize] as u16;
        let high = self.memory.memory[self.registers.sp as usize + 1] as u16;
        let ret: u16 = (high as u16) << 8 | (low as u16);

        if self.registers.debug {
            println!("Returning to: {:04X}", ret);
        }
        self.adv_cycles(10);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.registers.pc = ret;
    }

    fn out(&mut self) {
        let port = self.memory.read_low(self.registers.pc);
        match port {
            // Set offset size for shift register
            0x02 => {
                self.registers.shift_offset = self.registers.reg_a & 0x7;
            }
            // sound port
            0x03 => println!("Sound not implemented"),
            // Set shift register values
            0x04 => {
                self.registers.shift_0 = self.registers.shift_1;
                self.registers.shift_1 = self.registers.reg_a;
            }
            // Sound port
            0x05 => println!("Sound not implemented"),
            // Watchdog port
            0x06 => println!("Watchdog timer not implemented"),
            _ => println!("Out port does not match implementation"),
        }
        self.adv_pc(2);
        self.adv_cycles(10);
    }
    fn ora(&mut self, reg: Register) {
        match reg {
            Register::A => self.registers.reg_a |= self.registers.reg_a,
            Register::B => self.registers.reg_a |= self.registers.reg_b,
            Register::C => self.registers.reg_a |= self.registers.reg_c,
            Register::D => self.registers.reg_a |= self.registers.reg_d,
            Register::E => self.registers.reg_a |= self.registers.reg_e,
            Register::H => self.registers.reg_a |= self.registers.reg_h,
            Register::L => self.registers.reg_a |= self.registers.reg_l,
            Register::M => {
                let hl = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                self.registers.reg_a |= hl as u8;
                self.adv_cycles(3);
            }
        }
        self.registers.half_carry = false;
        self.registers.carry = false;
        self.registers.zero = self.registers.reg_a == 0;
        self.registers.sign = 0x80 == (self.registers.reg_a & 0x80);
        self.registers.parity = self.parity(self.registers.reg_a);

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // Or Immediate with Accumulator
    fn ori(&mut self) {
        self.registers.reg_a |= self.memory.read_imm(self.registers.pc) as u8;
        self.registers.half_carry = false;
        self.registers.carry = false;
        self.registers.zero = self.registers.reg_a == 0;
        self.registers.sign = 0x80 == (self.registers.reg_a & 0x80);
        self.registers.parity = self.parity(self.registers.reg_a);

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn mov(&mut self, dst: Register, src: Register) {
        let value = self.read_reg(src);
        let addr: u16 = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);

        match dst {
            Register::A => {
                if src == Register::M {
                    self.registers.reg_a = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value)
                }
            }
            Register::B => {
                if src == Register::M {
                    self.registers.reg_b = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::C => {
                if src == Register::M {
                    self.registers.reg_c = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::D => {
                if src == Register::M {
                    self.registers.reg_d = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value) ;
                }
            }
            Register::E => {
                if src == Register::M {
                    self.registers.reg_e = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::H => {
                if src == Register::M {
                    self.registers.reg_h = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                self.write_reg(dst, value);
                }
            }
            Register::L => {
                if src == Register::M {
                    self.registers.reg_l = self.memory.memory[addr as usize];
                    self.adv_cycles(2);
                } else {
                self.write_reg(dst, value);
                }
            }
            Register::M => {
                match src {
                    Register::A => self.memory.memory[addr as usize] = self.registers.reg_a,
                    Register::B => self.memory.memory[addr as usize] = self.registers.reg_b,
                    Register::C => self.memory.memory[addr as usize] = self.registers.reg_c,
                    Register::D => self.memory.memory[addr as usize] = self.registers.reg_d,
                    Register::E => self.memory.memory[addr as usize] = self.registers.reg_e,
                    Register::H => self.memory.memory[addr as usize] = self.registers.reg_h,
                    Register::L => self.memory.memory[addr as usize] = self.registers.reg_l,
                    Register::M => self.memory.memory[addr as usize] = self.registers.reg_m,
                }
                self.adv_cycles(2);

                }
            }
        if self.registers.debug {
            println!("MOV, Source: {:?}, Destination: {:?}, Value: {:04X}", src, dst, value);;
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // RESET (used for interrupt jump / calls)
    // TODO Investigate RST(5)
    pub fn rst(&mut self, value: u8) {
        // Address to return to after interrupt is finished.

        let ret = self.registers.pc;
        if self.registers.debug {  println!("RST return address: {:04X}", ret); }

        self.memory.memory[self.registers.sp as usize - 1] = (ret >> 8) as u8;
        self.memory.memory[self.registers.sp as usize - 2] = ret as u8;


        match value {
            0 => self.registers.pc = 0x0000,
            1 => self.registers.pc = 0x0008,
            2 => self.registers.pc = 0x0010,
            3 => self.registers.pc = 0x0018,
            4 => self.registers.pc = 0x0020,
            5 => self.registers.pc = 0x0028,
            6 => self.registers.pc = 0x0030,
            7 => self.registers.pc = 0x0038,
            _ => println!("Couldn't match RST value, {:04X}", value)
        };

        self.registers.sp -= 2;
        self.registers.pc = (value & 0x38).into();
        self.adv_cycles(11);

    }

    fn sphl(&mut self) {
        self.registers.sp = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16) as u16;
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // Store H & L direct
    fn shld(&mut self) {
        let hl = (self.registers.reg_h as u16) << 8 | self.registers.reg_l as u16;
        self.memory.memory[hl as usize + 1] = self.registers.reg_h;
        self.memory.memory[hl as usize] = self.registers.reg_l;

        self.adv_cycles(16);
        self.adv_pc(3);
    }

    pub fn nop(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }


    pub fn decode(&mut self, instruction: Instruction) {
        if self.registers.debug {
            println!(
                "Opcode: {:#02X} Instruction: {:?},",
                self.registers.opcode,
                instruction
            );
        }
        if self.registers.debug {
            println!(
                "PC: {:04X}, SP: {:X}, Cycles: {}",
                self.registers.pc,
                self.registers.sp,
                self.registers.cycles
            );
            println!(
                    "Registers: A: {:02X}, B: {:02X}, C: {:02X}, D: {:02X}, \
                E: {:02X}, H: {:02X}, L: {:02X}, M: {:02X}",
                    self.registers.reg_a,
                    self.registers.reg_b,
                    self.registers.reg_c,
                    self.registers.reg_d,
                    self.registers.reg_e,
                    self.registers.reg_h,
                    self.registers.reg_l,
                    self.registers.reg_m,
                );


            let bc = (self.registers.reg_b as u16) << 8 | self.registers.reg_c as u16;
            let de = (self.registers.reg_d as u16) << 8 | self.registers.reg_e as u16;
            let hl = (self.registers.reg_h as u16) << 8 | self.registers.reg_l as u16;

            let stack = (self.memory.memory[self.registers.sp as usize + 1] as u16) << 8 |
                self.memory.memory[self.registers.sp as usize] as u16;


            println!(
                "Register Pairs: BC: {:04X}, DE: {:04X}, HL: {:04X}",
                bc,
                de,
                hl
            );
            println!(
                    "Flags: S: {}, Z: {}, P: {}, C: {}, AC: {}, Interrupt: {}, Interrupt addr: {:02X}",
                    self.registers.sign,
                    self.registers.zero,
                    self.registers.parity,
                    self.registers.carry,
                    self.registers.half_carry,
                    self.registers.interrupt,
                    self.registers.interrupt_addr,
                );
            println!("Stack: {:04X}", stack as u16);
        };

        match instruction {
            Instruction::Nop => self.nop(),

            Instruction::Aci => self.aci(),
            Instruction::Add(reg) => self.add(reg),
            Instruction::Adi => self.adi(),
            Instruction::Adc(reg) => self.adc(reg),
            Instruction::Ana(reg) => self.ana(reg),
            Instruction::Ani => self.ani(),

            Instruction::Call(addr) => self.call(addr),
            Instruction::Cc(addr) => self.cc(addr),
            Instruction::Cpi => self.cpi(),
            Instruction::Cpo(addr) => self.cpo(addr),
            Instruction::Cnz(addr) => self.cnz(addr),
            Instruction::Cz(addr) => self.cz(addr),
            Instruction::Cm(addr) => self.cm(addr),
            Instruction::Cnc(addr) => self.cnc(addr),
            Instruction::Cma => self.cma(),
            Instruction::Cmc => self.cmc(),

            Instruction::Cmp(reg) => self.cmp(reg),
            Instruction::Cp(addr) => self.cp(addr),
            Instruction::Cpe(addr) => self.cpe(addr),
            Instruction::Dcr(reg) => self.dcr(reg),
            Instruction::Dcx(reg) => self.dcx(reg),
            Instruction::DcxSp(reg) => self.dcx(reg),

            Instruction::Di => self.di(),
            Instruction::Daa => self.daa(),
            Instruction::Dad(reg) => self.dad(reg),
            Instruction::DadSp(reg) => self.dad(reg),
            Instruction::Ei => self.ei(),
            Instruction::Jc => self.jc(),
            Instruction::Jmp => self.jmp(),
            Instruction::Jp => self.jp(),
            Instruction::Jpe => self.jpe(),
            Instruction::Jpo => self.jpo(),

            Instruction::Mov(dst, src) => self.mov(dst, src),
            Instruction::Mvi(reg) => self.mvi(reg),
            Instruction::Sub(reg) => self.sub(reg),
            Instruction::Sbb(reg) => self.sbb(reg),

            Instruction::Xra(reg) => self.xra(reg),
            Instruction::Rp => self.rp(),
            Instruction::Rpe => self.rpe(),
            Instruction::Ret => self.ret(),

            Instruction::Pop(reg) => self.pop(reg),
            Instruction::PopPsw(reg) => self.pop_psw(),
            Instruction::Push(reg) => self.push(reg),
            Instruction::PushPsw() => self.push_psw(),

            Instruction::In => self.input(),
            Instruction::Inr(reg) => self.inr(reg),
            Instruction::Inx(reg) => self.inx(reg),
            Instruction::InxSp(reg) => self.inx(reg),
            Instruction::Out => self.out(),

            Instruction::Sta => self.sta(),
            Instruction::Stax(reg) => self.stax(reg),
            Instruction::Lda => self.lda(),
            Instruction::Ldax(reg) => self.ldax(reg),
            Instruction::Lhld => self.lhld(),
            Instruction::Lxi(reg) => self.lxi(reg),
            Instruction::LxiSp => self.lxi_sp(),
            Instruction::Ral => self.ral(),
            Instruction::Rar => self.rar(),
            Instruction::Rlc => self.rlc(),
            Instruction::Rc => self.rc(),
            Instruction::Rnc => self.rnc(),
            Instruction::Rrc => self.rrc(),
            Instruction::Rim => println!("Not implemented: {:?}", instruction),
            Instruction::Rpo => self.rpo(),
            Instruction::Rst(0) => self.rst(0),
            Instruction::Rst(1) => self.rst(1),
            Instruction::Rst(2) => self.rst(2),
            Instruction::Rst(3) => self.rst(3),
            Instruction::Rst(4) => self.rst(4),
            Instruction::Rst(5) => self.rst(5),
            Instruction::Rst(6) => self.rst(6),
            Instruction::Rst(7) => self.rst(7),

            Instruction::Rnz => self.rnz(),
            Instruction::Rm => self.rm(),
            Instruction::Rz => self.rz(),

            Instruction::Hlt => self.hlt(),

            Instruction::Sim => println!("Not implemented: {:?}", instruction),

            Instruction::Stc => self.stc(),
            Instruction::Shld => self.shld(),
            Instruction::Sphl => self.sphl(),

            Instruction::Ora(reg) => self.ora(reg),
            Instruction::Ori => self.ori(),

            // Jump instructions can probably use just one function?
            Instruction::Jnc => self.jnc(),
            Instruction::Jnz => self.jnz(),
            Instruction::Jm => self.jm(),
            Instruction::Jz => self.jz(),

            Instruction::Xri => self.xri(),
            Instruction::Xchg => self.xchg(),
            Instruction::Xthl => self.xthl(),
            Instruction::Pchl => self.pchl(),

            _ => println!("Unknown instruction {:#X}", self.registers.opcode),
        }
    }

    pub fn execute_instruction(&mut self, instruction: u8) {
        use self::Register::*;
        use self::RegisterPair::*;
        self.registers.opcode = instruction;

        match self.registers.opcode {
            0x00 => self.decode(Instruction::Nop),
            0x01 => self.decode(Instruction::Lxi(BC)),
            0x02 => self.decode(Instruction::Stax(BC)),
            0x03 => self.decode(Instruction::Inx(BC)),
            0x04 => self.decode(Instruction::Inr(B)),
            0x05 => self.decode(Instruction::Dcr(B)),
            0x06 => self.decode(Instruction::Mvi(B)),
            0x07 => self.decode(Instruction::Rlc),
            0x08 => self.decode(Instruction::Nop),
            0x09 => self.decode(Instruction::Dad(BC)),

            0x0A => self.decode(Instruction::Ldax(BC)),
            0x0B => self.decode(Instruction::Dcx(BC)),
            0x0C => self.decode(Instruction::Inr(C)),
            0x0D => self.decode(Instruction::Dcr(D)),
            0x0E => self.decode(Instruction::Mvi(C)),
            0x0F => self.decode(Instruction::Rrc),

            0x10 => self.decode(Instruction::Nop),
            0x11 => self.decode(Instruction::Lxi(DE)),
            0x12 => self.decode(Instruction::Stax(DE)),
            0x13 => self.decode(Instruction::Inx(DE)),
            0x14 => self.decode(Instruction::Inr(D)),
            0x15 => self.decode(Instruction::Dcr(D)),
            0x16 => self.decode(Instruction::Mvi(D)),
            0x17 => self.decode(Instruction::Ral),
            0x18 => self.decode(Instruction::Nop),
            0x19 => self.decode(Instruction::Dad(DE)),

            0x1A => self.decode(Instruction::Ldax(DE)),
            0x1B => self.decode(Instruction::Dcx(DE)),
            0x1C => self.decode(Instruction::Inr(E)),
            0x1D => self.decode(Instruction::Dcr(E)),
            0x1E => self.decode(Instruction::Mvi(E)),
            0x1F => self.decode(Instruction::Rar),

            0x20 => self.decode(Instruction::Nop),
            0x21 => self.decode(Instruction::Lxi(HL)),
            0x22 => self.decode(Instruction::Shld),
            0x23 => self.decode(Instruction::Inx(HL)),
            0x24 => self.decode(Instruction::Inr(H)),
            0x25 => self.decode(Instruction::Dcr(H)),
            0x26 => self.decode(Instruction::Mvi(H)),
            0x27 => self.decode(Instruction::Daa),
            0x28 => self.decode(Instruction::Nop),
            0x29 => self.decode(Instruction::Dad(HL)),

            0x2A => self.decode(Instruction::Lhld),
            0x2B => self.decode(Instruction::Dcx(HL)),
            0x2C => self.decode(Instruction::Inr(L)),
            0x2D => self.decode(Instruction::Dcr(L)),
            0x2E => self.decode(Instruction::Mvi(L)),
            0x2F => self.decode(Instruction::Cma),

            0x30 => self.decode(Instruction::Nop),
            0x31 => self.decode(Instruction::Lxi(SP)),
            0x32 => self.decode(Instruction::Sta),
            0x33 => self.decode(Instruction::InxSp(SP)),
            0x34 => self.decode(Instruction::Inr(M)),
            0x35 => self.decode(Instruction::Dcr(M)),
            0x36 => self.decode(Instruction::Mvi(M)),
            0x37 => self.decode(Instruction::Stc),
            0x38 => self.decode(Instruction::Nop),
            0x39 => self.decode(Instruction::DadSp(SP)),

            0x3A => self.decode(Instruction::Lda),
            0x3B => self.decode(Instruction::DcxSp(SP)),
            0x3C => self.decode(Instruction::Inr(A)),
            0x3D => self.decode(Instruction::Dcr(A)),
            0x3E => self.decode(Instruction::Mvi(A)),
            0x3F => self.decode(Instruction::Cmc),

            // MOV Instructions 0x40 - 0x7F
            0x40 => self.decode(Instruction::Mov(B, B)),
            0x41 => self.decode(Instruction::Mov(B, C)),
            0x42 => self.decode(Instruction::Mov(B, D)),
            0x43 => self.decode(Instruction::Mov(B, E)),
            0x44 => self.decode(Instruction::Mov(B, H)),
            0x45 => self.decode(Instruction::Mov(B, L)),
            0x46 => self.decode(Instruction::Mov(B, M)),
            0x47 => self.decode(Instruction::Mov(B, A)),

            0x48 => self.decode(Instruction::Mov(C, B)),
            0x49 => self.decode(Instruction::Mov(C, C)),
            0x4A => self.decode(Instruction::Mov(C, D)),
            0x4B => self.decode(Instruction::Mov(C, E)),
            0x4C => self.decode(Instruction::Mov(C, H)),
            0x4D => self.decode(Instruction::Mov(C, L)),
            0x4E => self.decode(Instruction::Mov(C, M)),
            0x4F => self.decode(Instruction::Mov(C, A)),

            0x50 => self.decode(Instruction::Mov(D, B)),
            0x51 => self.decode(Instruction::Mov(D, C)),
            0x52 => self.decode(Instruction::Mov(D, D)),
            0x53 => self.decode(Instruction::Mov(D, E)),
            0x54 => self.decode(Instruction::Mov(D, H)),
            0x55 => self.decode(Instruction::Mov(D, L)),
            0x56 => self.decode(Instruction::Mov(D, M)),
            0x57 => self.decode(Instruction::Mov(D, A)),

            0x58 => self.decode(Instruction::Mov(E, B)),
            0x59 => self.decode(Instruction::Mov(E, C)),
            0x5A => self.decode(Instruction::Mov(E, D)),
            0x5B => self.decode(Instruction::Mov(E, E)),
            0x5C => self.decode(Instruction::Mov(E, H)),
            0x5D => self.decode(Instruction::Mov(E, L)),
            0x5E => self.decode(Instruction::Mov(E, M)),
            0x5F => self.decode(Instruction::Mov(E, A)),

            0x60 => self.decode(Instruction::Mov(H, B)),
            0x61 => self.decode(Instruction::Mov(H, C)),
            0x62 => self.decode(Instruction::Mov(H, D)),
            0x63 => self.decode(Instruction::Mov(H, E)),
            0x64 => self.decode(Instruction::Mov(H, H)),
            0x65 => self.decode(Instruction::Mov(H, L)),
            0x66 => self.decode(Instruction::Mov(H, M)),
            0x67 => self.decode(Instruction::Mov(H, A)),

            0x68 => self.decode(Instruction::Mov(L, B)),
            0x69 => self.decode(Instruction::Mov(L, C)),
            0x6A => self.decode(Instruction::Mov(L, D)),
            0x6B => self.decode(Instruction::Mov(L, E)),
            0x6C => self.decode(Instruction::Mov(L, H)),
            0x6D => self.decode(Instruction::Mov(L, L)),
            0x6E => self.decode(Instruction::Mov(L, M)),
            0x6F => self.decode(Instruction::Mov(L, A)),

            0x70 => self.decode(Instruction::Mov(M, B)),
            0x71 => self.decode(Instruction::Mov(M, C)),
            0x72 => self.decode(Instruction::Mov(M, D)),
            0x73 => self.decode(Instruction::Mov(M, E)),
            0x74 => self.decode(Instruction::Mov(M, H)),
            0x75 => self.decode(Instruction::Mov(M, L)),
            0x76 => self.decode(Instruction::Hlt),
            0x77 => self.decode(Instruction::Mov(M, A)),

            0x78 => self.decode(Instruction::Mov(A, B)),
            0x79 => self.decode(Instruction::Mov(A, C)),
            0x7A => self.decode(Instruction::Mov(A, D)),
            0x7B => self.decode(Instruction::Mov(A, E)),
            0x7C => self.decode(Instruction::Mov(A, H)),
            0x7D => self.decode(Instruction::Mov(A, L)),
            0x7E => self.decode(Instruction::Mov(A, M)),
            0x7F => self.decode(Instruction::Mov(A, A)),

            // ADD Instructions
            0x80 => self.decode(Instruction::Add(B)),
            0x81 => self.decode(Instruction::Add(C)),
            0x82 => self.decode(Instruction::Add(D)),
            0x83 => self.decode(Instruction::Add(E)),
            0x84 => self.decode(Instruction::Add(H)),
            0x85 => self.decode(Instruction::Add(L)),
            0x86 => self.decode(Instruction::Add(M)),
            0x87 => self.decode(Instruction::Add(A)),

            0x88 => self.decode(Instruction::Adc(B)),
            0x89 => self.decode(Instruction::Adc(C)),
            0x8A => self.decode(Instruction::Adc(D)),
            0x8B => self.decode(Instruction::Adc(E)),
            0x8C => self.decode(Instruction::Adc(H)),
            0x8D => self.decode(Instruction::Adc(L)),
            0x8E => self.decode(Instruction::Adc(M)),
            0x8F => self.decode(Instruction::Adc(A)),

            // SUB Instructions
            0x90 => self.decode(Instruction::Sub(B)),
            0x91 => self.decode(Instruction::Sub(C)),
            0x92 => self.decode(Instruction::Sub(D)),
            0x93 => self.decode(Instruction::Sub(E)),
            0x94 => self.decode(Instruction::Sub(H)),
            0x95 => self.decode(Instruction::Sub(L)),
            0x96 => self.decode(Instruction::Sub(M)),
            0x97 => self.decode(Instruction::Sub(A)),

            0x98 => self.decode(Instruction::Sbb(B)),
            0x99 => self.decode(Instruction::Sbb(C)),
            0x9A => self.decode(Instruction::Sbb(D)),
            0x9B => self.decode(Instruction::Sbb(E)),
            0x9C => self.decode(Instruction::Sbb(H)),
            0x9D => self.decode(Instruction::Sbb(L)),
            0x9E => self.decode(Instruction::Sbb(M)),
            0x9F => self.decode(Instruction::Sbb(A)),

            // ANA & XRA Instructions
            0xA0 => self.decode(Instruction::Ana(B)),
            0xA1 => self.decode(Instruction::Ana(C)),
            0xA2 => self.decode(Instruction::Ana(D)),
            0xA3 => self.decode(Instruction::Ana(E)),
            0xA4 => self.decode(Instruction::Ana(H)),
            0xA5 => self.decode(Instruction::Ana(L)),
            0xA6 => self.decode(Instruction::Ana(M)),
            0xA7 => self.decode(Instruction::Ana(A)),

            0xA8 => self.decode(Instruction::Xra(B)),
            0xA9 => self.decode(Instruction::Xra(C)),
            0xAA => self.decode(Instruction::Xra(D)),
            0xAB => self.decode(Instruction::Xra(E)),
            0xAC => self.decode(Instruction::Xra(H)),
            0xAD => self.decode(Instruction::Xra(L)),
            0xAE => self.decode(Instruction::Xra(M)),
            0xAF => self.decode(Instruction::Xra(A)),

            // ORA Instructions  0xB(reg)
            0xB0 => self.decode(Instruction::Ora(B)),
            0xB1 => self.decode(Instruction::Ora(C)),
            0xB2 => self.decode(Instruction::Ora(D)),
            0xB3 => self.decode(Instruction::Ora(E)),
            0xB4 => self.decode(Instruction::Ora(H)),
            0xB5 => self.decode(Instruction::Ora(L)),
            0xB6 => self.decode(Instruction::Ora(M)),
            0xB7 => self.decode(Instruction::Ora(A)),

            // CMP
            0xB8 => self.decode(Instruction::Cmp(B)),
            0xB9 => self.decode(Instruction::Cmp(C)),
            0xBA => self.decode(Instruction::Cmp(D)),
            0xBB => self.decode(Instruction::Cmp(E)),
            0xBC => self.decode(Instruction::Cmp(H)),
            0xBD => self.decode(Instruction::Cmp(L)),
            0xBE => self.decode(Instruction::Cmp(M)),
            0xBF => self.decode(Instruction::Cmp(A)),

            0xC0 => self.decode(Instruction::Rnz),
            0xC1 => self.decode(Instruction::Pop(BC)),
            0xC2 => self.decode(Instruction::Jnz),
            0xC3 => self.decode(Instruction::Jmp),
            0xC4 => self.decode(Instruction::Cnz(0xC4)),
            0xC5 => self.decode(Instruction::Push(B)),
            0xC6 => self.decode(Instruction::Adi),
            0xC7 => self.decode(Instruction::Rst(0)),
            0xC8 => self.decode(Instruction::Rz),
            0xC9 => self.decode(Instruction::Ret),

            0xCA => self.decode(Instruction::Jz),
            0xCB => self.decode(Instruction::Jmp),
            0xCC => self.decode(Instruction::Cz(0xCC)),
            0xCD => self.decode(Instruction::Call(0xCD)),
            0xCE => self.decode(Instruction::Adi),
            0xCF => self.decode(Instruction::Rst(1)),

            0xD0 => self.decode(Instruction::Rnc),
            0xD1 => self.decode(Instruction::Pop(DE)),
            0xD2 => self.decode(Instruction::Jnc),
            0xD3 => self.decode(Instruction::Out),
            0xD4 => self.decode(Instruction::Call(0xD4)),
            0xD5 => self.decode(Instruction::Push(D)),
            0xD6 => self.decode(Instruction::Sui),
            0xD7 => self.decode(Instruction::Rst(2)),
            0xD8 => self.decode(Instruction::Rc),
            0xD9 => self.decode(Instruction::Ret),

            0xDA => self.decode(Instruction::Jc),
            0xDB => self.decode(Instruction::In),
            0xDC => self.decode(Instruction::Cc(0xDC)),
            0xDD => self.decode(Instruction::Call(0xDD)),
            0xDE => self.decode(Instruction::Sbi),
            0xDF => self.decode(Instruction::Rst(3)),

            0xE0 => self.decode(Instruction::Rpo),
            0xE1 => self.decode(Instruction::Pop(HL)),
            0xE2 => self.decode(Instruction::Jpo),
            0xE3 => self.decode(Instruction::Xthl),
            0xE4 => self.decode(Instruction::Cpo(0xE4)),
            0xE5 => self.decode(Instruction::Push(H)),
            0xE6 => self.decode(Instruction::Ani),
            0xE7 => self.decode(Instruction::Rst(4)),
            0xE8 => self.decode(Instruction::Rpe),
            0xE9 => self.decode(Instruction::Pchl),

            0xEA => self.decode(Instruction::Jpe),
            0xEB => self.decode(Instruction::Xchg),
            0xEC => self.decode(Instruction::Cpe(0xEC)),
            0xED => self.decode(Instruction::Call(0xED)),
            0xEE => self.decode(Instruction::Xri),
            0xEF => self.decode(Instruction::Rst(5)),

            0xF0 => self.decode(Instruction::Rp),
            0xF1 => self.decode(Instruction::PopPsw(A)),
            0xF2 => self.decode(Instruction::Jp),
            0xF3 => self.decode(Instruction::Di),
            0xF4 => self.decode(Instruction::Cp(0xF4)),
            0xF5 => self.decode(Instruction::PushPsw()),
            0xF6 => self.decode(Instruction::Ani),
            0xF7 => self.decode(Instruction::Rst(4)),
            0xF8 => self.decode(Instruction::Rm),
            0xF9 => self.decode(Instruction::Sphl),

            0xFA => self.decode(Instruction::Jm),
            0xFB => self.decode(Instruction::Ei),
            0xFC => self.decode(Instruction::Cm(0xFC)),
            0xFD => self.decode(Instruction::Call(0xFD)),
            0xFE => self.decode(Instruction::Cpi),
            0xFF => self.decode(Instruction::Rst(7)),

            _ => println!("Unknown opcode: {:#X}", self.registers.opcode),
        }
    }

    // Step one instruction
    pub fn step(&mut self, mut times: u8) {
        let addr = self.memory.read(self.registers.pc);
        for _ in 0..times {
            self.execute_instruction(addr);
            self.try_interrupt();
            self.registers.pc &= 0xFFFF;
            times += 1;
        }
    }

    pub fn reset(&mut self) {
        println!("Resetting emulator");

        self.registers.reg_a = 0;
        self.registers.reg_b = 0;
        self.registers.reg_c = 0;
        self.registers.reg_d = 0;
        self.registers.reg_e = 0;
        self.registers.reg_h = 0;
        self.registers.reg_l = 0;

        self.registers.reg_bc = 0;
        self.registers.reg_de = 0;
        self.registers.reg_hl = 0;

        // Reset flag conditions
        self.registers.sign = false;
        self.registers.zero = false;
        self.registers.parity = false;
        self.registers.carry = false;
        self.registers.half_carry = false;
        self.registers.reg_psw = 0;

        self.adv_pc(1);
    }

    fn hlt(&mut self) {
        self.adv_cycles(7);
        println!("Halting CPU");
        ::std::process::exit(0);
        //println!("HALT: Should wait for interrupt to happen.. This is not implemented");

    }
    fn half_carry_add(&self, mut value: u16) -> u16 {
        let mut add = [0, 0, 1, 0, 1, 0, 1, 1];
        let a = (self.registers.reg_a & 0xFF) as u16;
        // Immediate value
        value = value & 0xFF;
        // u16 word value (allow wrapping)
        let word: u16 = a.wrapping_sub(value).wrapping_add(0x100) & 0xFF;
        let row: u16 = ((a & 0x88) >> 1) | ((value & 0x88) >> 2) | ((word & 0x88) >> 3);
        // Return half carry add value
        add[row as usize & 0x7]
    }

    fn half_carry_sub(&self, mut value: u16) -> u16 {
        let sub = [0, 1, 1, 1, 0, 0, 0, 1];
        let a = (self.registers.reg_a & 0xFF) as u16;
        value = value & 0xFF;
        let word: u16 = a.wrapping_sub(value).wrapping_add(0x100) & 0xFF;
        let row: u16 = (a & 0x88) >> 1 | ((value & 0x88) >> 2) | ((word & 0x88) >> 3);
        // Return half carry sub value
        sub[row as usize & 0x7]
    }
    fn parity(&self, mut b: u8) -> bool {
        let mut bits = 0;
        let mut result: bool = false;
        for i in 0..8 {
            if b & 1 << i == 1 {
                bits += 1;
            }
            if bits & 1 == 0 {
                result = false;
            } else if bits & 1 == 1 {
                result = true;
            } else {
                result = false;
            }
        }
        result
    }
    fn emulate_interrupt(&mut self) {
        if self.registers.interrupt {
            let ret = self.registers.pc;
            self.memory.memory[self.registers.sp as usize - 1] = ((ret as u16 >> 8) & 0xFF as u16) as u8;
            self.memory.memory[self.registers.sp as usize - 2] = ((ret as u16 >> 8) & 0xFF as u16) as u8;
            self.registers.sp -= 2;

            self.registers.pc = self.registers.interrupt_addr as u16;
        }

    }
    pub fn try_interrupt(&mut self) {
        if self.registers.cycles < 16_667 {
            return;
        }
        if self.registers.interrupt_addr == 0x10 && self.registers.cycles > 16_667 {
            self.registers.cycles -= 16_667;
            self.registers.interrupt_addr = 0x08;

            // Call Reset with interrupt code
            if self.registers.interrupt {
                    println!("Interrupt enabled");
                // Emulate interrupt
                self.emulate_interrupt();
                // self.rst(8);
                self.registers.interrupt = false;
            }
        } else if self.registers.interrupt_addr == 0x08 && self.registers.cycles > 16_667 {
            self.registers.cycles -= 16_667;
            self.registers.interrupt_addr = 0x10;


            if self.registers.interrupt {
                println!("Interrupt enabled");
                // self.rst(16);
                self.registers.interrupt = false;
            }
        }
    }
}
