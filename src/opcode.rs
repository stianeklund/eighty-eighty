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
