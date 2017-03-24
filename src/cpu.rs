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
    reg_m: u8, // psuedo register

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
            reg_m: 0,

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
            Register::M => self.reg_m,
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
            Register::M => self.reg_m = value,
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

    fn adv_pc(&mut self) {
        self.pc += 1;
    }

    pub fn adc(&mut self, reg: Register) {
        self.pc += 1;

    }
    pub fn add(&mut self, reg: Register) {
        match reg {
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
        if DEBUG { println!(" LXI on reg: {:?}", reg);}
        let mut reg = self.read_reg(reg);
        if DEBUG { println!(" read_reg: {:X}", reg);}
        reg = self.memory[self.pc as usize + 1];
        if DEBUG { println!(" reg: {:X}", reg);}

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
    pub fn call(&mut self, addr: u16) {
        match self.opcode {
            0xCD => {
                let ret = self.pc + 2;
                self.memory[self.sp as usize - 1] = (ret >> 8 & 0xFF) as u8;
            },
            _ => println!("Unknown call address"),
        }
        self.pc = addr;
    }

    // TODO
    pub fn cmp(&mut self, reg: Register) {
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

    // TODO
    pub fn mvi(&mut self, reg: Register) {
        self.pc += 1;
    }


    // TODO Increment Register
    pub fn inr(&mut self, reg: Register) {
        match reg {
            Register::A => self.write_reg(Register::A, 1),
            Register::B => self.write_reg(Register::B, 1),
            Register::C => self.write_reg(Register::C, 1),
            Register::D => self.write_reg(Register::D, 1),
            Register::E => self.write_reg(Register::D, 1),
            Register::H => self.write_reg(Register::D, 1),
            Register::L => self.write_reg(Register::D, 1),
            Register::M => self.write_reg(Register::M, 1),
        }
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

    // TODO SBB
    pub fn sbb(&mut self, reg: Register) {
        self.pc += 1;
    }
    // TODO SUB
    pub fn sub(&mut self, reg: Register) {
        self.pc += 1;
    }
    // XRA Logical Exclusive-Or memory with Accumulator (Zero accumulator)
    pub fn xra(&mut self, reg: Register) {
        self.pc += 1;
    }

    // TODO
    fn rpe(&mut self) {
        self.pc += 1;
    }
    fn rst(&mut self, instruction: Instruction) {

       match self.opcode  {
           0xC7 | 0xE7 |  0xF7 | 0xCF | 0xDF | 0xEF | 0xFF  => {
               self.memory[(self.sp as usize -1) & 0xFFFF] = (self.reg_h) & 0xFF;
               self.memory[(self.sp as usize - 2) & 0xFFFF] = (self.reg_l) & 0xFF;
               self.sp -= 2;
               self.pc = (self.opcode & 0x38) as u16;
           },
           _ => println!("Unknown instruction: {:?}", instruction),
       }
    }

    // TODO
    pub fn out(&mut self) {
        self.pc += 1;
    }

    pub fn mov(&mut self,src: Register, dst: Register) {
        let value = self.read_reg(src);
        self.write_reg(dst, value);
        if DEBUG { println!("Read reg value: {}, src: {:?}, dst:{:?}", value, src, dst)}
    }

    // I think it might be a good idea to segment instructions based on functionality.
    // Move logical operations to a separate file, jump & call to one etc?
    pub fn decode(&mut self, instruction: Instruction) {
        use self::Register::*;

        match instruction {
            Instruction::NOP => self.adv_pc(),
            Instruction::ACI => self.aci(),

            Instruction::ADD(reg) => self.add(reg),
            Instruction::ADI => self.adv_pc(),
            Instruction::ADC(reg) => self.adc(reg),
            Instruction::ANA(reg) => self.ana(reg),

            Instruction::INR(reg) => self.inr(reg),
            Instruction::CALL(addr) => self.call(addr),
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
            Instruction::RPE => self.rpe(),

            Instruction::PUSH_D => self.push_d(),

            Instruction::INX_H => self.inx_h(),
            Instruction::INX_SP => self.inx_sp(),
            Instruction::INR_B => self.inr(B),
            Instruction::OUT => self.out(),
            Instruction::ANI => self.ani(),
            Instruction::STAX_D => self.stax_d(),
            Instruction::LXI(reg) => self.lxi(D),
            Instruction::LXI_SP => self.lxi_sp(),
            Instruction::RST_7 => self.adv_pc(), // self.reset(),
            Instruction::INR_E => self.inr_e(),
            Instruction::CM => self.adv_pc(), // TODO
            Instruction::CMC => self.adv_pc(), // TODO
            Instruction::HLT => self.adv_pc(), // self.reset(),
            Instruction::RRC => self.adv_pc(), // TODO
            Instruction::XRA_L => self.adv_pc(), // TODO

            Instruction::ORA(reg) => self.adv_pc(), // TODO


            Instruction::POP(reg) => self.adv_pc(), // TODO
            Instruction::POP_PSW => self.adv_pc(), // TODO

            Instruction::JNZ => self.adv_pc(), // TODO
            Instruction::JM => self.adv_pc(), // TODO

            _ => println!("Unknown instruction {:X}", self.opcode),
        }
        self.pc += 1;

    }
    pub fn execute_instruction(&mut self) {
        self.opcode = self.memory[self.pc as usize];

       use self::Register::*;
        use self::RegisterPair::*;

        if DEBUG { println!("Opcode: 0x{:X}, PC: {}, SP: {}", self.opcode, self.pc, self.sp); }

        match self.opcode {
            // NOP Instruction: Do nothing
            0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => {
                self.decode(Instruction::NOP);
            },

            0x01 => self.decode(Instruction::NOP),
            0x02 => self.decode(Instruction::STA),
            0x03 => self.decode(Instruction::INR_B),

            0x05 => self.decode(Instruction::DCR_B),
            0x06 => self.decode(Instruction::MVI_B),
            0x07 =>self.decode(Instruction::RLC),
            0x7E => self.decode(Instruction::RST_4),
            0x11 => self.decode(Instruction::LXI(D)),
            0x13 => self.decode(Instruction::LXI_SP),
            0x14 => self.decode(Instruction::INR_D),

            0x15 => self.decode(Instruction::MOV(D, C)),

            0x16 => self.decode(Instruction::MOV(H, C)),
            // 0x17 => self.decode(Instruction::MOV(HL, C)),
            0x17 => self.decode(Instruction::MOV_R_PR(HL, C)),
            0x19 => self.decode(Instruction::SUB(C)),
            0x1C => self.decode(Instruction::POP(B)),
            0x1D => self.decode(Instruction::POP(D)),
            0x2A => self.decode(Instruction::ANA(D)),
            0x2C => self.decode(Instruction::JNZ),

            0x21 => self.decode(Instruction::STAX_D),
            0x23 => self.decode(Instruction::STA),
            0x26 => self.decode(Instruction::MOV(H, D)),
            0x37 => self.decode(Instruction::MOV(M, E)),
            0x39 => self.decode(Instruction::SUB(E)),
            0x3B => self.decode(Instruction::ORA(E)),

            0x4 => self.decode(Instruction::INR_B),
            0x44 => self.decode(Instruction::MOV(B, H)),
            0x4C => self.decode(Instruction::CNZ),
            0x4E => self.decode(Instruction::CPO),
            0x4F => self.decode(Instruction::CP),
            0x5D => self.decode(Instruction::PUSH_D),
            0x9 => self.decode(Instruction::SUB(B)),
            0xA => self.decode(Instruction::ANA(B)),
            0xA3 => self.decode(Instruction::ANA(E)),
            0xAF => self.decode(Instruction::XRA_A),
            0xA7 => self.decode(Instruction::MOV_A_D),

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
            0xC2 => self.decode(Instruction::RNZ),
            0xC3 => self.decode(Instruction::JMP),
            0xC5 => self.decode(Instruction::PUSH_B),
            0xC7 => self.decode(Instruction::MOV(A, H)),
            0xC8 => self.decode(Instruction::RZ),
            0xC9 => self.decode(Instruction::SBB(H)),
            0xCA => self.decode(Instruction::XRA_H),
            0xCB => self.decode(Instruction::JMP),
            0xCD => self.decode(Instruction::CC(0xBEEF)),
            0xCF => self.decode(Instruction::CM),
            0xD => self.decode(Instruction::DCR_C),
            0xD1 => self.decode(Instruction::POP_D),
            0xD9 => self.decode(Instruction::SBB(L)),

            0xE => self.decode(Instruction::MVI_C),
            0xEA => self.decode(Instruction::XRA_A),
            0xE6 => self.decode(Instruction::ANI),

            0x3A => self.decode(Instruction::ANA_E),
            0x3C => self.decode(Instruction::JMP),
            0x3D => self.decode(Instruction::OUT),

            0x3E => self.decode(Instruction::MVI_A),

            0x32 => self.decode(Instruction::INX_H),
            0x33 => self.decode(Instruction::INX_SP),
            0x34 => self.decode(Instruction::MOV(B, E)),
            0x35 => self.decode(Instruction::MOV(D, E)),
            0xDA => self.decode(Instruction::XRA_L),
            0xD3 => self.decode(Instruction::DCR_A),
            0xD8 => self.decode(Instruction::ADC(L)),
            0xFA => self.decode(Instruction::JM),
            0xFE => self.adv_pc(),
            0xE9 => self.decode(Instruction::RPE),
            0xEB => self.decode(Instruction::CMP_M),
            0xEF => self.decode(Instruction::CPI),

            // Instructions from 0x4A - 0x4F to 0x7A to 0x7F are MOV instructions
            0x47 => self.decode(Instruction::MOV(M, H)),
            0x49 => self.decode(Instruction::SUB_H),
            0x56 => self.decode(Instruction::MOV(H, M)),
            0x59 => self.decode(Instruction::SUB_L),
            0x6C => self.decode(Instruction::ADI),
            0x6F => self.decode(Instruction::MOV_L_A),
            0x67 => self.decode(Instruction::HLT),
            0x72 => self.decode(Instruction::DAA),
            0x74 => self.adv_pc(),
            0x75 => self.decode(Instruction::MOV_D_A),

            // ADD instructions
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
            0x93 => self.decode(Instruction::DAD(H)),
            0x98 => self.decode(Instruction::ADC(C)),
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
