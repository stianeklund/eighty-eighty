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

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Instruction {
    Nop,
    Aci,
    Adc(Register),
    Adi,
    Add(Register),
    Ana(Register),
    Ani,
    Call(u16),
    Cc,
    Cnz,
    Cma,
    Cnc,
    Cmc,
    Cmp(Register),
    Cpo,
    Cz,
    Daa,
    Dad(RegisterPair),
    DadSp,
    Dcr(Register),
    Dcx(RegisterPair),
    DcxSp,
    Hlt,
    Inx(RegisterPair),
    InxSp,
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
    Xra(Register),
    Ora(Register),
    Out,
    In,
    Xthl,
    Pchl,
    Jpe,
    Xchg,
    Cpe,
    Xri,
    Rp,
    PopPsw(Register),
    Jp,
    Di,
    Cp,
    Ori,
    Rm,
    Sphl,
    Jm,
    Ei,
    Cm,
    Cpi,
    Rrc,
    Ral,
    Lhld,
    Sta,
    Stc,
}
