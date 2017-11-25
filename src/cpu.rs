use std::fmt;
use opcode::{Register, RegisterPair};
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

// #[derive(Copy, Clone)]
pub struct Registers {
    pub opcode: u8,
    pub current_instruction: String,
    pub breakpoint: bool,
    pub debug: bool,

    pub pc: u16,
    pub prev_pc: u16,
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

    // Status Register (Flags)
    pub sign: bool,
    pub zero: bool, // If the zero bit is one = true
    pub parity: bool,

    pub carry: bool,
    pub half_carry: bool,

    pub cycles: usize,
    pub interrupt: bool,
    pub interrupt_addr: u8,

    // I/O Read port
    port_0_in: u8, // Input port 0
    port_1_in: u8, // Input port 1
    port_2_in: u8, // Input port 2
    port_3_in: u8, // Bit shift register read / shift in

    // I/O Write port
    port_2_out: u8,
    // Shift amount (3 bits)
    port_3_out: u8,
    // Sound bits
    port_4_out_high: u8, // Shift data port high
    port_4_out_low: u8,
    // Shift data port low
    port_5_out: u8,
    // Sound bits
    port_6_out: u8,
    // Watchdog (read or write to reset)
}

impl Registers {
    pub fn new() -> Registers {

        Registers {
            opcode: 0,
            current_instruction: String::new(),
            debug: false,
            breakpoint: false,

            pc: 0,
            prev_pc: 0,
            sp: 0,

            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            reg_m: 0,

            sign: false,
            zero: false,
            parity: false,

            carry: false,
            half_carry: false,

            cycles: 0,
            interrupt: false,
            interrupt_addr: 0x08,

            port_0_in: 0x0E,
            port_1_in: 0x08, // 0xFE, (coin value)
            port_2_in: 0x00,
            port_3_in: 0,

            port_2_out: 0,
            port_3_out: 0,
            port_4_out_high: 0,
            port_4_out_low: 0,
            port_5_out: 0,
            port_6_out: 0,
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //           I   O   PC  Cycles A   BC  DE  HL  SP    S     Z     P     C    AC    Intr
        writeln!(f, "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}    \t{}\t{}\t{}\t{}\t{}\t{}\t",
                 "Instruction", "Opcode", "PC", "A", "BC", "DE", "HL", "SP", "S   ", "Z   ", "P   ", "C   ", "AC   ", "I   ");
        write!(
            f, // S   Z   P   C   AC  Interrupt
            "{}\t{:04X}\t{:04X}\t{:02X}\t{:02X}{:02X}\t{:02X}{:02X}\t{:02X}{:02X}\t{:04X}\t{}\t{}\t{}\t{}\t\t{}\t{}",
            self.current_instruction,
            self.opcode,
            self.prev_pc,
            self.reg_a,
            self.reg_b,
            self.reg_c,
            self.reg_d,
            self.reg_e,
            self.reg_h,
            self.reg_l,
            self.sp,
            self.sign as u8,
            self.zero as u8,
            self.parity as u8,
            self.carry as u8,
            self.half_carry as u8,
            self.interrupt as u8
        )
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
        self.registers.sp = byte;
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

    fn write_reg(&mut self, reg: Register, value: u8) {
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

    fn adv_pc(&mut self, t: u16) {
        self.registers.prev_pc = self.registers.pc;
        self.registers.pc = self.registers.pc.wrapping_add(t);
    }

    fn adv_cycles(&mut self, t: usize) {
        self.registers.cycles += t;
    }

    fn adc(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };
        let result = (self.registers.reg_a as u16).wrapping_add(value as u16).wrapping_add(self.registers.carry as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a & 0x0F) + (value as u8 & 0x0F) + (self.registers.carry as u8) > 0x0F;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Add Immediate to Accumulator with Carry
    fn aci(&mut self) {
        let value = self.memory.read(self.registers.pc + 1) as u16;

        // Add immediate with accumulator + carry flag value
        let reg_a = self.registers.reg_a;
        let carry = self.registers.carry as u8;
        let result = (value).wrapping_add(reg_a as u16).wrapping_add(carry as u16);


        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (reg_a & 0x0F) + (value as u8 & 0x0F) + (carry & 0x0F) > 0x0F;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }


    fn add(&mut self, reg: Register) {
        let value = match reg {

            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };

        let result = (self.registers.reg_a as u16).wrapping_add(value as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as u8 & 0x0F) + (value as u8 & 0x0F) > 0x0F;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Add Immediate to Accumulator
    fn adi(&mut self) {
        // Read next byte of immediate data (low).
        let value = self.memory.read(self.registers.pc + 1) as u16;
        let result = (value).wrapping_add(self.registers.reg_a as u16);

        // Set CPU flags with new accumulator values
        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.half_carry = (self.registers.reg_a as u8 & 0x0F) + (value as u8 & 0x0F) > 0x0F;
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;
        self.adv_cycles(7);
        self.adv_pc(2);
    }



    fn ana(&mut self, reg: Register) {
        // Check if the 4th bit is set on all registers
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };
        // And value with accumulator
        let result = self.registers.reg_a as u16 & value as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = ((self.registers.reg_a | value) & 0x08) != 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = false;
        self.registers.reg_a = result as u8;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ani(&mut self) {
        // The byte of immediate data is ANDed with the contents of the accumulator
        // The Carry bit is reset to zero.
        // Set half carry if the accumulator or opcode and the lower 4 bits are 1.

        let value = self.memory.read(self.registers.pc + 1);
        let result = self.registers.reg_a as u16 & value as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = ((self.registers.reg_a | value as u8) & 0x08) != 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = false;
        self.registers.reg_a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn jmp(&mut self) {
        self.registers.prev_pc = self.registers.pc;
        self.registers.pc = self.memory.read_imm(self.registers.pc);
        self.adv_cycles(10);
    }

    // Jump if carry
    fn jc(&mut self) {
        if self.registers.carry {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if no carry
    fn jnc(&mut self) {
        if !self.registers.carry {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Zero (If zero bit is 1)
    fn jz(&mut self) {
        if self.registers.zero {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Not Zero (if zero bit is 0 jump)
    fn jnz(&mut self) {
        if !self.registers.zero {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Minus (If sign bit is one)
    fn jm(&mut self) {
        if self.registers.sign {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump if Positive (If sign bit is zero)
    fn jp(&mut self) {
        if !self.registers.sign {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity even (If parity bit is 1)
    fn jpe(&mut self) {
        if self.registers.parity {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // If parity odd (If parity bit is 0)
    fn jpo(&mut self) {
        if !self.registers.parity {
            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = self.memory.read_imm(self.registers.pc);
        } else {
            self.adv_pc(3);
        }
        self.adv_cycles(10);
    }

    // Jump to address in H:L
    fn pchl(&mut self) {
        self.adv_cycles(5);
        self.registers.prev_pc = self.registers.pc;
        self.registers.pc = self.get_hl();
    }

    // Load Register Pair Immediate
    // LXI H, 2000H (2000H is stored in HL & acts as as memory pointer)
    fn lxi(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                self.registers.reg_b = self.memory.read_hb(self.registers.pc);
                self.registers.reg_c = self.memory.read_lb(self.registers.pc);
            }

            RegisterPair::DE => {
                self.registers.reg_d = self.memory.read_hb(self.registers.pc);
                self.registers.reg_e = self.memory.read_lb(self.registers.pc);
            }

            RegisterPair::HL => {
                self.memory.read_hb(self.registers.pc);
                self.registers.reg_h = self.memory.read_hb(self.registers.pc);
                self.registers.reg_l = self.memory.read_lb(self.registers.pc);
            }
            RegisterPair::SP => self.registers.sp = self.memory.read_word(self.registers.pc + 1),
        }
        self.adv_cycles(10);
        self.adv_pc(3);
    }

    // Store Accumulator direct
    fn sta(&mut self) {
        let imm = self.memory.read_imm(self.registers.pc);
        // Store value of the accumulator in in memory
        self.memory.memory[imm as usize] = self.registers.reg_a;

        self.adv_cycles(13);
        self.adv_pc(3);
    }

    fn call(&mut self, opcode: u16) {
        // CALL instructions occupy three bytes. (See page 34 of the 8080 Programmers Manual)
        // CALL is just like JMP but also pushes a return address to stack.
        let ret: u16 = self.registers.pc + 3;
        match opcode {
            0xCC | 0xCD | 0xC4 | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFD | 0xFC => {
                // High order byte
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = (ret >> 8) as u8;
                // Low order byte
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = ret as u8;

                // Push return address to stack
                self.registers.sp = self.registers.sp.wrapping_sub(2);
            }
            _ => println!("Unknown call opcode: {:04X}", self.registers.opcode),
        };

        self.registers.prev_pc = self.registers.pc;

        self.registers.pc = self.memory.read_imm(self.registers.pc);
        self.adv_cycles(17);
    }

    // Call if Minus (if sign bit is 1, indicating a negative result)
    fn cm(&mut self, addr: u16) {
        if self.registers.sign {
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cc(&mut self, addr: u16) {
        if self.registers.carry {
            self.call(addr)
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
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
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    fn cma(&mut self) {
        self.registers.reg_a ^= 0xFF;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn cmc(&mut self) {
        self.registers.carry = !self.registers.carry;
        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn cmp(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };
        let result = (self.registers.reg_a as u16).wrapping_sub(value as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Compare Immediate with Accumulator
    fn cpi(&mut self) {
        // Fetch byte out of memory which we will use to compare & set flags with.
        let value = self.memory.read(self.registers.pc + 1) as u16;
        // Compare the result of the accumulator with the immediate address.
        let result = (self.registers.reg_a as u16).wrapping_sub(value as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // Call if Parity Even
    fn cpe(&mut self, addr: u16) {
        if self.registers.parity {
            self.registers.prev_pc = self.registers.pc;
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }
    // Call if Parity Odd
    fn cpo(&mut self, addr: u16) {
        if !self.registers.parity {
            self.registers.prev_pc = self.registers.pc;
            self.call(addr);
        } else {
            self.adv_cycles(11);
            self.adv_pc(3);
        }
    }

    // CALL if plus (if sign bit is zero, indicating a positive result)
    fn cp(&mut self, addr: u16) {
        if !self.registers.sign {
            self.registers.prev_pc = self.registers.pc;
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
                let value = (self.registers.reg_b as u32) << 8 | (self.registers.reg_c as u32);
                let result = (self.get_hl() as u32).wrapping_add(value as u32);

                self.registers.reg_h = (result >> 8) as u8;
                self.registers.reg_l = result as u8;
                self.registers.carry = (result & 0x10_000) != 0;
            }
            RegisterPair::DE => {
                let value = (self.registers.reg_d as u32) << 8 | (self.registers.reg_e as u32);
                let result = (self.get_hl() as u32).wrapping_add(value as u32);

                self.registers.reg_h = (result >> 8) as u8;
                self.registers.reg_l = result as u8;
                self.registers.carry = (result & 0x10_000) != 0;
            }
            RegisterPair::HL => {
                let mut value = (self.registers.reg_h as u32) << 8 | (self.registers.reg_l as u32);
                let result = (self.get_hl() as u32).wrapping_add(value as u32);

                self.registers.reg_h = (result >> 8) as u8;
                self.registers.reg_l = result as u8;
                self.registers.carry = (result & 0x10_000) != 0;
            }
            RegisterPair::SP => {
                let mut value = self.registers.sp as u32;
                let result = (self.get_hl() as u32).wrapping_add(value);

                self.registers.reg_h = (result >> 8) as u8;
                self.registers.reg_l = result as u8;
                self.registers.carry = (result & 0x10_000) != 0;
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

        let value = match reg {
            Register::A => {
                self.registers.reg_a = self.registers.reg_a.wrapping_sub(1);
                self.registers.reg_a
            }
            Register::B => {
                self.registers.reg_b = self.registers.reg_b.wrapping_sub(1);
                self.registers.reg_b
            }
            Register::C => {
                self.registers.reg_c = self.registers.reg_c.wrapping_sub(1);
                self.registers.reg_c
            }
            Register::D => {
                self.registers.reg_d = self.registers.reg_d.wrapping_sub(1);
                self.registers.reg_d
            }
            Register::E => {
                self.registers.reg_e = self.registers.reg_e.wrapping_sub(1);
                self.registers.reg_e
            }
            Register::H => {
                self.registers.reg_h = self.registers.reg_h.wrapping_sub(1);
                self.registers.reg_h
            }
            Register::L => {
                self.registers.reg_l = self.registers.reg_l.wrapping_sub(1);
                self.registers.reg_l
            }
            Register::M => {
                self.adv_cycles(5);
                let hl = self.get_hl();
                self.memory.memory[hl as usize] = self.memory.memory[hl as usize].wrapping_sub(1);
                self.memory.memory[hl as usize]
            }
        };
        self.registers.sign = (value & 0x80) != 0;
        self.registers.zero = (value & 0xFF) == 0;
        self.registers.half_carry = !((value & 0x0F) == 0x0F);
        self.registers.parity = self.parity(value as u8);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn dcx(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                let value = self.get_bc().wrapping_sub(1);
                self.registers.reg_b = (value >> 8) as u8;
                self.registers.reg_c = value as u8;
            }
            RegisterPair::DE => {
                let value = self.get_de().wrapping_sub(1);
                self.registers.reg_d = (value >> 8) as u8;
                self.registers.reg_e = value as u8;
            }
            RegisterPair::HL => {
                let value = self.get_hl().wrapping_sub(1);
                self.registers.reg_h = (value >> 8) as u8;
                self.registers.reg_l = value as u8;
            }
            RegisterPair::SP => self.registers.sp = self.registers.sp.wrapping_sub(1),
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // Double precision add
    fn daa(&mut self) {
        let mut add = if self.registers.half_carry || self.registers.reg_a & 0x0F > 9 { 0x06 } else { 0 };

        if self.registers.carry || (self.registers.reg_a >> 4) > 9 || (self.registers.reg_a >> 4) >= 9 && self.registers.reg_a & 0x0F > 9 {
            add |= 0x60;
            self.registers.carry = true;
        }

        let result = (self.registers.reg_a as u16).wrapping_add(add as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as u8 & 0x0F) + (add as u8 & 0x0F) > 0x0F;
        self.registers.parity = self.parity(result as u8);

        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn di(&mut self) {
        // if self.registers.debug { println!("Disabling interrupts"); }
        self.registers.interrupt = false;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn ei(&mut self) {
        // if self.registers.debug { println!("Enabling interrupts"); }
        self.registers.interrupt = true;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {
        // The contents of the accumulator are rotated one bit position to the left.
        // The high-order bit of the accumulator replaces the carry bit while the carry bit
        // replaces the high-order bit of the accumulator
        // let carry = self.registers.carry as u8;
        // self.registers.reg_a = (self.registers.reg_a << 1) | carry;
        // self.registers.carry = (self.registers.reg_a & 0x80) != 0;

        let carry = (self.registers.reg_a & 0x80) != 0;
        self.registers.reg_a = (self.registers.reg_a << 1) | (self.registers.carry as u8);
        self.registers.carry = carry;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        // If last bit is 1 bit shift one up so that the accumulator is 1
        let carry = (self.registers.reg_a & 1) != 0;
        self.registers.reg_a = (self.registers.reg_a >> 1) | ((self.registers.carry as u8) << 7);
        self.registers.carry = carry;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 higher bits are 1 we set the carry flag.
        self.registers.reg_a = self.registers.reg_a.rotate_left(1);
        self.registers.carry = (self.registers.reg_a & 1)  != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn rrc(&mut self) {
        // The Carry bit is set equal to the low-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        self.registers.reg_a = self.registers.reg_a.rotate_right(1);
        self.registers.carry = (self.registers.reg_a & 0x80) != 0;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Return if no carry
    fn rnc(&mut self) {
        if !self.registers.carry {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Parity Even
    fn rpe(&mut self) {
        if self.registers.parity {
            self.adv_cycles(1);
            self.ret()
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Parity Odd
    fn rpo(&mut self) {
        if !self.registers.parity {
            self.adv_cycles(1);
            self.ret()
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if Carry
    fn rc(&mut self) {
        // If Carry flag is set, return from subroutine
        if self.registers.carry {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    fn rnz(&mut self) {
        if !self.registers.zero {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if minus
    fn rm(&mut self) {
        if self.registers.sign {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    // Return if positive (if sign bit is 0)
    fn rp(&mut self) {
        if !self.registers.sign {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }
    fn rz(&mut self) {
        if self.registers.zero {
            self.adv_cycles(1);
            self.ret();
        } else {
            self.adv_cycles(5);
            self.adv_pc(1);
        }
    }

    // Move Immediate Data
    fn mvi(&mut self, reg: Register) {
        // The MVI instruction uses a 8-bit data quantity, as opposed to
        // LXI which uses a 16-bit data quantity.
        let value = self.memory.read(self.registers.pc + 1) as u8;

        match reg {
            Register::A => self.registers.reg_a = value,
            Register::B => self.registers.reg_b = value,
            Register::C => self.registers.reg_c = value,
            Register::D => self.registers.reg_d = value,
            Register::E => self.registers.reg_e = value,
            Register::H => self.registers.reg_h = value,
            Register::L => self.registers.reg_l = value,
            Register::M => {
                self.adv_cycles(3);
                let hl = self.get_hl();
                self.memory.memory[hl as usize] = value;
            }
        }
        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // LDA Load Accumulator direct
    fn lda(&mut self) {
        let imm = self.memory.read_imm(self.registers.pc);
        self.registers.reg_a = self.memory.memory[imm as usize];
        self.adv_cycles(13);
        self.adv_pc(3);
    }


    // LDAX(Load accumulator indirect)
    fn ldax(&mut self, reg: RegisterPair) {
        // The contents of the designated register pair point to a memory location.
        // This instruction copies the contents of that memory location into the
        // accumulator. The contents of either the register pair or the
        // memory location are not altered.

        match reg {
            RegisterPair::BC => {
                let bc = self.get_bc() as usize;
                self.registers.reg_a = self.memory.memory[bc];
            }
            RegisterPair::DE => {
                let de = self.get_de() as usize;
                self.registers.reg_a = self.memory.memory[de];
            }
            _ => eprintln!("LDAX on invalid register"),
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
        let imm = self.memory.read_imm(self.registers.pc) as usize;

        self.registers.reg_h = self.memory.memory[imm + 1];
        self.registers.reg_l = self.memory.memory[imm];

        self.adv_cycles(16);
        self.adv_pc(3);
    }

    fn input(&mut self) {
        let port = self.memory.read(self.registers.pc + 1);

        let mut result: u16 = 0;
        match port {
            0 => result = self.registers.port_0_in as u16,
            1 => {
                result = self.registers.port_1_in as u16;
                self.registers.port_1_in &= 0xFE;
            },
            2 => result = (self.registers.port_2_in as u16) & 0x8F | (self.registers.port_2_in as u16) & 0x70,
            3 => result = ((self.registers.port_4_out_high as u16) << 8) |
                (self.registers.port_4_out_low as u16) << ((self.registers.port_2_out as u16) >> 8) & 0xFF,
            _ => eprintln!("Input port {}, not implemented", port),
        };

        if self.registers.debug {
            println!("Input port: {}, Result: {:04X}", port, result);
        }
        self.registers.reg_a = result as u8;
        self.adv_cycles(10);
        self.adv_pc(2);
    }

    fn inr(&mut self, reg: Register) {
        let value = match reg {
            Register::A => {
                self.registers.reg_a = self.registers.reg_a.wrapping_add(1);
                self.registers.reg_a
            },
            Register::B => {
                self.registers.reg_b = self.registers.reg_b.wrapping_add(1);
                self.registers.reg_b
            }
            Register::C => {
                self.registers.reg_c = self.registers.reg_c.wrapping_add(1);
                self.registers.reg_c
            }
            Register::D => {
                self.registers.reg_d = self.registers.reg_d.wrapping_add(1);
                self.registers.reg_d
            }
            Register::E => {
                self.registers.reg_e = self.registers.reg_e.wrapping_add(1);
                self.registers.reg_e
            }
            Register::H => {
                self.registers.reg_h = self.registers.reg_h.wrapping_add(1);
                self.registers.reg_h
            }
            Register::L => {
                self.registers.reg_l = self.registers.reg_l.wrapping_add(1);
                self.registers.reg_l

            }
            Register::M => {
                self.adv_cycles(5);
                let hl = self.get_hl() as usize;
                self.memory.memory[hl] = self.memory.memory[hl].wrapping_add(1);
                self.memory.memory[hl]
            }
        };

        self.registers.sign = (value & 0x80) != 0;
        self.registers.zero = (value & 0xFF) == 0;
        self.registers.half_carry = (value & 0x0F) == 0x00;
        self.registers.parity = self.parity(value as u8);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn inx(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                let value = self.get_bc().wrapping_add(1);
                self.registers.reg_b = (value >> 8) as u8;
                self.registers.reg_c = (value) as u8;

            }

            RegisterPair::DE => {
                let value = self.get_de().wrapping_add(1);
                self.registers.reg_d = (value >> 8) as u8;
                self.registers.reg_e = (value) as u8;
            }

            RegisterPair::HL => {
                let value = self.get_hl().wrapping_add(1);
                self.registers.reg_h = (value >> 8) as u8;
                self.registers.reg_l = (value) as u8;
            }
            RegisterPair::SP => {
                self.registers.sp = self.registers.sp.wrapping_add(1);
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
        let psw =
            if self.registers.sign { 0x80 } else { 0x0 } |
            if self.registers.zero { 0x40 } else { 0x0 } |
            if self.registers.parity { 0x04 } else { 0x0 } |
            if self.registers.half_carry { 0x10 } else { 0x0 } | 0x02 |
            if self.registers.carry { 0x01 } else { 0x0 };

        self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_a;
        self.memory.memory[self.registers.sp as usize - 2] = psw;
        self.registers.sp = self.registers.sp.wrapping_sub(2);

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Store the contents of the accumulator addressed by registers B, C
    // or by registers D and E.
    fn stax(&mut self, reg: RegisterPair) {
        let reg_a = self.registers.reg_a;
        match reg {
            RegisterPair::BC => self.memory.memory[(self.registers.reg_b as usize) << 8 | self.registers.reg_c as usize] = reg_a,
            RegisterPair::DE => self.memory.memory[(self.registers.reg_d as usize) << 8 | self.registers.reg_e as usize] = reg_a,
            RegisterPair::HL => self.memory.memory[(self.registers.reg_h as usize) << 8 | self.registers.reg_l as usize] = reg_a,
            RegisterPair::SP => eprintln!("STAX should not run on SP register"),
        }
        self.adv_cycles(7);
        self.adv_pc(1);
    }

    // SBB Subtract Register or Memory from Accumulator with borrow
    fn sbb(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };

        let result = (self.registers.reg_a as u16).wrapping_sub(value as u16).wrapping_sub(self.registers.carry as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) - (self.registers.carry as i8) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    // Subtract Immediate with Borrow
    fn sbi(&mut self) {
        let imm = self.memory.read(self.registers.pc + 1);
        let value = imm + self.registers.carry as u16;
        let result = (self.registers.reg_a as u16).wrapping_sub(value);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // SUB Subtract Register or Memory From Accumulator
    fn sub(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };
        let result = (self.registers.reg_a as u16).wrapping_sub(value as u16);

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // SUI Subtract Immediate From Accumulator
    fn sui(&mut self) {
        let value = self.memory.read(self.registers.pc + 1);
        let result = (self.registers.reg_a as u16).wrapping_sub(value) as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = (self.registers.reg_a as i8 & 0x0F) - (value as i8 & 0x0F) >= 0;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.reg_a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    // Set Carry (set carry bit to 1)
    fn stc(&mut self) {
        self.registers.carry = true;
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    fn xra(&mut self, reg: Register) {
        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };

        let result = self.registers.reg_a as u16 ^ value as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = false;
        self.registers.carry = false;
        self.registers.parity = self.parity(result as u8);
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // XRI Exclusive-Or Immediate with Accumulator
    fn xri(&mut self) {
        let imm = self.memory.read(self.registers.pc + 1);
        let result = self.registers.reg_a ^ imm as u8;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = false;
        self.registers.parity = self.parity(result);
        self.registers.carry = false;
        self.registers.reg_a = result;

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn xchg(&mut self) {
        use std::mem;

        mem::swap(&mut self.registers.reg_h, &mut self.registers.reg_d);
        mem::swap(&mut self.registers.reg_l, &mut self.registers.reg_e);

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn xthl(&mut self) {

        // Swap H:L with top word on stack
        let old_h = self.registers.reg_h;
        let old_l = self.registers.reg_l;
        let new_h = self.memory.memory[self.registers.sp as usize + 1];
        let new_l = self.memory.memory[self.registers.sp as usize];

        self.registers.reg_h = new_h;
        self.registers.reg_l = new_l;

        // Write old HL values to memory
        self.memory.memory[self.registers.sp as usize + 1] = old_h;
        self.memory.memory[self.registers.sp as usize] = old_l;
        self.adv_cycles(18);
        self.adv_pc(1);
    }


    fn pop(&mut self, reg: RegisterPair) {
        match reg {
            RegisterPair::BC => {
                self.registers.reg_c = self.memory.memory[self.registers.sp as usize];
                self.registers.reg_b = self.memory.memory[self.registers.sp as usize + 1];
            }

            RegisterPair::DE => {
                self.registers.reg_e = self.memory.memory[self.registers.sp as usize];
                self.registers.reg_d = self.memory.memory[self.registers.sp as usize + 1];
            }

            RegisterPair::HL => {
                self.registers.reg_l = self.memory.memory[self.registers.sp as usize];
                self.registers.reg_h = self.memory.memory[self.registers.sp as usize + 1];
            }
            RegisterPair::SP => {}
        }
        self.registers.sp = self.registers.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_psw(&mut self) {
        let sp = self.registers.sp as usize;
        self.registers.sign = (self.memory.memory[sp] & 0x80) != 0;
        self.registers.zero = (self.memory.memory[sp] & 0x40) != 0;
        self.registers.parity = (self.memory.memory[sp] & 0x04) != 0;
        self.registers.half_carry = (self.memory.memory[sp] & 0x10) != 0;
        self.registers.carry = (self.memory.memory[sp] & 0x01) != 0;

        // self.registers.reg_a = self.memory.read_imm(sp as u16) as u8;
        self.registers.reg_a = self.memory.read(sp as u16 + 1 ) as u8;
        self.registers.sp = sp.wrapping_add(2) as u16;

        self.adv_cycles(10);
        self.adv_pc(1);
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.memory.read_imm(self.registers.sp);
        if self.registers.debug {
            println!("Popping stack. SP value: {:04X}", sp);
        }
        self.registers.sp += 2;
        sp
    }

    fn ret(&mut self) {
        let low = self.memory.memory[self.registers.sp as usize];
        let high = self.memory.memory[self.registers.sp as usize + 1];
        let mut ret: u16 = (high as u16) << 8 | (low as u16);

        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.adv_cycles(10);

        // Set program counter for debug output
        self.registers.prev_pc = self.registers.pc;
        // println!("Returning to {:04X}", ret);
        self.registers.pc = ret as u16;
    }

    fn output(&mut self) {
        let port = self.memory.read(self.registers.pc + 1);
        println!("Output is running");
        match port {
            // Sets the offset size for shift register
            0x02 => {
                self.registers.port_2_out = self.registers.reg_a & 0x7;
                println!("Out port 2: {:04X}", self.registers.port_2_out);
            }
            // Sound port
            0x03 => {
                self.registers.port_3_out = self.registers.reg_a;
                println!("Output SND Port 3: {:04X}", self.registers.port_3_out);
            }

            // Sets shift register values
            0x04 => {
                self.registers.port_4_out_low = self.registers.port_4_out_high;
                self.registers.port_4_out_high = self.registers.reg_a;
                println!("Setting shift register values: High:{:04X}, Low:{:04X}",
                         self.registers.port_4_out_high,
                         self.registers.port_4_out_low
                );
            }
            // Sound port
            0x05 => self.registers.port_5_out = self.registers.reg_a,
            // Watchdog port
            0x06 => {
                self.registers.port_6_out = self.registers.reg_a;
                println!("Watchdog, value: {:04X}", self.registers.port_6_out);
            }
            _ => println!("Port: {:04X}, does not match implementation", port),
        }
        self.adv_cycles(10);
        self.adv_pc(2);
    }
    fn ora(&mut self, reg: Register) {

        let value = match reg {
            Register::A => self.registers.reg_a,
            Register::B => self.registers.reg_b,
            Register::C => self.registers.reg_c,
            Register::D => self.registers.reg_d,
            Register::E => self.registers.reg_e,
            Register::H => self.registers.reg_h,
            Register::L => self.registers.reg_l,
            Register::M => {
                self.adv_cycles(3);
                self.memory.memory[self.get_hl() as usize]
            }
        };

        let result = self.registers.reg_a as u16 | value as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = false;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = false;
        self.registers.reg_a = result as u8;

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // Or Immediate with Accumulator
    fn ori(&mut self) {
         let result = self.registers.reg_a as u16 | self.memory.read(self.registers.pc + 1) as u16;

        self.registers.sign = (result & 0x80) != 0;
        self.registers.zero = (result & 0xFF) == 0;
        self.registers.half_carry = false;
        self.registers.parity = self.parity(result as u8);
        self.registers.carry = false;
        self.registers.reg_a = result as u8;

        self.adv_cycles(7);
        self.adv_pc(2);
    }
    fn mov(&mut self, dst: Register, src: Register) {
        let value = self.read_reg(src);
        let addr: u16 = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);

        match dst {
            Register::A => {
                if src == Register::M {
                    self.registers.reg_a = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value)
                }
            }
            Register::B => {
                if src == Register::M {
                    let hl = self.get_hl();
                    self.registers.reg_b = self.memory.read_byte(hl);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::C => {
                if src == Register::M {
                    self.registers.reg_c = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::D => {
                if src == Register::M {
                    self.registers.reg_d = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::E => {
                if src == Register::M {
                    self.registers.reg_e = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::H => {
                if src == Register::M {
                    self.registers.reg_h = self.memory.read_byte(addr);
                    self.adv_cycles(2);
                } else {
                    self.write_reg(dst, value);
                }
            }
            Register::L => {
                if src == Register::M {
                    self.registers.reg_l = self.memory.read_byte(addr);
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
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // RESET (used for interrupt jump / calls)
    pub fn rst(&mut self, value: u8) {
        // Address to return to after interrupt is finished.
        let ret = self.registers.pc + 1;


        self.memory.memory[(self.registers.sp as usize).wrapping_sub(1)] = (ret >> 8) as u8;
        self.memory.memory[(self.registers.sp as usize).wrapping_sub(2)] = ret as u8;

        self.registers.prev_pc = self.registers.pc;

        match value {
            0 => self.registers.pc = 0x0000,
            1 => self.registers.pc = 0x0008,
            2 => self.registers.pc = 0x0010,
            3 => self.registers.pc = 0x0018,
            4 => self.registers.pc = 0x0020,
            5 => self.registers.pc = 0x0028,
            6 => self.registers.pc = 0x0030,
            7 => self.registers.pc = 0x0038,
            _ => println!("Couldn't match RST value, {:04X}", value),
        };

        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.adv_cycles(11);
    }

    fn sphl(&mut self) {
        self.registers.sp = self.get_hl();
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // Store H & L direct
    fn shld(&mut self) {
        let addr: u16 = self.memory.read_imm(self.registers.pc);
        self.memory.memory[addr as usize + 1] = self.registers.reg_h;
        self.memory.memory[addr as usize] = self.registers.reg_l;

        self.adv_cycles(16);
        self.adv_pc(3);
    }

    pub fn nop(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    pub fn execute_instruction(&mut self) {
        use self::Register::*;
        use self::RegisterPair::*;

        let opcode = self.memory.memory[self.registers.pc as usize];

        match opcode {
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => self.nop(),
            0x01 => self.lxi(BC),
            0x02 => self.stax(BC),
            0x03 => self.inx(BC),
            0x04 => self.inr(B),
            0x05 => self.dcr(B),
            0x06 => self.mvi(B),
            0x07 => self.rlc(),
            0x09 => self.dad(BC),

            0x0A => self.ldax(BC),
            0x0B => self.dcx(BC),
            0x0C => self.inr(C),
            0x0D => self.dcr(C),
            0x0E => self.mvi(C),
            0x0F => self.rrc(),

            0x11 => self.lxi(DE),
            0x12 => self.stax(DE),
            0x13 => self.inx(DE),
            0x14 => self.inr(D),
            0x15 => self.dcr(D),
            0x16 => self.mvi(D),
            0x17 => self.ral(),
            0x19 => self.dad(DE),

            0x1A => self.ldax(DE),
            0x1B => self.dcx(DE),
            0x1C => self.inr(E),
            0x1D => self.dcr(E),
            0x1E => self.mvi(E),
            0x1F => self.rar(),

            0x21 => self.lxi(HL),
            0x22 => self.shld(),
            0x23 => self.inx(HL),
            0x24 => self.inr(H),
            0x25 => self.dcr(H),
            0x26 => self.mvi(H),
            0x27 => self.daa(),
            0x29 => self.dad(HL),

            0x2A => self.lhld(),
            0x2B => self.dcx(HL),
            0x2C => self.inr(L),
            0x2D => self.dcr(L),
            0x2E => self.mvi(L),
            0x2F => self.cma(),

            0x31 => self.lxi(SP),
            0x32 => self.sta(),
            0x33 => self.inx(SP),
            0x34 => self.inr(M),
            0x35 => self.dcr(M),
            0x36 => self.mvi(M),
            0x37 => self.stc(),
            0x39 => self.dad(SP),

            0x3A => self.lda(),
            0x3B => self.dcx(SP),
            0x3C => self.inr(A),
            0x3D => self.dcr(A),
            0x3E => self.mvi(A),
            0x3F => self.cmc(),

            // MOV Instructions 0x40 - 0x7F
            0x40 => self.mov(B, B),
            0x41 => self.mov(B, C),
            0x42 => self.mov(B, D),
            0x43 => self.mov(B, E),
            0x44 => self.mov(B, H),
            0x45 => self.mov(B, L),
            0x46 => self.mov(B, M),
            0x47 => self.mov(B, A),

            0x48 => self.mov(C, B),
            0x49 => self.mov(C, C),
            0x4A => self.mov(C, D),
            0x4B => self.mov(C, E),
            0x4C => self.mov(C, H),
            0x4D => self.mov(C, L),
            0x4E => self.mov(C, M),
            0x4F => self.mov(C, A),

            0x50 => self.mov(D, B),
            0x51 => self.mov(D, C),
            0x52 => self.mov(D, D),
            0x53 => self.mov(D, E),
            0x54 => self.mov(D, H),
            0x55 => self.mov(D, L),
            0x56 => self.mov(D, M),
            0x57 => self.mov(D, A),

            0x58 => self.mov(E, B),
            0x59 => self.mov(E, C),
            0x5A => self.mov(E, D),
            0x5B => self.mov(E, E),
            0x5C => self.mov(E, H),
            0x5D => self.mov(E, L),
            0x5E => self.mov(E, M),
            0x5F => self.mov(E, A),

            0x60 => self.mov(H, B),
            0x61 => self.mov(H, C),
            0x62 => self.mov(H, D),
            0x63 => self.mov(H, E),
            0x64 => self.mov(H, H),
            0x65 => self.mov(H, L),
            0x66 => self.mov(H, M),
            0x67 => self.mov(H, A),

            0x68 => self.mov(L, B),
            0x69 => self.mov(L, C),
            0x6A => self.mov(L, D),
            0x6B => self.mov(L, E),
            0x6C => self.mov(L, H),
            0x6D => self.mov(L, L),
            0x6E => self.mov(L, M),
            0x6F => self.mov(L, A),

            0x70 => self.mov(M, B),
            0x71 => self.mov(M, C),
            0x72 => self.mov(M, D),
            0x73 => self.mov(M, E),
            0x74 => self.mov(M, H),
            0x75 => self.mov(M, L),

            0x76 => self.hlt(),
            0x77 => self.mov(M, A),

            0x78 => self.mov(A, B),
            0x79 => self.mov(A, C),
            0x7A => self.mov(A, D),
            0x7B => self.mov(A, E),
            0x7C => self.mov(A, H),
            0x7D => self.mov(A, L),
            0x7E => self.mov(A, M),
            0x7F => self.mov(A, A),

            // ADD Instructions
            0x80 => self.add(B),
            0x81 => self.add(C),
            0x82 => self.add(D),
            0x83 => self.add(E),
            0x84 => self.add(H),
            0x85 => self.add(L),
            0x86 => self.add(M),
            0x87 => self.add(A),

            0x88 => self.adc(B),
            0x89 => self.adc(C),
            0x8A => self.adc(D),
            0x8B => self.adc(E),
            0x8C => self.adc(H),
            0x8D => self.adc(L),
            0x8E => self.adc(M),
            0x8F => self.adc(A),

            // SUB Instructions
            0x90 => self.sub(B),
            0x91 => self.sub(C),
            0x92 => self.sub(D),
            0x93 => self.sub(E),
            0x94 => self.sub(H),
            0x95 => self.sub(L),
            0x96 => self.sub(M),
            0x97 => self.sub(A),

            0x98 => self.sbb(B),
            0x99 => self.sbb(C),
            0x9A => self.sbb(D),
            0x9B => self.sbb(E),
            0x9C => self.sbb(H),
            0x9D => self.sbb(L),
            0x9E => self.sbb(M),
            0x9F => self.sbb(A),

            // ANA
            0xA0 => self.ana(B),
            0xA1 => self.ana(C),
            0xA2 => self.ana(D),
            0xA3 => self.ana(E),
            0xA4 => self.ana(H),
            0xA5 => self.ana(L),
            0xA6 => self.ana(M),
            0xA7 => self.ana(A),

            // XRA
            0xA8 => self.xra(B),
            0xA9 => self.xra(C),
            0xAA => self.xra(D),
            0xAB => self.xra(E),
            0xAC => self.xra(H),
            0xAD => self.xra(L),
            0xAE => self.xra(M),
            0xAF => self.xra(A),

            // ORA Instructions  0xB(reg)
            0xB0 => self.ora(B),
            0xB1 => self.ora(C),
            0xB2 => self.ora(D),
            0xB3 => self.ora(E),
            0xB4 => self.ora(H),
            0xB5 => self.ora(L),
            0xB6 => self.ora(M),
            0xB7 => self.ora(A),

            // CMP
            0xB8 => self.cmp(B),
            0xB9 => self.cmp(C),
            0xBA => self.cmp(D),
            0xBB => self.cmp(E),
            0xBC => self.cmp(H),
            0xBD => self.cmp(L),
            0xBE => self.cmp(M),
            0xBF => self.cmp(A),

            0xC0 => self.rnz(),
            0xC1 => self.pop(BC),
            0xC2 => self.jnz(),
            0xC3 => self.jmp(),
            0xC4 => self.cnz(0xC4),
            0xC5 => self.push(B),
            0xC6 => self.adi(),
            0xC7 => self.rst(0),
            0xC8 => self.rz(),
            0xC9 => self.ret(),

            0xCA => self.jz(),
            0xCB => self.jmp(),
            0xCC => self.cz(0xCC),
            0xCD => self.call(0xCD),
            0xCE => self.aci(),
            0xCF => self.rst(1),

            0xD0 => self.rnc(),
            0xD1 => self.pop(DE),
            0xD2 => self.jnc(),
            0xD3 => self.output(),
            0xD4 => self.cnc(0xD4),
            0xD5 => self.push(D),
            0xD6 => self.sui(),
            0xD7 => self.rst(2),
            0xD8 => self.rc(),
            0xD9 => self.ret(),

            0xDA => self.jc(),
            0xDB => self.input(),
            0xDC => self.cc(0xDC),
            0xDD => self.call(0xDD),
            0xDE => self.sbi(),
            0xDF => self.rst(3),

            0xE0 => self.rpo(),
            0xE1 => self.pop(HL),
            0xE2 => self.jpo(),
            0xE3 => self.xthl(),
            0xE4 => self.cpo(0xE4),
            0xE5 => self.push(H),
            0xE6 => self.ani(),
            0xE7 => self.rst(4),
            0xE8 => self.rpe(),
            0xE9 => self.pchl(),

            0xEA => self.jpe(),
            0xEB => self.xchg(),
            0xEC => self.cpe(0xEC),
            0xED => self.call(0xED),
            0xEE => self.xri(),
            0xEF => self.rst(5),

            0xF0 => self.rp(),
            0xF1 => self.pop_psw(),
            0xF2 => self.jp(),
            0xF3 => self.di(),
            0xF4 => self.cp(0xF4),
            0xF5 => self.push_psw(),
            0xF6 => self.ori(),
            0xF7 => self.rst(4),
            0xF8 => self.rm(),
            0xF9 => self.sphl(),

            0xFA => self.jm(),
            0xFB => self.ei(),
            0xFC => self.cm(0xFC),
            0xFD => self.call(0xFD),
            0xFE => self.cpi(),
            0xFF => self.rst(7),

            _ => println!("Unknown opcode: {:04X}", self.registers.opcode),
        }
        self.registers.opcode = opcode;
        use opcode::Opcode;

        // Lookup opcode & get instruction name back for debugging purposes
        self.registers.current_instruction = Opcode::print(self.registers.opcode).to_string();

    }
    pub fn execute_tests(&mut self) {
        self.execute_instruction();
        self.try_reset_cycles();
    }

    pub fn step(&mut self) {
    let mut cycles_executed: usize = 0;

    while cycles_executed < 16667 {
        let start_cycles = self.registers.cycles;
        self.execute_instruction();
        if self.registers.debug {
            println!("{:?}", self.registers);
        }
        cycles_executed += self.registers.cycles - start_cycles;
        self.try_interrupt();
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

        // Reset flag conditions
        self.registers.sign = false;
        self.registers.zero = false;
        self.registers.parity = false;
        self.registers.carry = false;
        self.registers.half_carry = false;

        self.adv_pc(1);
    }

    fn hlt(&mut self) {
        eprintln!("Halting CPU");
        self.adv_cycles(7);
        ::std::process::exit(1);
    }


    fn get_bc(&self) -> u16 {
        (self.registers.reg_b as u16) << 8 | (self.registers.reg_c as u16)

    }
    fn get_de(&self) -> u16 {
        (self.registers.reg_d as u16) << 8 | (self.registers.reg_e as u16)

    }
    fn get_hl(&self) -> u16 {
        (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16)

    }
    fn parity(&self, mut value: u8) -> bool {
        let mut bits: u8 = 0;
        for i in 0..8 {
            bits += (value >> i) & 1;
        }
        (bits & 1) == 0
    }
    fn emulate_interrupt(&mut self) {
        if self.registers.interrupt {
            let ret: u16 = self.registers.pc + 3;

            self.memory.memory[(self.registers.sp as usize).wrapping_sub(1) & 0xFFFF] = (ret as u16 >> 8) as u8;
            self.memory.memory[(self.registers.sp as usize).wrapping_sub(2) & 0xFFFF] = ret as u8;

            self.registers.sp = self.registers.sp.wrapping_sub(2);

            println!("Servicing interrupt {:04X}", u16::from(self.registers.interrupt_addr));

            self.registers.prev_pc = self.registers.pc;
            self.registers.pc = u16::from(self.registers.interrupt_addr);
            self.registers.interrupt = false;
        }
    }
    pub fn try_reset_cycles(&mut self) {
        if self.registers.cycles < 16_667 {
            return;
        } else {
            self.registers.cycles = 0;
        }
    }

    pub fn try_interrupt(&mut self) {
        self.try_reset_cycles();

        if self.registers.interrupt_addr == 0x08 {
            self.emulate_interrupt();
            self.registers.interrupt_addr = 0x10;

        } else if self.registers.interrupt_addr == 0x10 {
            self.emulate_interrupt();
            self.registers.interrupt_addr = 0x08;
        }
    }
}
