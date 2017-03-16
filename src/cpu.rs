use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    // 8-bit Registers
    a_reg: u8,
    b_reg: u8,
    c_reg: u8,
    d_reg: u8,
    e_reg: u8,
    h_reg: u8,
    l_reg: u8,

    // 16-bit Register pairs
    bc_reg: u16,
    de_reg: u16,
    hl_reg: u16,

    // Status Register (Flags)
    sign: u8,
    zero: u8,
    parity: u8,

    carry: u8,
    half_carry: u8,

    psw_reg: u16,

    interrupt: u8,
    interrupt_addr: u16,

    opcode: u16,
    pub memory: Box<[u8; 65536]>,

    pc: u16,
    sp: u16,

}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            a_reg: 0,
            b_reg: 0,
            c_reg: 0,
            d_reg: 0,
            e_reg: 0,
            h_reg: 0,
            l_reg: 0,

            bc_reg: 0,
            de_reg: 0,
            hl_reg: 0,

            sign: 0,
            zero: 0,
            parity: 0,

            carry: 0,
            half_carry: 0,

            psw_reg: 0,

            interrupt: 0,
            interrupt_addr: 0,

            opcode: 0,
            memory: Box::new([0; 65536]),

            pc: 0,
            sp: 0,
        }
    }


    // TODO: Implement separate functions to fetch, read & write 16 bit (word) & 8 bit integers
    //  fn read_word(addr + 1) << 8) |
    // Read low byte (word): fn read_byte[addr & 0xffff];
    // This will make the whole match tree smaller & easier to read.


    // Set stack pointer value
    pub fn set_sp(&mut self, byte: u16) {
        self.sp = byte & 0xFFFF;
    }

    pub fn read_byte(&mut self, addr: u8) -> u8 {
        self.memory[addr as usize & 0xFFFF]
    }


    pub fn read_word(&mut self, addr: u8) -> u16 {
        (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16)
    }

    pub fn execute_instruction(&mut self) {
        let opcode = self.memory[self.pc as usize];
        // (self.memory[self.pc as usize] as u16) << 8 | (self.memory[self.pc as usize + 1] as u16)

        println!("Opcode: {:X}, PC: {}, SP: {}", opcode, self.pc, self.sp);
        match opcode {
            0x00 => {
                // NOP
                self.pc += 1;
            },
            0x08 => {
                // NOP
                self.pc += 1;
            },
            0x10 => {
                self.pc += 1;
            },
            0x18 => {
                self.pc += 1
            },
            0x20 => {
                self.pc += 1;
            },
            0x28 => {
                self.pc += 1;
            }
            0x30 => {
                self.pc += 1;
            },

            0x38 => {
                self.pc += 1;
            },
            0x01 => {
                // LXI (Load SP with immediate 16-bit value)
                self.sp = (self.memory[self.pc as usize] as u16) << 8 |
                (self.memory[self.pc as usize + 1] as u16);
                self.pc += 2;
            },
            0x02 => {
                // TODO: Write byte
            }
            0x3E => {
                self.memory[self.pc as usize + 1] as u16;
            },
            0x06 => {
                // TODO
            },
            _ => return
        }
    }

    pub fn reset(&mut self) {
        println!("Resetting emulator");

        self.a_reg = 0;
        self.b_reg = 0;
        self.c_reg = 0;
        self.d_reg = 0;
        self.e_reg = 0;
        self.h_reg = 0;
        self.l_reg = 0;

        self.bc_reg = 0;
        self.de_reg = 0;
        self.hl_reg = 0;

        self.sign = 0;
        self.zero = 0;
        self.parity = 0;

        self.carry = 0;
        self.half_carry = 0;
        self.psw_reg = 0;

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

