use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use opcode::{Instruction, Register, RegisterPair};

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

    // 16-bit Register pairs
    reg_bc: u16,
    reg_de: u16,
    reg_hl: u16,

    reg_psw: u16,

    // Status Register (Flags)
    sign: u8,
    zero: u8,
    parity: u8,

    carry: u8,
    half_carry: u8,

    interrupt: u8,
    interrupt_addr: u16,

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

            reg_bc: 0,
            reg_de: 0,
            reg_hl: 0,

            reg_psw: 0,

            sign: 0,
            zero: 0,
            parity: 0,

            carry: 0,
            half_carry: 0,

            interrupt: 0,
            interrupt_addr: 0,

        }
    }

    pub fn set_sp(&mut self, byte: u16) {
        self.sp = byte & 0xFFFF;
    }

    pub fn read_byte(&mut self, addr: u8) -> u8 {
        self.memory[addr as usize & 0xFFFF]
    }

    pub fn write_byte(&mut self, addr: u8, mut byte: u16) {
        byte = self.memory[addr as usize & 0xFFFF] as u16;
    }

    pub fn read_word(&mut self, addr: u8) -> u16 {
        (self.read_byte(addr + 1) as u16) << 8 | self.read_byte(addr) as u16
    }

    pub fn write_word(&mut self, addr: u8, word: u16) {
        self.write_byte(addr, word & 0xFF);
        self.write_byte(addr + 1, (word >> 8) & 0xFF);
    }


    pub fn read_reg(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.reg_a,
            Register::B => self.reg_b,
            Register::C => self.reg_c,
            Register::D => self.reg_d,
            Register::E => self.reg_e,
            Register::H => self.reg_h,
            Register::L => self.reg_l,
        }
    }

    pub fn read_reg16(&self, reg: RegisterPair) -> u16 {
        match reg {
            RegisterPair::BC => self.reg_bc,
            RegisterPair::DE => self.reg_de,
            RegisterPair::HL => self.reg_hl,
        }
    }

    pub fn write_reg(&mut self, reg: Register, value: u8) {
        match reg {
            Register::A => self.reg_a = value,
            Register::B => self.reg_b = value,
            Register::C => self.reg_c = value,
            Register::D => self.reg_d = value,
            Register::E => self.reg_e = value,
            Register::H => self.reg_h = value,
            Register::L => self.reg_l = value,
        }
    }

    pub fn write_reg16(&mut self, reg: RegisterPair, value: u16) {
        match reg {
            RegisterPair::BC => self.reg_bc = value,
            RegisterPair::DE => self.reg_de = value,
            RegisterPair::HL => self.reg_hl = value,
        }
    }


    // Instruction functions, possible improvement is to group functions
    // by type or separate them by type, e.g mov goes together.

    pub fn nop(&mut self) {
        self.pc += 1;
    }

    pub fn adc(&mut self, reg: Register) {
        self.pc += 1;

    }
    pub fn add(&mut self, reg: Register) {
        let value = match self.opcode {
            Register => self.read_reg(reg),
        };
    }

    pub fn ana(&mut self, reg: Register) {
        self.pc += 1;
    }

    // TODO
    pub fn ani(&mut self) {
        self.pc+=1;
    }

    // TODO
    pub fn aci(&mut self) {
        self.pc += 1;
    }

    pub fn jmp(&mut self) {
        self.pc = (self.opcode & 0x0FFF) as u16;
    }

    pub fn lxi_sp(&mut self) {
        self.sp = (self.memory[self.pc as usize] as u16) << 8 |
        (self.memory[self.pc as usize + 1] as u16);

        self.pc += 1;
    }

    // LXI
    pub fn lxi(&mut self, reg: Register) {
        let reg = self.read_reg(reg);
        reg = self.memory[self.pc as usize + 1];

    }
    pub fn mvi_a(&mut self) {
        let byte = self.reg_a;
        self.read_byte(byte);
        self.pc += 1;
    }

    pub fn sta(&mut self) {
        let reg_a = self.reg_a;
        let reg_bc = self.reg_bc;
        self.write_word(reg_a, reg_bc);
    }

    // TODO
    pub fn call(&mut self) {
        self.pc += 1;
    }

    // TODO Compare M reg
    pub fn cmp_m(&mut self) {
        self.pc += 1;
    }
    // TODO Compare Immidiate with Accumulator
    pub fn cpi(&mut self) {
        self.pc +=1;
    }

    pub fn dcr(&mut self, register: Register) {
        // TODO See Add function
        // self.reg_b -= self.reg_b;
        self.pc += 1;
    }

    // TODO
    pub fn daa(&mut self) {
    self.pc += 1;}

    // TODO
    pub fn ei(&mut self) {
        self.pc += 1;
    }

    // TODO I believe zero & sign flags need to be set for SUB instructions.
    pub fn sub_b(&mut self) {
        self.pc+=1;
    }
    pub fn sub_c(&mut self) {
        self.pc+=1;
    }

    pub fn sub_d(&mut self) {
        self.pc+=1;
    }
    pub fn sub_e(&mut self) {
        self.pc+=1;
    }
    pub fn sub_h(&mut self) {
        self.pc+=1;
    }
    pub fn sub_l(&mut self) {
        self.pc+=1;
    }
    pub fn sub_m(&mut self) {
        self.pc+=1;
    }
    pub fn sub_a(&mut self) {
        self.pc+=1;
    }

    // TODO
    pub fn rnz(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn rz(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_mh(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_ad(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_dc(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_la(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_hd(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_hb(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn move_hc(&mut self) {
        self.pc += 1;
    }

    // TODO Increment Register
    pub fn inr(&mut self, reg: Register) {
        let value = reg;
        self.write_reg(reg = reg + value);
    }
    // Increment BC
    pub fn inr_bc(&mut self) {
        self.reg_bc += self.reg_bc;
        self.pc += 1;
    }

    // Increment PC
    pub fn inr_pc(&mut self, amount: u16) {
        self.pc += amount;
    }

    // Increment E register
    pub fn inr_e(&mut self) {
        self.reg_e += self.reg_e;
        self.pc += 1;
    }

    // TODO
    pub fn inx_h(&mut self) {
        self.reg_h < self.reg_h + 1;
        self.pc += 1;
    }

    // TODO
    pub fn inx_sp(&mut self) {
        self.sp < self.sp + 1;
        self.pc += 1;
    }

    // PUSH B register
    pub fn push_b(&mut self) {
        self.sp -= 2;
        let sp = self.sp;
        let b = self.reg_b;
        self.write_word(b, sp);
        self.pc += 1;
    }
    // PUSH D register
    pub fn push_d(&mut self) {
        // self.sp -= 2;
        self.sp.wrapping_sub(2);
        let sp = self.sp;
        let d = self.reg_d;
        self.write_word(d, sp);
        self.pc += 1;
    }

    // TODO STAX_D
    pub fn stax_d(&mut self) {
        self.pc += 1;
    }

    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    pub fn xra_a(&mut self) {
        self.pc += 1;

    }

    pub fn xra_m(&mut self) {
        self.pc += 1;

    }

    // TODO
    pub fn xra_h(&mut self) {
        self.pc += 1;

    }
    // TODO
    pub fn rpe(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn out(&mut self) {
        self.pc += 1;
    }

    pub fn mov(&mut self,src: Register, dst: Register) {
        let value = self.read_reg(src);
        self.write_reg(dst, value);
    }

    // I think it might be a good idea to segment instructions based on functionality.
    // Move logical operations to a separate file, jump & call to one etc?
    pub fn decode(&mut self, instruction: Instruction) {
        use self::Register::*;

        match instruction {
            Instruction::NOP => self.nop(),
            Instruction::ACI => self.aci(),

            Instruction::ADD(reg) => self.add(reg),
            Instruction::ADC(reg) => self.adc(reg),
            Instruction::ANA(reg) => self.ana(reg),

            Instruction::INR(reg) => self.inr(reg),
            Instruction::CALL => self.call(),
            Instruction::CPI => self.cpi(),
            Instruction::CMP(reg) => self.cmp(reg),
            Instruction::DCR(reg) => self.dcr(reg),

            Instruction::DAA =>  self.daa(),
            Instruction::EI => self.ei(),
            Instruction::JMP =>  { self.pc = (self.opcode & 0x0FFF) as u16; },
            Instruction::RNZ => self.rnz(),
            Instruction::RZ => self.rz(),

            // MOV Instructions
            Instruction::MOV(dst, src) => self.mov(dst, src),
            Instruction::MVI(reg) => self.mvi(reg),
            Instruction::SUB(reg) => self.sub(reg),
            Instruction::SBB(reg) => self.sbb(reg),

            Instruction::XRA(reg) => self.xra(reg),
            Instruction::XRA_M => self.xra_m(),
            Instruction::XRA_H => self.xra_h(),
            Instruction::RPE => self.rpe(),

            Instruction::PUSH_D => self.push_d(),

            Instruction::INX_H => self.inx_h(),
            Instruction::INX_SP => self.inx_sp(),
            Instruction::INR_B => self.inr_b(),
            Instruction::OUT => self.out(),
            Instruction::ANI => self.ani(),
            Instruction::STAX_D => self.stax_d(),
            Instruction::LXI_D => self.lxi_d(),
            Instruction::LXI_SP => self.lxi_sp(),
            Instruction::RST_7 => self.nop(), // self.reset(),
            Instruction::INR_E => self.inr_e(),
            Instruction::CM => self.nop(), // TODO
            Instruction::CMC => self.nop(), // TODO
            Instruction::HLT => self.nop(), // self.reset(),
            Instruction::RRC => self.nop(), // TODO
            Instruction::XRA_L => self.nop(), // TODO

            Instruction::ORA_B => self.nop(), // TODO
            Instruction::ORA_C => self.nop(), // TODO
            Instruction::ORA_D => self.nop(), // TODO
            Instruction::ORA_E => self.nop(), // TODO
            Instruction::ORA_H => self.nop(), // TODO
            Instruction::ORA_L => self.nop(), // TODO
            Instruction::ORA_M => self.nop(), // TODO
            Instruction::ORA_A => self.nop(), // TODO

            Instruction::CMP_B => self.nop(), // TODO
            Instruction::CMP_C => self.nop(), // TODO
            Instruction::CMP_D => self.nop(), // TODO
            Instruction::CMP_E => self.nop(), // TODO
            Instruction::CMP_H => self.nop(), // TODO
            Instruction::CMP_L => self.nop(), // TODO
            Instruction::CMP_M => self.nop(), // TODO
            Instruction::CMP_A => self.nop(), // TODO

            Instruction::POP_B => self.nop(), // TODO
            Instruction::POP_D => self.nop(), // TODO
            Instruction::POP_H => self.nop(), // TODO
            Instruction::POP_PSW => self.nop(), // TODO

            Instruction::JNZ => self.nop(), // TODO
            Instruction::JM => self.nop(), // TODO

            _ => println!("Unknown instruction {:X}", self.opcode),
        }

    }
    pub fn execute_instruction(&mut self) {
        self.opcode = self.memory[self.pc as usize];

        use self::Register::*;
        use self::RegisterPair::*;

        if DEBUG { println!("Opcode: 0x{:X}, PC: {}, SP: {}", self.opcode, self.pc, self.sp); }

        match self.opcode {
            // NOP Instructions: Do nothing
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                self.decode(Instruction::NOP);
            },

            0x2 => self.decode(Instruction::NOP),
            0x2C => self.decode(Instruction::JNZ),
            0x4 => self.decode(Instruction::INR_B),
            0x01 => self.decode(Instruction::NOP),
            0x02 => self.decode(Instruction::STA),
            0x03 => self.decode(Instruction::INR_B),

            0x05 => self.decode(Instruction::DCR_B),
            0x06 => self.decode(Instruction::MVI_B),
            0x11 => self.decode(Instruction::LXI_D),
            0x13 => self.decode(Instruction::LXI_SP),
            0x14 => self.decode(Instruction::INR_D),

            0x15 => self.decode(Instruction::MOV(D, C)),

            0x16 => self.decode(Instruction::MOV(H, C)),
            // 0x17 => self.decode(Instruction::MOV(HL, C)),
            0x17 => self.decode(Instruction::MOV_R_PR(HL, C)),
            0x19 => self.decode(Instruction::SUB_C),
            0x1C => self.decode(Instruction::POP_B),
            0x21 => self.decode(Instruction::STAX_D),
            0x26 => self.decode(Instruction::MOV_H_D),

            0x5D => self.decode(Instruction::PUSH_D),
            0x6 => self.decode(Instruction::MOV_H_B),
            0x9 => self.decode(Instruction::SUB_B),
            0xA => self.decode(Instruction::ANA_B),
            0xAF => self.decode(Instruction::XRA_A),
            0xA7 => self.decode(Instruction::MOV_A_D),

            // ORA Instructions  0xB(reg)
            0xB0 => self.decode(Instruction::ORA_B),
            0xB1 => self.decode(Instruction::ORA_C),
            0xB2 => self.decode(Instruction::ORA_D),
            0xB3 => self.decode(Instruction::ORA_E),
            0xB4 => self.decode(Instruction::ORA_H),
            0xB5 => self.decode(Instruction::ORA_L),
            0xB6 => self.decode(Instruction::ORA_M),
            0xB7 => self.decode(Instruction::ORA_A),

            // CMP
            0xB8 => self.decode(Instruction::CMP_B),
            0xB9 => self.decode(Instruction::CMP_C),
            0xBA => self.decode(Instruction::CMP_D),
            0xBB => self.decode(Instruction::CMP_E),
            0xBC => self.decode(Instruction::CMP_H),
            0xBD => self.decode(Instruction::CMP_L),
            0xBE => self.decode(Instruction::CMP_M),
            0xBF => self.decode(Instruction::CMP_A),

            0xC0 => self.decode(Instruction::RNZ),
            0xC1 => self.decode(Instruction::INR_E),
            0xCA => self.decode(Instruction::XRA_H),
            0xC2 => self.decode(Instruction::RNZ),

            0xC3 => self.decode(Instruction::JMP),
            0xC5 => self.decode(Instruction::PUSH_B),

            0xC8 => self.decode(Instruction::RZ),
            0xCB => self.decode(Instruction::JMP),
            0xCD => self.decode(Instruction::CALL),
            0xCF => self.decode(Instruction::CM),
            0xD => self.decode(Instruction::DCR_C),
            0xD1 => self.decode(Instruction::POP_D),

            0xE => self.decode(Instruction::MVI_C),
            0xEA => self.decode(Instruction::XRA_A),
            0xE6 => self.decode(Instruction::ANI),

            0x3A => self.decode(Instruction::ANA_E),
            0x3C => self.decode(Instruction::JMP),
            0x3D => self.decode(Instruction::OUT),

            0x3E => self.decode(Instruction::MVI_A),

            0x32 => self.decode(Instruction::INX_H),
            0x33 => self.decode(Instruction::INX_SP),
            0x35 => self.decode(Instruction::MOV_D_E),
            0xBF => self.decode(Instruction::EI),
            0xDA => self.decode(Instruction::XRA_L),
            0xD3 => self.decode(Instruction::DCR_A),
            0xD8 => self.decode(Instruction::ADC_L),
            0xFA => self.decode(Instruction::JM),
            0xFE => self.nop(),
            0xE9 => self.decode(Instruction::RPE),
            0xEB => self.decode(Instruction::CMP_M),
            0xEF => self.decode(Instruction::CPI),

            // Instructions from 0x4A - 0x4F to 0x7A to 0x7F are MOV instructions
            0x47 => self.decode(Instruction::MOV_M_H),
            0x49 => self.decode(Instruction::SUB_H),
            0x59 => self.decode(Instruction::SUB_L),
            0x6F => self.decode(Instruction::MOV_L_A),
            0x67 => self.decode(Instruction::HLT),
            0x72 => self.decode(Instruction::DAA),
            0x74 => self.nop(),
            0x75 => self.decode(Instruction::MOV_D_A),

            // ADD instructions
            0x80 => self.decode(Instruction::ADD_B),
            0x81 => self.decode(Instruction::ADD_C),
            0x82 => self.decode(Instruction::ADD_D),
            0x83 => self.decode(Instruction::ADD_E),
            0x84 => self.decode(Instruction::ADD_H),
            0x85 => self.decode(Instruction::ADD_L),
            0x86 => self.decode(Instruction::ADD_M),

            0x87 => self.decode(Instruction::ADD_A),
            0x88 => self.decode(Instruction::ADC_B),
            0x89 => self.decode(Instruction::ADC_C),
            0xF => self.decode(Instruction::RRC),
            0xF3 => self.decode(Instruction::CMC),
            0xFF => self.decode(Instruction::RST_7),

            _ => println!("Unknown opcode: {:X}", self.opcode),
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

        self.sign = 0;
        self.zero = 0;
        self.parity = 0;

        self.carry = 0;
        self.half_carry = 0;
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
