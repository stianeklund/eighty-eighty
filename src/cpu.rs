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

    memory: Box<[u8; 65536]>,
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

    // Instruction functions

    pub fn nop(&mut self) {
        self.pc += 1;
    }

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

    pub fn dcr_b(&mut self) {
        // TODO
        self.reg_b -= self.reg_b;
        self.pc += 1;
    }

    pub fn dcr_c(&mut self) {
        // TODO
        self.reg_c -= self.reg_c;
        self.pc += 1;
    }

    pub fn rnz(&mut self) {
        // TODO
        self.pc += 1;
    }

    pub fn rz(&mut self) {
        // TODO
        self.pc += 1;
    }

    pub fn move_mh(&mut self) {
        // TODO
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

    // PUSH B register
    pub fn push_b(&mut self) {
        self.sp.wrapping_sub(2);
        let sp = self.sp;
        let b = self.reg_b;
        self.write_word(b, sp);
        self.pc += 1;
    }


    pub fn decode(&mut self, instr: Instruction) {

        match instr {
            Instruction::NOP => self.nop(),
            Instruction::ACI => self.aci(),
            Instruction::INC_B => self.inc_bc(),
            Instruction::DCR_B => self.dcr_b(),
            Instruction::JMP =>  self.jmp(),
            Instruction::RNZ => self.rnz(),
            Instruction::RZ => self.rz(),
            Instruction::MOV_M_H => self.move_mh(),



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

            0xC2 => self.decode(Instruction::RNZ),

            0xC3 => self.decode(Instruction::JMP),
            0xC5 => self.decode(Instruction::PUSH_B),

            0xC8 => self.decode(Instruction::RZ),
            0xCD => self.decode(Instruction::CALL),
            0xD => self.decode(Instruction::DCR_C),
            0x3C => self.decode(Instruction::JMP),
            0x3D => self.nop(),

            0x3E => self.mvi_a(),

            0x32 => self.nop(),
            0xD3 => self.nop(),
            0xFE => self.nop(),
            0x47 => self.decode(Instruction::MOV_M_H),
            0x74 => self.nop(),

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
