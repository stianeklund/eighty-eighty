use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use opcode::Instruction;

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

    pub fn byte_to_u16(byte: u8, word: u8) -> u16 {
        (((word as u8) as u16) << 8) | ((byte as u8) as u16)
    }

    // TODO Move this out of cpu.rs & into separate file for cleaner memory management.
    // Make interconnect for memory management?
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



    // Instruction functions, possible improvement is to group functions
    // by type or separate them by type, e.g mov goes together.

    pub fn nop(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn ana_e(&mut self) {
        self.pc += 1;
    }

    // TODO
    pub fn ana_b(&mut self) {
        self.pc += 1;
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

        self.pc += 2;
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

    // TODO
    pub fn dcr_b(&mut self) {
        self.reg_b -= self.reg_b;
        self.pc += 1;
    }
    // TODO
    pub fn dcr_a(&mut self) {
        self.reg_d -= self.reg_d;
        self.pc += 1;
    }
    // TODO
    pub fn dcr_c(&mut self) {
        self.reg_c -= self.reg_c;
        self.pc += 1;
    }

    // TODO
    pub fn ei(&mut self) {
        self.pc += 1;
    }

    // TODO I believe zero & sign flags need to be set for SUB instructions.
    pub fn sub_c(&mut self) {
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
    pub fn move_la(&mut self) {
        self.pc += 1;
    }

    // Increment BC
    pub fn inc_bc(&mut self) {
        self.reg_bc += self.reg_bc;
        self.pc += 1;
    }

    // Increment PC
    pub fn inc_pc(&mut self, amount: u16) {
        self.pc += amount;
    }

    // TODO
    pub fn inx_h(&mut self) {
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

    pub fn decode(&mut self, instr: Instruction) {

        match instr {
            Instruction::NOP => self.nop(),
            Instruction::ACI => self.aci(),
            Instruction::ANA_E => self.ana_e(),
            Instruction::ANA_B => self.ana_b(),
            Instruction::INC_B => self.inc_bc(),
            Instruction::CALL => self.call(),
            Instruction::CPI => self.cpi(),
            Instruction::CMP_M => self.cmp_m(),
            Instruction::DCR_A => self.dcr_a(),
            Instruction::DCR_B => self.dcr_b(),
            Instruction::EI => self.ei(),
            Instruction::JMP =>  self.jmp(),
            Instruction::RNZ => self.rnz(),
            Instruction::RZ => self.rz(),
            Instruction::MOV_M_H => self.move_mh(),
            Instruction::MOV_A_D => self.move_ad(),
            Instruction::MOV_L_A => self.move_la(),
            Instruction::MVI_A => self.mvi_a(),
            Instruction::SUB_C => self.sub_c(),
            Instruction::XRA_A => self.xra_a(),
            Instruction::XRA_M => self.xra_m(),
            Instruction::XRA_H => self.xra_h(),
            Instruction::RPE => self.rpe(),
            Instruction::PUSH_D => self.push_d(),
            Instruction::INX_H => self.inx_h(),
            Instruction::OUT => self.out(),


            _ => println!("Unknown instruction {:X}", self.opcode),
        }

    }
    pub fn execute_instruction(&mut self) {
        self.opcode = self.memory[self.pc as usize];

        if DEBUG { println!("Opcode: 0x{:X}, PC: {}, SP: {}", self.opcode, self.pc, self.sp); }

        match self.opcode {
            // NOP Instructions: Do nothing
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                self.decode(Instruction::NOP);
            },

            0x01 => self.decode(Instruction::NOP),
            0x02 => self.decode(Instruction::STA),
            0x03 => self.decode(Instruction::INC_B),

            0x05 => self.decode(Instruction::DCR_B),
            0x06 => self.decode(Instruction::MVI_B),
            0x14 => self.decode(Instruction::INC_D),
            0x19 => self.decode(Instruction::SUB_C),

            0x5D => self.decode(Instruction::PUSH_D),
            0xA => self.decode(Instruction::ANA_B),
            0xAF => self.decode(Instruction::XRA_A),
            0xA7 => self.decode(Instruction::MOV_A_D),
            0xEA => self.decode(Instruction::XRA_A),
            0xCA => self.decode(Instruction::XRA_H),
            0xC2 => self.decode(Instruction::RNZ),

            0xC3 => self.decode(Instruction::JMP),
            0xC5 => self.decode(Instruction::PUSH_B),

            0xC8 => self.decode(Instruction::RZ),
            0xCD => self.decode(Instruction::CALL),
            0xD => self.decode(Instruction::DCR_C),
            0x3A => self.decode(Instruction::ANA_E),
            0x3C => self.decode(Instruction::JMP),
            0x3D => self.decode(Instruction::OUT),

            0x3E => self.decode(Instruction::MVI_A),

            0x32 => self.decode(Instruction::INX_H),
            0xBF => self.decode(Instruction::EI),
            0xD3 => self.decode(Instruction::DCR_A),
            0xFE => self.nop(),
            0xE9 => self.decode(Instruction::RPE),
            0xEB => self.decode(Instruction::CMP_M),
            0xEF => self.decode(Instruction::CPI),

            // Instructions from 0x4A - 0x4F to 0x7A to 0x7F are MOV instructions
            0x47 => self.decode(Instruction::MOV_M_H),
            0x6F => self.decode(Instruction::MOV_L_A),
            0x74 => self.nop(),
            0x82 => self.decode(Instruction::NOP),

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
