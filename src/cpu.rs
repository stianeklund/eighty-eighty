use std::fs::File;
use std::path::Path;

use opcode::{Instruction, Register, RegisterPair};
// use super::interconnect::Interconnect;
use memory::Memory;

const DEBUG: bool = true;

// Intel 8080 Notes:
//
// The Intel 8080 has 7 8-bit registers (A,B,C,D,E,H and L).
// The A register is the primary 8-bit accumulator.
// The other 6 registers can be used as individual registers, or as 3 16-bit
// register pairs
// (BC, DE and HL).

// Some instructions enable the HL register pair as a 16-bit accumulator & a
// psuedo reg, M.
// The M register can be used almost anywhere that any other registers can use,
// referring to the memory address pointed to by the HL pair.

// BC, DE, or HL, (referred to as B, D, H in Intel documents)
// or SP can be loaded with an immediate 16-bit value (using LXI).
// Incremented or decremented (using INX and DCX)
// or added to HL (using DAD).

// The 8080 has a 16-bit stack pointer, and a 16-bit program counter

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub opcode: u8,

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
    pub reg_m: u8, // psuedo register

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
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            opcode: 0,

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
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExecutionContext<'a> {
    pub memory: &'a mut Memory,
    pub registers: &'a mut Registers,
}

impl<'a> ExecutionContext<'a> {
    pub fn new(memory: &'a mut Memory, registers: &'a mut Registers) -> Self {
        ExecutionContext {
            memory: memory,
            registers: registers,
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

    fn write_rp(&mut self, reg: RegisterPair, value: u8) {
        match reg {
            RegisterPair::BC => self.registers.reg_bc = value as u16,
            RegisterPair::DE => self.registers.reg_de = value as u16,
            RegisterPair::HL => self.registers.reg_hl = value as u16,
        }
    }

    fn adv_pc(&mut self, t: u16) {
        self.registers.pc += t;
    }

    fn adv_cycles(&mut self, t: usize) {
        self.registers.cycles += t;
    }

    // TODO Read page 18 of 8080 Programmers Manual
    fn adc(&mut self, reg: Register) {
        let mut a = self.registers.reg_a;

        match reg {
            Register::A => {
                if self.registers.carry == true {
                    a += self.registers.reg_a;
                }
            }

            Register::B => a += self.registers.reg_b,
            Register::C => a += self.registers.reg_c,
            Register::D => a += self.registers.reg_d,
            Register::E => a += self.registers.reg_e,
            Register::H => a += self.registers.reg_h,
            Register::L => a += self.registers.reg_l,
            Register::M => a += self.registers.reg_m,
        }
    }

    fn add(&mut self, reg: Register) {
        let mut reg_a = self.registers.reg_a;

        match reg {
            Register::A => reg_a += self.registers.reg_a,
            Register::B => reg_a += self.registers.reg_b,
            Register::C => reg_a += self.registers.reg_c,
            Register::D => reg_a += self.registers.reg_d,
            Register::E => reg_a += self.registers.reg_e,
            Register::H => reg_a += self.registers.reg_h,
            Register::L => reg_a += self.registers.reg_l,
            Register::M => reg_a += self.registers.reg_m,
        }
        self.registers.half_carry = self.half_carry_add(reg_a as u16) == 0;
        self.registers.carry = reg_a > 0xFF;
        self.registers.parity = self.parity(reg_a & 0x0F);
        self.adv_pc(1);
        self.adv_cycles(4)
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
                if DEBUG {
                    println!("Setting half carry flag for ANA: {}", self.registers.half_carry);
                }
                self.registers.reg_a &= self.registers.reg_b;
            }

            Register::C => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_c) & 0x08 != 0;
                if DEBUG {
                    println!("Setting half carry flag for ANA: {}", self.registers.half_carry);
                }
                self.registers.reg_a &= self.registers.reg_c;
            }

            Register::D => {
                self.registers.half_carry = (self.registers.reg_a | self.registers.reg_d) & 0x08 != 0;
                if DEBUG {
                    println!("Setting half carry flag for ANA: {}", self.registers.half_carry);
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
        self.registers.reg_a += self.registers.opcode;
        self.registers.carry = true;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn adi(&mut self) {
        // Add Immediate to Accumulator

        // I'm not sure this is correct, investigate this.
        // self.registers.opcode & 0xF00 >> 8;
        self.registers.reg_a += self.registers.opcode;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn jmp(&mut self) {
        self.registers.pc = self.memory.read_imm(self.registers.pc);
        if DEBUG {
            println!("Jumping to address: {:04X}", self.registers.pc);
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
        let hl = (self.registers.reg_h as u16) << 8 | self.registers.reg_l as u16;
        self.registers.pc = hl;
        if DEBUG {
            println!("Jumping to address: {:X}", self.registers.pc);
        };
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn lxi_sp(&mut self) {
        self.registers.sp = self.memory.read_imm(self.registers.pc);

        self.adv_pc(3);
        self.adv_cycles(10);
    }

    // Load Register Pair Immediate
    // E.g: LXI H, 2000H (2000H is stored in the HL reg pair and acts as as memory
    // pointer)
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
        };

        self.adv_cycles(10);
        self.adv_pc(3);
    }

    fn sta(&mut self) {
        let reg_a = self.registers.reg_a;

        let value = self.memory.read_imm(self.registers.pc);

        let addr = value + 2 << 8 | value + 1;
        self.memory.write_word(reg_a as u16, addr);

        self.adv_pc(3);
        self.adv_cycles(13);
    }

    fn call(&mut self, addr: u16) {
        // CALL instructions occupy three bytes. (See page 34 of the 8080 Programmers
        // Manual)
        // CALL is just like JMP but also pushes a return address to stack.
        let ret: u16 = self.registers.pc + 3;

        match addr {
            0xCC | 0xCD | 0xC4 | 0xCC | 0xD4 | 0xDC | 0xE4 | 0xEC | 0xF4 | 0xFC => {
                // High order byte
                self.memory.memory[self.registers.sp.wrapping_sub(1) as usize] = (ret >> 8 & 0xFF) as u8;
                // Low order byte
                self.memory.memory[self.registers.sp.wrapping_sub(2) as usize] = ret as u8 & 0xFF;

                // Push return address to stack
                self.registers.sp = self.registers.sp.wrapping_sub(2);
                self.registers.pc = self.memory.read_imm(self.registers.pc);
            }
            _ => println!("Unknown call address: {:04X}", self.registers.opcode),
        };

        // if DEBUG {
            // println!("Subroutine call: {:04X}", self.registers.pc);
            // println!("Return address is: {:04X}", ret);
        // }

        self.adv_cycles(17);
    }

    fn cm(&mut self, addr: u16) {
        if self.registers.sign == true {
            self.call(addr);
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

        // The specified byte is compared to the contents of the accumulator
        // The comparison is performed by internally subtracting
        // the contents of REG from the accumulator (leaving both unchanged)
        // and setting the conditional flags according to the result
        // The Zero flag should be set if the quantities are equal
        // and reset if they're not equal

        // Since a subtraction operation is performed the carry bit should be set
        // if there is no carry out of bit 7, indicating that the contents of REG
        // are greater than the contents of the accumulator
        // Otherwise it should reset

        // Flag Register bits:
        // 7  6  5  4  3  2  1  0
        // S  Z  0  A  0  P  1  C
        // Sign | Zero | Not used | AC | Not used | Parity | Always 1 | Carry

        // Conditional Flags affected: Carry, Zero, Sign, Parity, Half Carry
        // E.g:
        // Accumulator:
        // 0 0 0 0 1 0 1
        // B Register:
        // 1 1 1 1 1 0 1 1
        // Result:
        // 0 0 0 0 0 1 0 1
        let mut result = self.registers.reg_a;
        match reg {
            Register::A => {
                if result < self.registers.reg_a {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_a {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_a {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::B => {
                if result < self.registers.reg_b {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_b {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_b {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::C => {
                if result < self.registers.reg_c {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_c {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_c {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::D => {
                if result < self.registers.reg_d {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_d {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_d {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::E => {
                if result < self.registers.reg_e {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_e {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_e {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::H => {
                if result < self.registers.reg_h {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_h {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_h {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::L => {
                if result < self.registers.reg_l {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_l {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_l {
                    self.registers.carry = true;
                    self.registers.zero = true;
                }
            }
            Register::M => {
                if result < self.registers.reg_m {
                    self.registers.carry = true;
                }
                if result == self.registers.reg_m {
                    self.registers.zero = true;
                }
                if result > self.registers.reg_m {
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
        println!("Value: {:X}", value);
        println!("A reg: {:X}", self.registers.reg_a);

        let result = value - self.registers.reg_a;
        println!("Result: {:X}", result);
        println!("Zero result: {:X}", result & 0xFF);

        self.registers.sign = result & 0x80 != 0;
        self.registers.zero = result & 0xFF == 0;
        self.registers.half_carry = !self.half_carry_sub(value as u16) != 0;
        self.registers.carry = (result & 0x0100) != 0;
        self.registers.parity = self.parity(result as u8 & 0xFF);

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

    // CALL if plus
    fn cp(&mut self, addr: u16) {
        if self.registers.sign {
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

        let mut value: u16 = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16) as u16;

        match reg {
            RegisterPair::BC => {
                value;
                value = value.wrapping_add(
                    (self.registers.reg_b as u16) >> 8 | (self.registers.reg_c as u16),
                );
                self.registers.half_carry = 0 < (value & 0xFFFF);
            }

            RegisterPair::DE => {
                value;

                value += (self.registers.reg_d as u16) >> 8 | (self.registers.reg_e as u16);
                self.registers.half_carry = 0 < (value & 0xFFFF);
            }

            RegisterPair::HL => {
                println!("DAD shouldn't run on HL, dad_sp handles this");
            }
        };

        self.registers.reg_h = ((value as u16) >> 8 & 0xFFFF) as u8;
        self.registers.reg_l = ((value as u16) >> 0 & 0xFFFF) as u8;

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn dad_sp(&mut self) {
        let mut value: u16;
        value = (self.registers.reg_h.wrapping_shl(8) | self.registers.reg_l) as u16;
        self.registers.carry = true;
        self.adv_pc(1);
        self.adv_cycles(10);
        self.registers.sp = value;
    }

    // Decrement memory or register
    fn dcr(&mut self, reg: Register) {
        // Example:
        // If the H register contains 3AH, and the L register contains 7CH
        // and memory location 3A7CH contains 40H, the instruction:
        // DCR M will cause memory location 3A7CH to contain 3FH.

        match reg {
            Register::A => {
                self.registers.reg_a = self.registers.reg_a - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_a & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_a & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_a & 0xFF);
                self.registers.sign = self.registers.reg_a & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::B => {
                self.registers.reg_b = self.registers.reg_b - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_b & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_b & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_b & 0xFF);
                self.registers.sign = self.registers.reg_b & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::C => {
                self.registers.reg_c = self.registers.reg_c - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_c & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_c & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_c & 0xFF);
                self.registers.sign = self.registers.reg_c & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::D => {
                self.registers.reg_d = self.registers.reg_d - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_d & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_d & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_d & 0xFF);
                self.registers.sign = self.registers.reg_d & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::E => {
                self.registers.reg_e = self.registers.reg_e - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_e & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_e & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_e & 0xFF);
                self.registers.sign = self.registers.reg_e & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::H => {
                self.registers.reg_h = self.registers.reg_h - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_h & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_h & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_h & 0xFF);
                self.registers.sign = self.registers.reg_h & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::L => {
                self.registers.reg_l = self.registers.reg_l - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_l & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_l & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_l & 0xFF);
                self.registers.sign = self.registers.reg_l & 0x80 != 0;
                self.adv_cycles(5);
            }

            Register::M => {
                self.registers.reg_m = self.registers.reg_m - 1 & 0xFF;
                self.registers.half_carry = !self.registers.reg_m & 0x0F == 0x0F;
                self.registers.zero = self.registers.reg_m & 0xFF == 0;
                self.registers.parity = self.parity(self.registers.reg_m & 0xFF);
                self.registers.sign = self.registers.reg_m & 0x80 != 0;
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
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn dcx_sp(&mut self) {
        self.registers.sp.wrapping_sub(1);
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // TODO
    fn daa(&mut self) {
        println!("Implementation not finished");
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // TODO
    fn ei(&mut self) {
        println!("Implementation not finished");
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // Rotate Accumulator Left Through Carry
    fn ral(&mut self) {
        let ral_debug: bool = true;

        // The contents of the accumulator are rotated one bit position to the left.
        // The high-order bit of the accumulator replaces the carry bit
        // while the carry bit replaces the high-order bit of the accumulator
        // Conditional flags affected: Carry

        // Example (Accumulator) carry is set:
        // 1 0 1 (1 0) 1 0 1
        // After RAL instruction:
        // 0 1 1 (0 1) 0 1 0

        if ral_debug {
            // Set Accumulator value for debugging purposes
            // self.registers.reg_a = 0b10110101;
            println!("RAL, Accumulator: {:b}", self.registers.reg_a);
        }
        self.registers.reg_a = self.registers.reg_a << 1;
        self.registers.carry = (self.registers.reg_a << 1) | ((self.registers.reg_a) & 0x40) != 1;
        if ral_debug {
            println!("After RAL, Accumulator: {:b}", self.registers.reg_a);
        }
        self.adv_pc(1);
        self.adv_cycles(4);
    }
    // Rotate Accumulator Right Through Carry
    fn rar(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        // If last bit is 1 bit shift one up so that the accumulator is 1
        self.registers.reg_a = (self.registers.reg_a >> 1) | (self.registers.reg_a << 7);
        self.registers.carry = self.registers.reg_a & 0x08 != 0;

        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // Rotate Accumulator Left
    fn rlc(&mut self) {
        // The Carry bit is set equal to the high-order bit of the accumulator
        // If one of the 4 higher bits are 1 we set the carry flag.
        self.registers.reg_a.rotate_left(1);
        self.registers.carry = self.registers.reg_a & 0x08 != 0;

        self.adv_pc(1);
        self.adv_cycles(4);
    }

    fn rrc(&mut self) {
        // The Carry bit is set equal to the low-order bit of the accumulator
        // If one of the 4 lower bits are 1 we set the carry flag.
        self.registers.carry = self.registers.reg_a & 0x08 != 0;
        self.registers.reg_a = (self.registers.reg_a >> 1) | ((self.registers.reg_a & 0x1) << 7);
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // Return if no carry
    fn rnc(&mut self) {
        if !self.registers.carry {
            self.ret();
        } else {
            self.adv_pc(1);
        }
    }
    // TODO
    fn rpe(&mut self) {
        self.adv_pc(1);
        // TODO Cycles 11 / 5
        // self.adv_cycles(4);
    }
    fn rc(&mut self) {
        // If Carry flag is set, return from subroutine
        println!("Implementation not finished");
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
            self.ret()
        }
        if !self.registers.carry {
            self.adv_cycles(11)
        } else {
            self.adv_cycles(5);
        }
        self.adv_pc(1);
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
        }
        if !self.registers.carry {
            self.adv_cycles(11)
        } else {
            self.adv_cycles(5);
        }
        self.adv_pc(1);
    }

    fn mvi(&mut self, reg: Register) {
        let value = self.memory.read_imm(self.registers.pc) as u16;

        match reg {
            Register::A => self.write_reg(Register::A, value as u8),
            Register::B => self.write_reg(Register::B, value as u8),
            Register::C => self.write_reg(Register::C, value as u8),
            Register::D => self.write_reg(Register::D, value as u8),
            Register::E => self.write_reg(Register::D, value as u8),
            Register::H => self.write_reg(Register::D, value as u8),
            Register::L => self.write_reg(Register::D, value as u8),
            Register::M => {
                self.write_reg(Register::M, value as u8);
                self.adv_cycles(3);
            }
        }
        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn lda(&mut self) {
        let addr = self.memory.read_imm(self.registers.pc + 3) as u8;
        self.registers.reg_a = addr;
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
                let addr = (self.registers.reg_b.wrapping_shl(8) | self.registers.reg_c) as u16;
                self.registers.reg_a = self.memory.memory[addr as usize];
            }

            RegisterPair::DE => {
                let addr = (self.registers.reg_d.wrapping_shl(8) | self.registers.reg_e) as u16;
                self.registers.reg_a = self.memory.memory[addr as usize];
                println!("LDA RP Register A value: {:X}", self.registers.reg_a);
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

        self.registers.reg_l = self.memory.read_low(self.registers.pc);
        self.registers.reg_h = self.memory.read_high(self.registers.pc);

        self.adv_cycles(16);
        self.adv_pc(3);
    }

    // TODO Read up on IN & OUT instructions
    fn input(&mut self) {
        println!("Skipping IN instruction");
        self.adv_cycles(10);
        self.adv_pc(2);
    }
    fn inr(&mut self, reg: Register) {
        let mut value: u8 = 0;
        match reg {
            Register::A => {
                self.registers.reg_a += 1;
                value = self.registers.reg_a;
            }
            Register::B => {
                self.registers.reg_b += 1;
                value = self.registers.reg_b;
            }
            Register::C => {
                self.registers.reg_c += 1;
                value = self.registers.reg_c;
            }
            Register::D => {
                self.registers.reg_d += 1;
                value = self.registers.reg_d;
            }
            Register::E => {
                self.registers.reg_e += 1;
                value = self.registers.reg_e;
            }
            Register::H => {
                self.registers.reg_h += 1;
                value = self.registers.reg_h;
            }
            Register::L => {
                self.registers.reg_l += 1;
                value = self.registers.reg_l;
            }
            Register::M => {
                self.registers.reg_m += 1;
                value = self.registers.reg_m;
            }
        };

        if reg == Register::M {
            self.adv_cycles(10);
        } else {
            self.adv_cycles(5);
        }
        self.registers.sign = value & 0x80 != 0;
        self.registers.zero = value == 0;
        self.registers.half_carry = value & 0x0F == 0;
        self.registers.parity = self.parity(value as u8);
        self.adv_pc(2);
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
        };

        self.adv_cycles(6);
        self.adv_pc(1);
    }

    fn inx_sp(&mut self) {
        self.registers.sp += 1;
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn push(&mut self, reg: Register) {
        match reg {
            Register::B => {
                self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_b;
                self.memory.memory[self.registers.sp as usize - 2] = self.registers.reg_c;
                self.registers.sp = self.registers.sp - 2;
            }

            Register::D => {
                self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_d;
                self.memory.memory[self.registers.sp as usize - 2] = self.registers.reg_e;
                self.registers.sp = self.registers.sp - 2;
            }

            Register::H => {
                self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_h;
                self.memory.memory[self.registers.sp as usize - 2] = self.registers.reg_l;
                self.registers.sp = self.registers.sp - 2;
            }

            _ => println!("Unknown push instruction"),
        }
        self.adv_cycles(11);
        self.adv_pc(1);
    }
    fn push_psw(&mut self) {
        self.memory.memory[self.registers.sp as usize - 1] = self.registers.reg_a;
        self.memory.memory[self.registers.sp as usize - 2];
        self.registers.sp = self.registers.sp.wrapping_sub(2);
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
            Register::M => self.registers.reg_a ^= self.registers.reg_m,
        };

        if reg == Register::M {
            self.adv_cycles(3);
        } else {
            self.adv_cycles(4);
        }

        self.adv_pc(1);
    }

    // XRI Exclusive-Or Immediate with Accumulator
    fn xri(&mut self) {
        self.registers.half_carry = (self.registers.reg_a | self.registers.opcode) & 0x08 != 0;
        self.registers.reg_a ^= self.registers.opcode;
        self.registers.carry = false;
        self.adv_pc(2);
        self.adv_cycles(7);
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
        self.registers.reg_l = self.memory.memory[self.registers.sp as usize + 0];
        self.registers.reg_h = self.memory.memory[self.registers.sp as usize + 1];
        self.memory.memory[self.registers.sp as usize + 0] = self.registers.reg_l;
        self.memory.memory[self.registers.sp as usize + 1] = self.registers.reg_h;

        self.adv_cycles(18);
        self.adv_pc(1);
    }


    fn pop(&mut self, reg: RegisterPair) {
        let mut sp = self.registers.sp as usize;
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
        }
        self.registers.sp = self.registers.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_psw(&mut self) {
        self.registers.reg_a = self.memory.memory[self.registers.sp as usize + 1];
        self.registers.zero = self.memory.memory[self.registers.sp as usize] & 0x40 == 0;
        self.registers.sign = self.memory.memory[self.registers.sp as usize] & 0x80 == 0;
        self.registers.parity = self.memory.memory[self.registers.sp as usize] & 0x04 == 0;
        self.registers.carry = self.memory.memory[self.registers.sp as usize] & 0x01 == 0;
        self.registers.half_carry = self.memory.memory[self.registers.sp as usize] & 0x10 == 0;

        self.registers.sp = self.registers.sp.wrapping_add(2);
        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn pop_stack(&mut self) -> u16 {
        let sp = self.memory.read_word(self.registers.sp);
        if DEBUG {
            println!("Popping stack. SP value: {:04X}", sp);
        }
        self.registers.sp += 2;
        sp
    }

    fn ret(&mut self) {
        let return_addr = self.memory.pop(self.registers.sp);
        if DEBUG {
            println!("Returning from subroutine: {:04X}", return_addr);
        }
        self.adv_cycles(10);
        self.registers.pc = return_addr;
    }

    // TODO
    fn out(&mut self) {
        println!("Not implemented, skipping");
        self.adv_pc(2);
        self.adv_cycles(10);
    }
    fn ora(&mut self, reg: Register) {
        // TODO CPU Flags / Condition bits
        println!("CPU Flags not implemented!");
        match reg {
            Register::A => self.registers.reg_a |= self.registers.reg_a,
            Register::B => self.registers.reg_a |= self.registers.reg_b,
            Register::C => self.registers.reg_a |= self.registers.reg_c,
            Register::D => self.registers.reg_a |= self.registers.reg_d,
            Register::E => self.registers.reg_a |= self.registers.reg_e,
            Register::H => self.registers.reg_a |= self.registers.reg_h,
            Register::L => self.registers.reg_a |= self.registers.reg_l,
            Register::M => self.registers.reg_a |= self.registers.reg_m,
        }

        if reg == Register::M {
            self.adv_cycles(7);
        }

        self.adv_cycles(4);
        self.adv_pc(1);
    }
    fn mov(&mut self, dst: Register, src: Register) {
        let value = self.read_reg(src);
        match src {
            Register::A => self.write_reg(dst, value),
            Register::B => self.write_reg(dst, value),
            Register::C => self.write_reg(dst, value),
            Register::D => self.write_reg(dst, value),
            Register::E => self.write_reg(dst, value),
            Register::H => self.write_reg(dst, value),
            Register::L => self.write_reg(dst, value),
            Register::H => self.write_reg(dst, value),
            Register::M => {
                let addr = (self.registers.reg_h as u16) << 8 | (self.registers.reg_l as u16);
                let val = self.memory.memory[addr as usize];
                println!("Value:{:X}", val);
                self.write_reg(dst, val);
                self.adv_cycles(2);
            }
        }

        if DEBUG {
            println!("MOV, Source: {:?}, Destination: {:?}", src, dst);
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn rst(&mut self, value: u8) {
        let mut rst = 0u8;

        match value {
            0 => rst = 0x00,
            1 => rst = 0x08,
            2 => rst = 0x10,
            3 => rst = 0x18,
            4 => rst = 0x20,
            5 => rst = 0x28,
            6 => rst = 0x30,
            7 => rst = 0x38,

            _ => println!("RST address unknown: {:#X}", rst),
        }

        if DEBUG {
            println!("RST called: {:02X}", value);
        }

        self.registers.sp.wrapping_sub(2);

        self.adv_cycles(11);
        self.registers.pc = rst as u16;
    }

    fn sphl(&mut self) {
        self.registers.sp = (self.registers.reg_h as u16) << 8 | self.registers.reg_l as u16;
    }

    // Store H & L direct
    fn shld(&mut self) {
        let reg_a = self.registers.reg_a;
        let hl = (self.registers.reg_h as u16) << 8 | self.registers.reg_l as u16;
        self.memory.write_word(reg_a as u16, hl);

        self.adv_cycles(13);
        self.adv_pc(3);
    }

    pub fn nop(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }


    pub fn decode(&mut self, instruction: Instruction) {
        println!("Instruction: {:?},", instruction);

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
            Instruction::DcxSp => self.dcx_sp(),

            Instruction::Di => println!("Not implemented: {:?}", instruction),
            Instruction::Daa => self.daa(),
            Instruction::Dad(reg) => self.dad(reg),
            Instruction::DadSp => self.dad_sp(),
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

            Instruction::In => self.input(),
            Instruction::Inr(reg) => self.inr(reg),
            Instruction::Inx(reg) => self.inx(reg),
            Instruction::InxSp => self.inx_sp(),
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

            Instruction::Rst(0) => self.rst(1),
            Instruction::Rst(1) => self.rst(2),
            Instruction::Rst(2) => self.rst(2),
            Instruction::Rst(3) => self.rst(3),
            Instruction::Rst(4) => self.rst(4),
            Instruction::Rst(5) => self.rst(5),
            Instruction::Rst(6) => self.rst(6),
            Instruction::Rst(7) => self.rst(7),

            Instruction::Rnz => self.rnz(),
            Instruction::Rm => self.rm(),
            Instruction::Rz => self.rz(),

            Instruction::Hlt => {
                println!("HLT instruction called, resetting instead");
                self.reset();
            }

            Instruction::Sim => println!("Not implemented: {:?}", instruction),

            Instruction::Stc => self.stc(),
            Instruction::Shld => self.shld(),
            Instruction::Sphl => self.sphl(),

            Instruction::Ora(reg) => self.ora(reg),
            Instruction::Ori => println!("Not implemented: {:?}", instruction),

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
        if DEBUG {
            println!(
                "Opcode: {:#02X}, PC: {:02X}, SP: {:X}, Cycles: {}",
                self.registers.opcode,
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
                "Flags: S: {}, Z: {}, P: {}, C: {}, AC: {}",
                self.registers.sign,
                self.registers.zero,
                self.registers.parity,
                self.registers.carry,
                self.registers.half_carry
            );
            println!("Stack: {:04X}", stack as u16);
        };

        match self.registers.opcode {
            0x00 => self.decode(Instruction::Nop),
            0x01 => self.decode(Instruction::Nop),
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
            0x31 => self.decode(Instruction::LxiSp),
            0x32 => self.decode(Instruction::Sta),
            0x33 => self.decode(Instruction::InxSp),
            0x34 => self.decode(Instruction::Inr(M)),
            0x35 => self.decode(Instruction::Dcr(M)),
            0x36 => self.decode(Instruction::Mvi(M)),
            0x37 => self.decode(Instruction::Stc),
            0x38 => self.decode(Instruction::Nop),
            0x39 => self.decode(Instruction::DadSp),

            0x3A => self.decode(Instruction::Lda),
            0x3B => self.decode(Instruction::DcxSp),
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
            0xF2 => self.decode(Instruction::Jpo),
            0xF3 => self.decode(Instruction::Xthl),
            0xF4 => self.decode(Instruction::Cp(0xF4)),
            0xF5 => self.decode(Instruction::Push(H)),
            0xF6 => self.decode(Instruction::Ani),
            0xF7 => self.decode(Instruction::Rst(4)),
            0xF8 => self.decode(Instruction::Rm),
            0xF9 => self.decode(Instruction::Pchl),

            0xFA => self.decode(Instruction::Jpe),
            0xFB => self.decode(Instruction::Xchg),
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
            self.registers.pc &= 0xFFFF;
            times += 1;
        }
    }

    pub fn read_instruction(&mut self, instruction: &mut Instruction) {
        let addr = self.memory.read(self.registers.pc);
        self.execute_instruction(addr);
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
}
