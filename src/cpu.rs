use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use opcode::{Instruction, Register};

const DEBUG: bool = true;


// Intel 8080 Notes:
//
// The Intel 8080 has 7 8-bit registers (A,B,C,D,E,H and L).
// The A register is the primary 8-bit accumulator.
// The other 6 registers can be used as individual registers, or as 3 16-bit register pairs
// (BC, DE and HL).

// Some instructions enable the HL register pair as a 16-bit accumulator & a psuedo reg, M.
// The M register can be used almost anywhere that any other registers can use,
// referring to the memory address pointed to by the HL pair.

// BC, DE, or HL, (referred to as B, D, H in Intel documents)
// or SP can be loaded with an immediate 16-bit value (using LXI).
// Incremented or decremented (using INX and DCX)
// or added to HL (using DAD).

// The 8080 has a 16-bit stack pointer, and a 16-bit program counter

#[allow(dead_code)]
pub struct Cpu {

    pub memory: Box<[u8; 65536]>,
    opcode: u8,

    pc: u16,
    sp: u16,

    // 8-bit Registers
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,
    reg_m: u8, // psuedo register

    // 16-bit Register pairs
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,

    reg_psw: u16,

    // Status Register (Flags)
    sign: bool,
    zero: bool,
    parity: bool,

    carry: bool,
    half_carry: bool,

    interrupt: u8,
    interrupt_addr: u16,
    cycles: usize,

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            memory: Box::new([0; 65536]),
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

            interrupt: 0,
            interrupt_addr: 0,
            cycles: 0,

        }
    }

    #[allow(dead_code)]
    fn set_sp(&mut self, byte: u16) {
        self.sp = byte & 0xFFFF;
    }

    fn write_byte(&mut self, addr: u8, mut byte: u16) {
        byte = self.memory[addr as usize & 0xFFFF] as u16
    }

    fn write_word(&mut self, addr: u8, word: u16) {
        self.write_byte(addr, word & 0xFF);
        self.write_byte(addr + 1, (word >> 8) & 0xFF);
    }


    fn read_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
            Register::E => self.reg_e,
            Register::H => self.reg_h,
            Register::L => self.reg_l,
            Register::M => self.reg_m,
        }
    }

    fn write_reg(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
            Register::C => self.reg_c = value,
            Register::D => self.reg_d = value,
            Register::E => self.reg_e = value,
            Register::H => self.reg_h = value,
            Register::L => self.reg_l = value,
            Register::M => self.reg_m = value,
        }
    }

    // Instruction functions, possible improvement is to group functions
    // by type or separate them by type, e.g mov goes together.

    fn adv_pc(&mut self, t: u16) {
        self.pc += t;
    }
    fn adv_cycles(&mut self, t: usize) {
       self.cycles += t;
    }

    // TODO
    fn adc(&mut self, reg: Register) {
        self.adv_pc(1)
    }

    fn add(&mut self, reg: Register) {
        let mut a = self.reg_a;

        match reg {
            Register::A => a += self.reg_a,
            Register::B => a += self.reg_b,
            Register::C => a += self.reg_c,
            Register::D => a += self.reg_d,
            Register::E => a += self.reg_e,
            Register::H => a += self.reg_h,
            Register::L => a += self.reg_l,
            Register::M => a += self.reg_m,
        }

        self.adv_pc(1);
        self.adv_cycles(4)
    }

    fn ana(&mut self, reg: Register) {
        // Check if the 4th bit is set on all registers
        match reg {
            Register::A => {
                self.half_carry = (self.reg_a | self.reg_a) & 0x08 != 0;
                self.reg_a &= self.reg_a;
            },

            Register::B => {
                self.half_carry = (self.reg_a | self.reg_b) & 0x08 != 0;
                self.reg_a &= self.reg_b;
            },

            Register::C => {
                self.half_carry = (self.reg_a | self.reg_c) & 0x08 != 0;
                self.reg_a &= self.reg_c;
            },

            Register::D => {
                self.half_carry = (self.reg_a | self.reg_d) & 0x08 != 0;
                self.reg_a &= self.reg_d;
            },

            Register::E => {
                self.half_carry = (self.reg_a | self.reg_e) & 0x08 != 0;
                self.reg_a &= self.reg_e;
            },

            Register::H => {
                self.half_carry = (self.reg_a | self.reg_h) & 0x08 != 0;
                self.reg_a &= self.reg_h;
            },

            Register::L => {
                self.half_carry = (self.reg_a | self.reg_l) & 0x08 != 0;
                self.reg_a &= self.reg_l;
            },

            Register::M => {
                self.half_carry = (self.reg_a | self.reg_m) & 0x08 != 0;
                self.reg_a &= self.reg_m;
            }
        }

        self.adv_pc(1);
        self.adv_cycles(4);
    }

    fn ani(&mut self) {
        // The byte of immediate data is ANDed with the contents of the accumulator (reg_a).
        // The Carry bit is reset to zero.
        // Set half carry if the accumulator or opcode and the lower 4 bits are 1.

        self.half_carry = (self.reg_a | self.opcode) & 0x08 != 0;
        self.reg_a &= self.opcode;

        self.carry = false;
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    // TODO
    fn aci(&mut self) {
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn jmp(&mut self) {
        self.pc = (self.memory[self.pc as usize + 2] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        self.adv_cycles(10);
    }

    //TODO Conditional jump
    fn jc(&mut self) {
        self.pc = (self.memory[self.pc as usize + 2] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);

        self.adv_cycles(10);
    }


    fn lxi_sp(&mut self) {
        self.sp = (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
        self.adv_pc(3);
        self.adv_cycles(10);
    }

    // Load Register Pair Immediate
    // E.g: LXI H, 2000H (2000H is stored in the HL reg pair and acts as as memory pointer)
    fn lxi(&mut self, reg: Register) {
        match reg {
            Register::B => {
                self.reg_b = self.memory[self.pc as usize + 2];
                self.reg_c = self.memory[self.pc as usize + 1];
            },

            Register::D => {
                self.reg_d = self.memory[self.pc as usize + 2];
                self.reg_e = self.memory[self.pc as usize + 1];
            },

            Register::H => {
                self.reg_h = self.memory[self.pc as usize + 2];
                self.reg_l = self.memory[self.pc as usize + 1];
            },
            _ => println!("LXI should be run on registry pairs only"),

        };

        self.adv_pc(3);
        self.adv_cycles(10);
    }

    fn sta(&mut self) {
        let reg_a = self.reg_a;
        let reg_bc = self.reg_bc;
        self.write_word(reg_a, reg_bc);

        self.adv_pc(3);
        self.adv_cycles(13);
    }

    fn call(&mut self, addr: u16) {
        // CALL instructions occupy three bytes. (See page 34 of the 8080 Prrogrammers Manual)
        // CALL is just like JMP but also pushes a return address to stack.

        let ret: u16 = self.pc + 3;

        match self.opcode {
            0xCD |  0xE7 | 0xEF | 0xEE | 0xED | 0xDD | 0xFD | 0xFF => {

                self.memory[self.sp.wrapping_sub(1) as usize] = (ret >> 8 & 0xFF) as u8;
                self.memory[self.sp.wrapping_sub(2) as usize] = (ret & 0xFF) as u8;

                self.sp.wrapping_sub(2);

                self.pc = (self.memory[self.pc as usize + 2] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16);
            },
            _ => println!("Unknown call address: {:#X}", self.opcode),
        }

        if DEBUG {
            println!("Subroutine call: {:X}", self.pc);
            println!("Return address is: {:X}", ret);
        }

        self.adv_cycles(17);
    }


    // TODO
    fn cmp(&mut self, reg: Register) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // TODO Compare Immidiate with Accumulator
    fn cpi(&mut self) {
        self.adv_pc(2);
        self.adv_cycles(7);
    }

    fn dad(&mut self, reg: Register) {
        // Double precision ADD.
        // For these instructions, HL functions as an accumulator.
        // DAD B means BC + HL --> HL. DAD D means DE + HL -- HL.
        let value: u8;

        match reg {
            Register::B => {
                value = self.reg_h.wrapping_shl(8) | self.reg_l;
                value.wrapping_add(self.reg_b.wrapping_shl(8) | self.reg_c);
            },

            Register::D => {
                value = self.reg_h.wrapping_shl(8) | self.reg_l;
                value.wrapping_add(self.reg_d.wrapping_shl(8) | self.reg_e);
            },
            Register::H => {
                value = self.reg_h.wrapping_shl(8) | self.reg_l;
                value.wrapping_add(self.reg_h.wrapping_shl(8) | self.reg_l);
            },

            _ => println!("DAD on wrong register"),
        };
        self.adv_pc(1);
        self.adv_cycles(10);
    }

    fn dad_sp(&mut self) {
        let mut value:u16;
        value = (self.reg_h.wrapping_shl(8) | self.reg_l) as u16;
        self.carry = true;
        self.adv_pc(1);
        self.adv_cycles(10);
        value = self.sp

    }

    fn dcr(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a -= self.reg_a,
            Register::B => self.reg_b -= self.reg_b,
            Register::C => self.reg_c -= self.reg_c,
            Register::D => self.reg_d -= self.reg_d,
            Register::E => self.reg_e -= self.reg_e,
            Register::H => self.reg_h -= self.reg_h,
            Register::L => self.reg_l -= self.reg_l,
            Register::M => self.reg_m -= self.reg_m,

        }

        if reg == Register::M {
            self.adv_cycles(6);
        }

        self.adv_pc(1);
        self.adv_cycles(5);
    }

    // TODO
    fn daa(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // TODO
    fn ei(&mut self) {
        self.adv_pc(1);
        self.adv_cycles(4);
    }

    // TODO
    fn rnz(&mut self) {
        // Cycles should be 11 if the carry flag is false
        if self.carry == false {
            self.adv_cycles(6)
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    // TODO
    fn rz(&mut self) {
        // Cycles should be 11 if the carry flag is false
        if self.carry == false {
            self.adv_cycles(6)
        }
        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn mvi(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.write_reg(Register::A, value),
            Register::B => self.write_reg(Register::B, value),
            Register::C => self.write_reg(Register::C, value),
            Register::D => self.write_reg(Register::D, value),
            Register::E => self.write_reg(Register::D, value),
            Register::H => self.write_reg(Register::D, value),
            Register::L => self.write_reg(Register::D, value),
            Register::M => self.write_reg(Register::M, value),
        }
        if reg == Register::M {
            self.adv_cycles(3)
        }

        self.adv_cycles(7);
        self.adv_pc(2);
    }

    fn lda (&mut self) {
        self.reg_a = self.memory[self.pc as usize];
        self.adv_pc(3);
        self.adv_cycles(13);
    }

    fn ldax(&mut self, reg: Register) {
        // LDAX(Load accumulator indirect):
        // The contents of the designated register pair point to a memory location.
        // This instruction copies the contents of that memory location into the accumulator.
        // The contents of either the register pair or the memory location are not altered.
        match reg {
            Register::B =>  self.reg_a = 0xF,
            Register::D =>  self.reg_a = 0xD,

            _ => println!("LDAX on invalid register"),
        };

        self.adv_cycles(7);
        self.adv_pc(1);
    }

    fn inr(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a += 1,
            Register::B => self.reg_b += 1,
            Register::C => self.reg_c += 1,
            Register::D => self.reg_d += 1,
            Register::E => self.reg_e += 1,
            Register::H => self.reg_h += 1,
            Register::L => self.reg_l += 1,
            Register::M => self.reg_m += 1,
        };

        if reg == Register::M {
            self.adv_cycles(5);
        }

        self.adv_cycles(5);
        self.adv_pc(2);
    }

    fn inx(&mut self, reg: Register) {

        match reg {
            Register::B => {
                self.reg_c += 1;
                if self.reg_c == 0 {
                    self.reg_b += 1;
                }
            },

            Register::D => {
                self.reg_e += 1;
                if self.reg_e == 0 {
                    self.reg_d += 1;
                }
            },

            Register::H => {
                self.reg_l += 1;
                if self.reg_l == 0 {
                    self.reg_h += 1;
                }
            },

            _ => println!("INX call on the wrong register"),
        };

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn inx_sp(&mut self) {
        self.sp += 1;

        self.adv_cycles(5);
        self.adv_pc(1);
    }

    fn push(&mut self, reg: Register) {
        match reg {
            Register::B => {
                self.memory[self.sp.wrapping_sub(2) as usize] = self.reg_c;
                self.memory[self.sp.wrapping_sub(1) as usize] = self.reg_b;
                self.sp.wrapping_sub(2);
            },

            Register::D => {
                self.memory[self.sp.wrapping_sub(2) as usize] = self.reg_e;
                self.memory[self.sp.wrapping_sub(1) as usize] = self.reg_d;
                self.sp.wrapping_sub(2);
            },

            Register::H => {
                self.memory[self.sp.wrapping_sub(2) as usize] = self.reg_l;
                self.memory[self.sp.wrapping_sub(1) as usize] = self.reg_h;
                self.sp.wrapping_sub(2);
            },

            _ => println!("Unknown push instruction"),
        }

        self.sp.wrapping_sub(2);

        self.adv_cycles(11);
        self.adv_pc(1);
    }

    // Store the contents of the accumulator addressed by registers B, C
    // or by registers D and E.
    fn stax(&mut self, reg: Register) {
        let reg_bc = self.reg_bc;

        match reg {
            Register::B => self.memory[self.reg_b.wrapping_shl(8) as usize | self.reg_c as usize] = self.reg_a,
            Register::C => self.memory[self.reg_c.wrapping_shl(8) as usize | self.reg_c as usize] = self.reg_a,
            Register::D => self.memory[self.reg_e.wrapping_shl(8) as usize | self.reg_c as usize] = self.reg_a,
            Register::E => self.memory[self.reg_e.wrapping_shl(8) as usize | self.reg_c as usize] = self.reg_a,

            _ => println!("STAX call to invalid registry"),
        };

        self.adv_cycles(7);
        self.adv_pc(1);
    }

    // TODO SBB
    fn sbb(&mut self, reg: Register) {
        self.adv_cycles(4);
        self.adv_pc(1);
    }

    fn sub(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a - self.reg_a,
            Register::B => self.reg_b - self.reg_b,
            Register::C => self.reg_c - self.reg_c,
            Register::D => self.reg_d - self.reg_d,
            Register::E => self.reg_e - self.reg_e,
            Register::H => self.reg_h - self.reg_h,
            Register::L => self.reg_l - self.reg_l,
            Register::M => self.reg_m - self.reg_m,
        };

        if reg == Register::M {
            self.adv_cycles(4);
        }

        self.adv_cycles(4);
        self.adv_pc(1);

    }

    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    fn xra(&mut self, reg: Register) {
        match reg {
            Register::A => self.reg_a ^= self.reg_a,
            Register::B => self.reg_a ^= self.reg_b,
            Register::C => self.reg_a ^= self.reg_c,
            Register::D => self.reg_a ^= self.reg_d,
            Register::E => self.reg_a ^= self.reg_e,
            Register::H => self.reg_a ^= self.reg_h,
            Register::L => self.reg_a ^= self.reg_l,
            Register::M => self.reg_a ^= self.reg_m,
        };

        if reg == Register::M {
            self.adv_cycles(3);
        }

        self.adv_cycles(4);
        self.adv_pc(1);
    }

    // TODO
    fn rpe(&mut self) {
        self.adv_pc(1);
        // TODO Cycles 11 / 5
        // self.adv_cycles(4);
    }

    // POP Register Pairs (TODO PSW)
    fn pop(&mut self, reg: Register) {
        match reg {
            Register::B =>  {
                self.reg_c = (self.memory[self.sp as usize + 0]) & 0xFFFF;
                self.reg_b = (self.memory[self.sp as usize + 1]) & 0xFFFF;
            },

            Register::D => {
                self.reg_e = (self.memory[self.sp as usize + 0]) & 0xFFFF;
                self.reg_d = (self.memory[self.sp as usize + 1]) & 0xFFFF;

            },

            Register::H => {
                self.reg_l = (self.memory[self.sp as usize + 0]) & 0xFFFF;
                self.reg_h = (self.memory[self.sp as usize + 1]) & 0xFFFF;
            },

            Register::L => {
                self.reg_a = (self.memory[self.sp as usize]) & 0xFFFF;
                self.reg_h = (self.memory[self.sp as usize + 1]) & 0xFFFF;
            },

            _ => println!("Can't pop this register"),
        }

        self.sp.wrapping_add(2);

        self.adv_pc(1);
        self.adv_cycles(10);
    }



    fn pop_stack(&mut self) -> u16 {
        let sp = (self.memory[self.sp as usize + 1] | self.memory[self.sp as usize]) as u16;
        self.sp += 2;
        sp

    }

    fn ret(&mut self) {
        if DEBUG { println!("Returning to previous subroutine"); }
        self.pc = self.pop_stack();
    }

    // TODO
    fn out(&mut self) {
        self.adv_pc(2);
        self.adv_cycles(10);
    }

    fn mov(&mut self, dst: Register, src: Register) {
        let value = self.read_reg(src);
        self.write_reg(dst, value);

        if DEBUG {
            println!("Read reg value: {:X}, Source: {:?}, Destination:{:?}", value, src, dst);
        }

        self.adv_pc(1);
        // Cycles depend on the registry, TODO
        self.adv_cycles(5);
    }

    fn rst(&mut self, value: u8) {

        let mut reset = 0u8;

        match value {
            0 => reset = 0x00,
            1 => reset = 0x08,
            2 => reset = 0x10,
            3 => reset = 0x18,
            4 => reset = 0x20,
            5 => reset = 0x28,
            6 => reset = 0x30,
            7 => reset = 0x38,

            _ => println!("RESET address unknown: {:#X}", reset),
        }

        self.sp.wrapping_sub(2);

        self.pc = reset as u16;
        self.adv_cycles(11);
    }


    pub fn decode(&mut self, instruction: Instruction) {
        use self::Register::*;

        if DEBUG { println!("Instruction: {:?},", instruction) };

        match instruction {
            Instruction::NOP =>  self.adv_pc(1),
            Instruction::ACI => self.aci(),

            Instruction::ADD(reg) => self.add(reg),
            Instruction::ADI => self.adv_pc(2),
            Instruction::ADC(reg) => self.adc(reg),
            Instruction::ANA(reg) => self.ana(reg),
            Instruction::ANI => self.ani(),

            Instruction::CALL(addr) => self.call(addr),
            Instruction::CPI => self.cpi(),
            Instruction::CZ => self.adv_pc(3),      // TODO
            Instruction::CM => self.adv_pc(3),      // TODO
            Instruction::CNC => self.adv_pc(3),     // TODO
            Instruction::CMC => self.adv_pc(1),     // TODO
            Instruction::CMP(reg) => self.cmp(reg),
            Instruction::CPE => self.adv_pc(3),     // TODO
            Instruction::DCR(reg) => self.dcr(reg),
            Instruction::DCX(reg) => self.dcr(reg),

            Instruction::DAA =>  self.daa(),
            Instruction::DAD(reg) => self.dad(reg),
            Instruction::DAD_SP => self.dad_sp(),
            Instruction::EI => self.ei(),
            Instruction::JC => self.jc(),
            Instruction::JMP =>  self.jmp(),
            Instruction::JPE =>  self.adv_pc(3),

            Instruction::MOV(dst, src) => self.mov(dst, src),
            Instruction::MVI(reg, value) => self.mvi(reg, value),
            Instruction::SUB(reg) => self.sub(reg),
            Instruction::SBB(reg) => self.sbb(reg),

            Instruction::XRA(reg) => self.xra(reg),
            Instruction::RPE => self.rpe(),
            Instruction::RET => self.ret(),

            Instruction::POP(reg) => self.pop(reg),
            Instruction::POP_PSW => self.adv_pc(1), // TODO
            Instruction::PUSH(reg)=> self.push(reg),

            Instruction::IN => self.adv_pc(2),
            Instruction::INR(reg) => self.inr(reg),
            Instruction::INX(reg) => self.inx(reg),
            Instruction::INX_SP => self.inx_sp(),
            Instruction::OUT => self.out(),
            Instruction::STA => self.sta(),
            Instruction::STAX(reg) => self.stax(reg),
            Instruction::LDA => self.lda(),
            Instruction::LDAX(reg) => self.ldax(reg),
            Instruction::LHLD => self.adv_pc(3),
            Instruction::LXI(reg) => self.lxi(reg),
            Instruction::LXI_SP => self.lxi_sp(),

            Instruction::RAL => self.adv_pc(1),

            Instruction::RC => self.adv_pc(1),     // TODO
            Instruction::RST(0) => self.rst(1),
            Instruction::RST(1) => self.rst(2),
            Instruction::RST(2) => self.rst(2),
            Instruction::RST(3) => self.rst(3),
            Instruction::RST(4) => self.rst(4),
            Instruction::RST(5) => self.rst(5),
            Instruction::RST(6) => self.rst(6),
            Instruction::RST(7) => self.rst(7),

            Instruction::RNZ => self.rnz(),
            Instruction::RZ => self.rz(),

            Instruction::HLT => self.adv_pc(1),     // TODO
            Instruction::RLC => self.adv_pc(1),     // TODO
            Instruction::RNC => self.adv_pc(1),     // TODO
            Instruction::RRC => self.adv_pc(1),     // TODO

            Instruction::STC => self.adv_pc(1),     // TODO
            Instruction::SHLD => self.adv_pc(3),    // TODO
            Instruction::ORA(reg) => self.adv_pc(1),// TODO

            Instruction::JNC => self.adv_pc(3),     // TODO
            Instruction::JNZ => self.adv_pc(3),     // TODO
            Instruction::JM => self.adv_pc(3),      // TODO
            Instruction::JZ => self.adv_pc(3),      // TODO
            Instruction::XRA_L => self.adv_pc(1),   // TODO
            Instruction::XRI => self.adv_pc(1),
            Instruction::XCHG => self.adv_pc(1),    // TODO
            Instruction::XTHL => self.adv_pc(1),    // TODO

            _ => println!("Unknown instruction {:#X}", self.opcode),
        }

    }

    pub fn execute_instruction(&mut self) {
        self.opcode = self.memory[self.pc as usize];
        use self::Register::*;

        if DEBUG {
            println!("Opcode: {:#X}, PC: {:X}, SP: {:X}", self.opcode, self.pc, self.sp);
            println!("A: {:X}, B: {:X}, C: {:X}, D: {:X}, E: {:X}, H: {:X}, M: {:X}",
                     self.reg_a, self.reg_b, self.reg_c, self.reg_d, self.reg_e, self.reg_h, self.reg_m);
        };

        match self.opcode {

            0x00 => self.decode(Instruction::NOP),
            0x01 => self.decode(Instruction::NOP),
            0x02 => self.decode(Instruction::STAX(B)),
            0x03 => self.decode(Instruction::INX(B)),
            0x04 => self.decode(Instruction::INR(B)),
            0x05 => self.decode(Instruction::DCR(B)),
            0x06 => self.decode(Instruction::MVI(B, 0xD8)),
            0x07 => self.decode(Instruction::RLC),
            0x08 => self.decode(Instruction::NOP),
            0x09 => self.decode(Instruction::DAD(B)),

            0x0A => self.decode(Instruction::LDAX(B)),
            0x0B => self.decode(Instruction::DCX(B)),
            0x0C => self.decode(Instruction::INR(C)),
            0x0D => self.decode(Instruction::DCR(D)),
            0x0E => self.decode(Instruction::MVI(C, 0xD8)),
            0x0F => self.decode(Instruction::RRC),


            0x10 => self.decode(Instruction::NOP),
            0x11 => self.decode(Instruction::LXI(D)),
            0x12 => self.decode(Instruction::STAX(D)),
            0x13 => self.decode(Instruction::INX(D)),
            0x14 => self.decode(Instruction::INR(D)),
            0x15 => self.decode(Instruction::DCR(D)),
            0x16 => self.decode(Instruction::MVI(D, 0xD8)),
            0x17 => self.decode(Instruction::RAL),
            0x18 => self.decode(Instruction::NOP),
            0x19 => self.decode(Instruction::DAD(D)),

            0x1A => self.decode(Instruction::LDAX(D)),
            0x1B => self.decode(Instruction::DCX(D)),
            0x1C => self.decode(Instruction::INR(E)),
            0x1D => self.decode(Instruction::DCR(E)),
            0x1E => self.decode(Instruction::MVI(E, 0xD8)),
            0x1F => self.decode(Instruction::RAR),


            0x20 => self.decode(Instruction::NOP),
            0x21 => self.decode(Instruction::LXI(H)),
            0x22 => self.decode(Instruction::SHLD),
            0x23 => self.decode(Instruction::INX(H)),
            0x24 => self.decode(Instruction::INR(H)),
            0x25 => self.decode(Instruction::DCR(H)),
            0x26 => self.decode(Instruction::MVI(H, 0xD8)),
            0x27 => self.decode(Instruction::DAA),
            0x28 => self.decode(Instruction::NOP),
            0x29 => self.decode(Instruction::DAD(H)),

            0x2A => self.decode(Instruction::LHLD),
            0x2B => self.decode(Instruction::DCX(H)),
            0x2C => self.decode(Instruction::INR(L)),
            0x2D => self.decode(Instruction::DCR(L)),
            0x2E => self.decode(Instruction::MVI(L, 0xD8)),
            0x2F => self.decode(Instruction::CMA),


            // SP specific instructions are separated from the Register & RegisterPair enum's
            // Depending on how many instructions call the stack pointer it might be good to keep this out
            // the generic instruction functions.

            0x30 => self.decode(Instruction::NOP),
            0x31 => self.decode(Instruction::LXI_SP),
            0x32 => self.decode(Instruction::STA),
            0x33 => self.decode(Instruction::INX_SP),
            0x34 => self.decode(Instruction::INR(M)),
            0x35 => self.decode(Instruction::DCR(M)),
            0x36 => self.decode(Instruction::MVI(M, 0xD8)),
            0x37 => self.decode(Instruction::STC),
            0x38 => self.decode(Instruction::NOP),
            0x39 => self.decode(Instruction::DAD_SP),

            0x3A => self.decode(Instruction::LDA),
            0x3B => self.decode(Instruction::DCX_SP),
            0x3C => self.decode(Instruction::INR(A)),
            0x3D => self.decode(Instruction::DCR(A)),
            0x3E => self.decode(Instruction::MVI(A, 0xD8)),
            0x3F => self.decode(Instruction::CMC),

            // MOV Instructions 0x40 - 0x7F
            0x40 => self.decode(Instruction::MOV(B, B)),
            0x41 => self.decode(Instruction::MOV(B, C)),
            0x42 => self.decode(Instruction::MOV(B, D)),
            0x43 => self.decode(Instruction::MOV(B, E)),
            0x44 => self.decode(Instruction::MOV(B, H)),
            0x45 => self.decode(Instruction::MOV(B, L)),
            0x46 => self.decode(Instruction::MOV(B, M)),
            0x47 => self.decode(Instruction::MOV(B, A)),

            0x48 => self.decode(Instruction::MOV(C, B)),
            0x49 => self.decode(Instruction::MOV(C, C)),
            0x4A => self.decode(Instruction::MOV(C, D)),
            0x4B => self.decode(Instruction::MOV(C, E)),
            0x4C => self.decode(Instruction::MOV(C, H)),
            0x4D => self.decode(Instruction::MOV(C, L)),
            0x4E => self.decode(Instruction::MOV(C, M)),
            0x4F => self.decode(Instruction::MOV(C, A)),

            0x50 => self.decode(Instruction::MOV(D, B)),
            0x51 => self.decode(Instruction::MOV(D, C)),
            0x52 => self.decode(Instruction::MOV(D, D)),
            0x53 => self.decode(Instruction::MOV(D, E)),
            0x54 => self.decode(Instruction::MOV(D, H)),
            0x55 => self.decode(Instruction::MOV(D, L)),
            0x56 => self.decode(Instruction::MOV(D, M)),
            0x57 => self.decode(Instruction::MOV(D, A)),

            0x58 => self.decode(Instruction::MOV(E, B)),
            0x59 => self.decode(Instruction::MOV(E, C)),
            0x5A => self.decode(Instruction::MOV(E, D)),
            0x5B => self.decode(Instruction::MOV(E, E)),
            0x5C => self.decode(Instruction::MOV(E, H)),
            0x5D => self.decode(Instruction::MOV(E, L)),
            0x5E => self.decode(Instruction::MOV(E, M)),
            0x5F => self.decode(Instruction::MOV(E, A)),

            0x60 => self.decode(Instruction::MOV(H, B)),
            0x61 => self.decode(Instruction::MOV(H, C)),
            0x62 => self.decode(Instruction::MOV(H, D)),
            0x63 => self.decode(Instruction::MOV(H, E)),
            0x64 => self.decode(Instruction::MOV(H, H)),
            0x65 => self.decode(Instruction::MOV(H, L)),
            0x66 => self.decode(Instruction::MOV(H, M)),
            0x67 => self.decode(Instruction::MOV(H, A)),

            0x68 => self.decode(Instruction::MOV(L, B)),
            0x69 => self.decode(Instruction::MOV(L, C)),
            0x6A => self.decode(Instruction::MOV(L, D)),
            0x6B => self.decode(Instruction::MOV(L, E)),
            0x6C => self.decode(Instruction::MOV(L, H)),
            0x6D => self.decode(Instruction::MOV(L, L)),
            0x6E => self.decode(Instruction::MOV(L, M)),
            0x6F => self.decode(Instruction::MOV(L, A)),

            0x70 => self.decode(Instruction::MOV(M, B)),
            0x71 => self.decode(Instruction::MOV(M, C)),
            0x72 => self.decode(Instruction::MOV(M, D)),
            0x73 => self.decode(Instruction::MOV(M, E)),
            0x74 => self.decode(Instruction::MOV(M, H)),
            0x75 => self.decode(Instruction::MOV(M, L)),
            0x76 => self.decode(Instruction::HLT),
            0x77 => self.decode(Instruction::MOV(M, A)),

            0x78 => self.decode(Instruction::MOV(A, B)),
            0x79 => self.decode(Instruction::MOV(A, C)),
            0x7A => self.decode(Instruction::MOV(A, D)),
            0x7B => self.decode(Instruction::MOV(A, E)),
            0x7C => self.decode(Instruction::MOV(A, H)),
            0x7D => self.decode(Instruction::MOV(A, L)),
            0x7E => self.decode(Instruction::MOV(A, M)),
            0x7F => self.decode(Instruction::MOV(A, A)),

            // ADD Instructions
            0x80 => self.decode(Instruction::ADD(B)),
            0x81 => self.decode(Instruction::ADD(C)),
            0x82 => self.decode(Instruction::ADD(D)),
            0x83 => self.decode(Instruction::ADD(E)),
            0x84 => self.decode(Instruction::ADD(H)),
            0x85 => self.decode(Instruction::ADD(L)),
            0x86 => self.decode(Instruction::ADD(M)),
            0x87 => self.decode(Instruction::ADD(A)),

            0x88 => self.decode(Instruction::ADC(B)),
            0x89 => self.decode(Instruction::ADC(C)),
            0x8A => self.decode(Instruction::ADC(D)),
            0x8B => self.decode(Instruction::ADC(E)),
            0x8C => self.decode(Instruction::ADC(H)),
            0x8D => self.decode(Instruction::ADC(L)),
            0x8E => self.decode(Instruction::ADC(M)),
            0x8F => self.decode(Instruction::ADC(A)),

            // SUB Instructions
            0x90 => self.decode(Instruction::SUB(B)),
            0x91 => self.decode(Instruction::SUB(C)),
            0x92 => self.decode(Instruction::SUB(D)),
            0x93 => self.decode(Instruction::SUB(E)),
            0x94 => self.decode(Instruction::SUB(H)),
            0x95 => self.decode(Instruction::SUB(L)),
            0x96 => self.decode(Instruction::SUB(M)),
            0x97 => self.decode(Instruction::SUB(A)),

            0x98 => self.decode(Instruction::SBB(B)),
            0x99 => self.decode(Instruction::SBB(C)),
            0x9A => self.decode(Instruction::SBB(D)),
            0x9B => self.decode(Instruction::SBB(E)),
            0x9C => self.decode(Instruction::SBB(H)),
            0x9D => self.decode(Instruction::SBB(L)),
            0x9E => self.decode(Instruction::SBB(M)),
            0x9F => self.decode(Instruction::SBB(A)),

            // ANA & XRA Instructions
            0xA0 => self.decode(Instruction::ANA(B)),
            0xA1 => self.decode(Instruction::ANA(C)),
            0xA2 => self.decode(Instruction::ANA(D)),
            0xA3 => self.decode(Instruction::ANA(E)),
            0xA4 => self.decode(Instruction::ANA(H)),
            0xA5 => self.decode(Instruction::ANA(L)),
            0xA6 => self.decode(Instruction::ANA(M)),
            0xA7 => self.decode(Instruction::ANA(A)),

            0xA8 => self.decode(Instruction::XRA(B)),
            0xA9 => self.decode(Instruction::XRA(C)),
            0xAA => self.decode(Instruction::XRA(D)),
            0xAB => self.decode(Instruction::XRA(E)),
            0xAC => self.decode(Instruction::XRA(H)),
            0xAD => self.decode(Instruction::XRA(L)),
            0xAE => self.decode(Instruction::XRA(M)),
            0xAF => self.decode(Instruction::XRA(A)),

            // ORA Instructions  0xB(reg)
            0xB0 => self.decode(Instruction::ORA(B)),
            0xB1 => self.decode(Instruction::ORA(C)),
            0xB2 => self.decode(Instruction::ORA(D)),
            0xB3 => self.decode(Instruction::ORA(E)),
            0xB4 => self.decode(Instruction::ORA(H)),
            0xB5 => self.decode(Instruction::ORA(L)),
            0xB6 => self.decode(Instruction::ORA(M)),
            0xB7 => self.decode(Instruction::ORA(A)),

            // CMP
            0xB8 => self.decode(Instruction::CMP(B)),
            0xB9 => self.decode(Instruction::CMP(C)),
            0xBA => self.decode(Instruction::CMP(D)),
            0xBB => self.decode(Instruction::CMP(E)),
            0xBC => self.decode(Instruction::CMP(H)),
            0xBD => self.decode(Instruction::CMP(L)),
            0xBE => self.decode(Instruction::CMP(M)),
            0xBF => self.decode(Instruction::CMP(A)),

            0xC0 => self.decode(Instruction::RNZ),
            0xC1 => self.decode(Instruction::POP(B)),
            0xC2 => self.decode(Instruction::JNZ),
            0xC3 => self.decode(Instruction::JMP),
            0xC4 => self.decode(Instruction::CNZ),
            0xC5 => self.decode(Instruction::PUSH(B)),
            0xC6 => self.decode(Instruction::ADI),
            0xC7 => self.decode(Instruction::RST(0)),
            0xC8 => self.decode(Instruction::RZ),
            0xC9 => self.decode(Instruction::RET),

            0xCA => self.decode(Instruction::JZ),
            0xCB => self.decode(Instruction::JMP),
            0xCC => self.decode(Instruction::CZ),
            0xCD => self.decode(Instruction::CALL(0xCD)),
            0xCE => self.decode(Instruction::ADI),
            0xCF => self.decode(Instruction::RST(1)),

            0xD0 => self.decode(Instruction::RNC),
            0xD1 => self.decode(Instruction::POP(D)),
            0xD2 => self.decode(Instruction::JNC),
            0xD3 => self.decode(Instruction::OUT),
            0xD4 => self.decode(Instruction::CNC),
            0xD5 => self.decode(Instruction::PUSH(D)),
            0xD6 => self.decode(Instruction::SUI),
            0xD7 => self.decode(Instruction::RST(2)),
            0xD8 => self.decode(Instruction::RC),
            0xD9 => self.decode(Instruction::RET),

            0xDA => self.decode(Instruction::JC),
            0xDB => self.decode(Instruction::IN),
            0xDC => self.decode(Instruction::CC(0xDC)),
            0xDD => self.decode(Instruction::CALL(0xDD)),
            0xDE => self.decode(Instruction::SBI),
            0xDF => self.decode(Instruction::RST(3)),

            0xE0 => self.decode(Instruction::RPO),
            0xE1 => self.decode(Instruction::POP(H)),
            0xE2 => self.decode(Instruction::JPO),
            0xE3 => self.decode(Instruction::XTHL),
            0xE4 => self.decode(Instruction::CPO),
            0xE5 => self.decode(Instruction::PUSH(H)),
            0xE6 => self.decode(Instruction::ANI),
            0xE7 => self.decode(Instruction::RST(4)),
            0xE8 => self.decode(Instruction::RPE),
            0xE9 => self.decode(Instruction::PCHL),

            0xEA => self.decode(Instruction::JPE),
            0xEB => self.decode(Instruction::XCHG),
            0xEC => self.decode(Instruction::CPE),
            0xED => self.decode(Instruction::CALL(0xED)),
            0xEE => self.decode(Instruction::XRI),
            0xEF => self.decode(Instruction::RST(5)),

            0xF0 => self.decode(Instruction::RP),
            0xF1 => self.decode(Instruction::POP(H)),
            0xF2 => self.decode(Instruction::JPO),
            0xF3 => self.decode(Instruction::XTHL),
            0xF4 => self.decode(Instruction::CPO),
            0xF5 => self.decode(Instruction::PUSH(H)),
            0xF6 => self.decode(Instruction::ANI),
            0xF7 => self.decode(Instruction::RST(4)),
            0xF8 => self.decode(Instruction::RPE),
            0xF9 => self.decode(Instruction::PCHL),

            0xFA => self.decode(Instruction::JPE),
            0xFB => self.decode(Instruction::XCHG),
            0xFC => self.decode(Instruction::CPE),
            0xFD => self.decode(Instruction::CALL(0xFD)),
            0xFE => self.decode(Instruction::XRI),
            0xFF => self.decode(Instruction::RST(7)),

            _ => println!("Unknown opcode: {:#X}", self.opcode),
        }
    }

    pub fn reset(&mut self) {
        println!("Resetting emulator");

        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_c = 0;
        self.reg_d = 0;
        self.reg_e = 0;
        self.reg_h = 0;
        self.reg_l = 0;

        self.reg_bc = 0;
        self.reg_de = 0;
        self.reg_hl = 0;

        // Reset flag conditions
        self.sign = false;
        self.zero = false;
        self.parity = false;
        self.carry = false;
        self.half_carry = false;
        self.reg_psw = 0;

        self.interrupt = 0;
    }


    pub fn load_bin(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("File open failed");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read file");


        let buf_len = buf.len();
        for i in 0..buf_len { self.memory[i] = buf[i]; }
        println!("Loaded binary");
    }
}
