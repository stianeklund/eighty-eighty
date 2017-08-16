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

#[allow(allow_snake_case)]
#[allow(non_camel_case)]
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Instruction {
    NOP,
    ACI,
    ADC(Register),
    ADI,
    ADD(Register),
    ANA(Register),
    ANI,
    CALL(u16),
    CC,
    CNZ,
    CMA,
    CNC,
    CMC,
    CMP(Register),
    CPO,
    CZ,
    DAA,
    DAD(RegisterPair),
    DAD_SP,
    DCR(Register),
    DCX(RegisterPair),
    DCX_SP,
    HLT,
    INX(RegisterPair),
    INX_SP,
    INR(Register),
    JC,
    JNC,
    JNZ,
    JZ,
    JMP,
    JPO,
    LDA,
    LDAX(RegisterPair),
    LXI(RegisterPair),
    LXI_SP,
    MOV(Register, Register),
    MOV_RP(RegisterPair, Register),
    MVI(Register),
    RAR,
    RC,
    RET,
    RNC,
    RLC,
    RIM,
    RST(u8),
    RPO,
    RPE,
    RNZ,
    RZ,
    SBI,
    SIM,
    SHLD,
    SUB(Register),
    SUI,
    SBB(Register),
    STAX(RegisterPair),
    POP(RegisterPair),
    PUSH(Register),
    XRA(Register),
    ORA(Register),
    OUT,
    IN,
    XTHL,
    PCHL,
    JPE,
    XCHG,
    CPE,
    XRI,
    RP,
    POP_PSW(Register),
    JP,
    DI,
    CP,
    ORI,
    RM,
    SPHL,
    JM,
    EI,
    CM,
    CPI,
    RRC,
    RAL,
    LHLD,
    STA,
    STC,
}
