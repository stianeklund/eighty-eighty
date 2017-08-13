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
    DCR(Register),
    DCX(RegisterPair),
    HLT,
    INX(RegisterPair),
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
    PUSH_PSW,
    ORI,
    RST_6,
    RM,
    SPHL,
    JM,
    EI,
    CM,
    NOP_10,
    CPI,
    RST_7,
    LXI_B,
    STAX_B,
    INX_B,
    INR_B,
    DCR_B,
    MVI_B,
    NOP_1,
    DAD_B,
    LDAX_B,
    DCX_B,
    INR_C,
    DCR_C,
    MVI_C,
    RRC,
    NOP_2,
    LXI_D,
    STAX_D,
    INX_D,
    INR_D,
    DCR_D,
    MVI_D,
    RAL,
    NOP_3,
    DAD_D,
    LDAX_E,
    DCX_D,
    INR_E,
    DCR_E,
    MVI_E,
    LXI_H,
    INX_H,
    INR_H,
    DCR_H,
    MVI_H,
    NOP_4,
    DAD_H,
    LHLD,
    DCX_H,
    INR_L,
    DCR_L,
    MVI_L,
    LXI_SP,
    STA,
    INX_SP,
    INR_M,
    DCR_M,
    MVI_M,
    STC,
    NOP_5,
    DAD_SP,
    DCX_SP,
    INR_A,
    DCR_A,
    MVI_A,
    POP_B,
    PUSH_B,
    RST_0,
    NOP_6,
    RST_1,
    POP_D,
    PUSH_D,
    RST_2,
    NOP_7,
    NOP_8,
    RST_3,
    POP_H,
    PUSH_H,
    RST_4,
}
