use std::fmt;


#[derive(Debug)]
pub struct Opcode;

impl fmt::UpperHex for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:X}", val)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum RegisterPair {
    BC,
    DE,
    HL,
    SP,
}

impl fmt::UpperHex for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:X}", val)
    }
}

impl fmt::UpperHex for RegisterPair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:X}", val)
    }
}

/*
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Instruction {
    Nop = 0x00,
    Aci = 0xCE,
    Adc = 0x88 | 0x89 | 0x8A | 0x8B | 0xC | 0x8D | 0x8E | 0x8F,
    Adi = 0xC6,
    Add = 0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x86 | 0x87,
    Ana = 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 | 0xA6 | 0xA7,
    Ani = 0xE6,
    Call = 0xCD,
    MviC = 0x0E,
    MviA = 0x3E,
    MviL = 0x2E,
    MviE = 0x1E,
    /*
    Cc(u16),
    Cnz(u16),
    Cma,
    Cnc(u16),
    Cm(u16),
    Cmc,
    Cmp(Register),
    Cp(u16),
    Cpe(u16),
    Cpo(u16),
    Cz(u16),
    Daa,
    Dad(RegisterPair),
    DadSp(RegisterPair),
    Dcr(Register),
    Dcx(RegisterPair),
    DcxSp(RegisterPair),
    Hlt,
    Inx(RegisterPair),
    InxSp(RegisterPair),
    Inr(Register),
    Jc,
    Jnc,
    Jnz,
    Jz,
    Jmp,
    Jpo,
    Lda,
    Ldax(RegisterPair),
    Lxi(RegisterPair),
    LxiSp,
    Mov(Register, Register),
    MovRp(RegisterPair, Register),
    Mvi(Register),
    Rar,
    Rc,
    Ret,
    Rnc,
    Rlc,
    Rim,
    Rst(u8),
    Rpo,
    Rpe,
    Rnz,
    Rz,
    Sbi,
    Sim,
    Shld,
    Sub(Register),
    Sui,
    Sbb(Register),
    Stax(RegisterPair),
    Pop(RegisterPair),
    Push(Register),
    PushPsw(),
    Xra(Register),
    Ora(Register),
    Out,
    In,
    Xthl,
    Pchl,
    Jpe,
    Xchg,
    Xri,
    Rp,
    PopPsw(Register),
    Jp,
    Di,
    Ori,
    Rm,
    Sphl,
    Jm,
    Ei,
    Cpi,
    Rrc,
    Ral,
    Lhld,
    Sta,
    Stc,
}*/

// TODO Figure out something smart here
/* pub fn match_instruction(instruction: usize) -> String {
    match instruction {
        0x00 =>
        0xCE =>
        _ => println!("Nothing"),
    }
    Adc = 0x88 | 0x89 | 0x8A | 0x8B | 0xC | 0x8D | 0x8E | 0x8F,
    Adi = 0xC6,
    Add = 0x80 | 0x81 | 0x82 | 0x83 | 0x84 | 0x85 | 0x86 | 0x87,
    Ana = 0xA0 | 0xA1 | 0xA2 | 0xA3 | 0xA4 | 0xA5 | 0xA6 | 0xA7,
    Ani = 0xE6,
    Call = 0xCD,
    MviC = 0x0E,
    MviA = 0x3E,
    MviL = 0x2E,
    MviE = 0x1E,
    Cc(u16),
    Cnz(u16),
    Cma,
    Cnc(u16),
    Cm(u16),
    Cmc,
    Cmp(Register),
    Cp(u16),
    Cpe(u16),
    Cpo(u16),
    Cz(u16),
    Daa,
    Dad(RegisterPair),
    DadSp(RegisterPair),
    Dcr(Register),
    Dcx(RegisterPair),
    DcxSp(RegisterPair),
    Hlt,
    Inx(RegisterPair),
    InxSp(RegisterPair),
    Inr(Register),
    Jc,
    Jnc,
    Jnz,
    Jz,
    Jmp,
    Jpo,
    Lda,
    Ldax(RegisterPair),
    Lxi(RegisterPair),
    LxiSp,
    Mov(Register, Register),
    MovRp(RegisterPair, Register),
    Mvi(Register),
    Rar,
    Rc,
    Ret,
    Rnc,
    Rlc,
    Rim,
    Rst(u8),
    Rpo,
    Rpe,
    Rnz,
    Rz,
    Sbi,
    Sim,
    Shld,
    Sub(Register),
    Sui,
    Sbb(Register),
    Stax(RegisterPair),
    Pop(RegisterPair),
    Push(Register),
    PushPsw(),
    Xra(Register),
    Ora(Register),
    Out,
    In,
    Xthl,
    Pchl,
    Jpe,
    Xchg,
    Xri,
    Rp,
    PopPsw(Register),
    Jp,
    Di,
    Ori,
    Rm,
    Sphl,
    Jm,
    Ei,
    Cpi,
    Rrc,
    Ral,
    Lhld,
    Sta,
    Stc,
}
*/
*/

