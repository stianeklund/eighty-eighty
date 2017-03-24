#[derive(Debug)]
pub struct Opcode;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum Register {
    A, B, C, D, E, H, L, M
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd)]
pub enum RegisterPair {
    BC, DE, HL
}


#[derive(Debug, Clone, Eq, PartialEq, PartialOrd)]
pub enum Instruction {
    NOP,
    ADC(Register),
    ADD(Register),
    ACI,
    ADI,
    ANA(Register),
    ANI,
    CALL(u16),
    CC(u16),
    CNZ,
    CMA,
    CNC,
    CMC,
    CMP(Register),
    CPO,
    CZ,
    DAA,
    DAD(Register),
    DCR(Register),
    HLT,
    INX(Register),
    INR(Register),
    JC,
    JNC,
    JNZ,
    JZ,
    JMP,
    JPO,

    LDA(Register),
    LDAX(Register),
    LXI(Register),
    MOV(Register, Register),
    MOV_R_PR(RegisterPair, Register),
    MOV_R_RP(Register, RegisterPair),
    MVI(Register),
    RAR,
    RC,
    RET,
    RNC,
    RLC,
    RIM,
    RST,
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
    STAX(Register),
    POP(Register),
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
    NOP_9,
    XRI,
    RST_5,
    RP,
    POP_PSW,
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
    MOV_B_B,
    MOV_B_C,
    MOV_B_D,
    MOV_B_E,
    MOV_B_H,
    MOV_B_L,
    MOV_B_M,
    MOV_B_A,
    MOV_C_B,
    MOV_C_C,
    MOV_C_D,
    MOV_C_E,
    MOV_C_H,
    MOV_C_L,
    MOV_C_M,
    MOV_C_A,
    MOV_D_B,
    MOV_D_C,
    MOV_D_D,
    MOV_D_E,
    MOV_D_H,
    MOV_D_L,
    MOV_D_M,
    MOV_D_A,
    MOV_E_B,
    MOV_E_C,
    MOV_E_D,
    MOV_E_E,
    MOV_E_H,
    MOV_E_L,
    MOV_E_M,
    MOV_E_A,
    MOV_H_B,
    MOV_H_C,
    MOV_H_D,
    MOV_H_E,
    MOV_H_H,
    MOV_H_L,
    MOV_H_M,
    MOV_H_A,
    MOV_L_B,
    MOV_L_C,
    MOV_L_D,
    MOV_L_E,
    MOV_L_H,
    MOV_L_L,
    MOV_L_M,
    MOV_L_A,
    MOV_M_B,
    MOV_M_C,
    MOV_M_D,
    MOV_M_E,
    MOV_M_H,
    MOV_M_L,
    MOV_M_A,
    MOV_A_B,
    MOV_A_C,
    MOV_A_D,
    MOV_A_E,
    MOV_A_H,
    MOV_A_L,
    MOV_A_M,
    MOV_A_A,
    ADD_B,
    ADD_C,
    ADD_D,
    ADD_E,
    ADD_H,
    ADD_L,
    ADD_M,
    ADD_A,
    ADC_B,
    ADC_C,
    ADC_D,
    ADC_E,
    ADC_H,
    ADC_L,
    ADC_M,
    ADC_A,
    SUB_B,
    SUB_C,
    SUB_D,
    SUB_E,
    SUB_H,
    SUB_L,
    SUB_M,
    SUB_A,
    SBB_B,
    SBB_C,
    SBB_D,
    SBB_E,
    SBB_H,
    SBB_L,
    SBB_M,
    SBB_A,
    ANA_B,
    ANA_C,
    ANA_D,
    ANA_E,
    ANA_H,
    ANA_L,
    ANA_M,
    ANA_A,
    XRA_B,
    XRA_C,
    XRA_D,
    XRA_E,
    XRA_H,
    XRA_L,
    XRA_M,
    XRA_A,
    ORA_B,
    ORA_C,
    ORA_D,
    ORA_E,
    ORA_H,
    ORA_L,
    ORA_M,
    ORA_A,
    CMP_B,
    CMP_C,
    CMP_D,
    CMP_E,
    CMP_H,
    CMP_L,
    CMP_M,
    CMP_A,
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
