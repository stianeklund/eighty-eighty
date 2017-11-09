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
impl Opcode {
    pub fn print(opcode: u8) -> &'static str {
        match opcode {
            0x00 | 0x08 => "NOP       ",
            0x01 => "LXI BC    ",
            0x02 => "STAX BC   ",
            0x03 => "INX BC    ",
            0x04 => "INR B     ",
            0x05 => "DCR B     ",
            0x06 => "MVI B     ",
            0x07 => "RLC       ",
            0x09 => "DAD BC    ",

            0x0A => "LDAX BC   ",
            0x0B => "DCX BC    ",
            0x0C => "INR C     ",
            0x0D => "DCR D     ",
            0x0E => "MVI C     ",
            0x0F => "RRC       ",

            0x10 => "NOP       ",
            0x11 => "LXI DE    ",
            0x12 => "STAX DE   ",
            0x13 => "INX DE    ",
            0x14 => "INR D     ",
            0x15 => "DCR D     ",
            0x16 => "MVI D     ",

            0x17 => "RAL       ",
            0x18 => "NOP       ",
            0x19 => "DAD DE    ",
            0x1A => "LDAX DE   ",
            0x1B => "DCX DE    ",
            0x1C => "INR E     ",
            0x1D => "DCR E     ",
            0x1E => "MVI E     ",
            0x1F => "RAR       ",
            0x20 => "NOP       ",
            0x21 => "LXI HL    ",
            0x22 => "SHLD      ",
            0x23 => "INX HL    ",
            0x24 => "INR H     ",
            0x25 => "DCR H     ",
            0x26 => "MVI H     ",
            0x27 => "DAA       ",
            0x28 => "NOP       ",
            0x29 => "DAD HL    ",

            0x2A => "LHLD      ",
            0x2B => "DCX HL    ",
            0x2C => "INR L     ",
            0x2D => "DCR L     ",
            0x2E => "MVI L     ",
            0x2F => "CMA       ",

            0x30 => "NOP       ",
            0x31 => "LXI SP    ",
            0x32 => "STA       ",
            0x33 => "INX SP    ",
            0x34 => "INR M     ",
            0x35 => "DCR M     ",
            0x36 => "MVI M     ",
            0x37 => "STC       ",
            0x38 => "NOP       ",
            0x39 => "DAD SP    ",

            0x3A => "LDA       ",
            0x3B => "DCX SP    ",
            0x3C => "INR A     ",
            0x3D => "DCR A     ",
            0x3E => "MVI A     ",
            0x3F => "CMC       ",

            // MOV Instructions 0x40 - 0x7F
            0x40 => "MOV (B, B)",
            0x41 => "MOV (B, C)",
            0x42 => "MOV (B, D)",
            0x43 => "MOV (B, E)",
            0x44 => "MOV (B, H)",
            0x45 => "MOV (B, L)",
            0x46 => "MOV (B, M)",
            0x47 => "MOV (B, A)",

            0x48 => "MOV (C, B)",
            0x49 => "MOV (C, C)",
            0x4A => "MOV (C, D)",
            0x4B => "MOV (C, E)",
            0x4C => "MOV (C, H)",
            0x4D => "MOV (C, L)",
            0x4E => "MOV (C, M)",
            0x4F => "MOV (C, A)",

            0x50 => "MOV (D, B)",
            0x51 => "MOV (D, C)",
            0x52 => "MOV (D, D)",
            0x53 => "MOV (D, E)",
            0x54 => "MOV (D, H)",
            0x55 => "MOV (D, L)",
            0x56 => "MOV (D, M)",
            0x57 => "MOV (D, A)",

            0x58 => "MOV (E, B)",
            0x59 => "MOV (E, C)",
            0x5A => "MOV (E, D)",
            0x5B => "MOV (E, E)",
            0x5C => "MOV (E, H)",
            0x5D => "MOV (E, L)",
            0x5E => "MOV (E, M)",
            0x5F => "MOV (E, A)",

            0x60 => "MOV (H, B)",
            0x61 => "MOV (H, C)",
            0x62 => "MOV (H, D)",
            0x63 => "MOV (H, E)",
            0x64 => "MOV (H, H)",
            0x65 => "MOV (H, L)",
            0x66 => "MOV (H, M)",
            0x67 => "MOV (H, A)",

            0x68 => "MOV (L, B)",
            0x69 => "MOV (L, C)",
            0x6A => "MOV (L, D)",
            0x6B => "MOV (L, E)",
            0x6C => "MOV (L, H)",
            0x6D => "MOV (L, L)",
            0x6E => "MOV (L, M)",
            0x6F => "MOV (L, A)",

            0x70 => "MOV (M, B)",
            0x71 => "MOV (M, C)",
            0x72 => "MOV (M, D)",
            0x73 => "MOV (M, E)",
            0x74 => "MOV (M, H)",
            0x75 => "MOV (M, L)",
            0x76 => "HLT       ",
            0x77 => "MOV (M, A)",

            0x78 => "MOV (A, B)",
            0x79 => "MOV (A, C)",
            0x7A => "MOV (A, D)",
            0x7B => "MOV (A, E)",
            0x7C => "MOV (A, H)",
            0x7D => "MOV (A, L)",
            0x7E => "MOV (A, M)",
            0x7F => "MOV (A, A)",

            // ADD Instructions
            0x80 => "ADD B     ",
            0x81 => "ADD C     ",
            0x82 => "ADD D     ",
            0x83 => "ADD E     ",
            0x84 => "ADD H     ",
            0x85 => "ADD L     ",
            0x86 => "ADD M     ",
            0x87 => "ADD A     ",

            0x88 => "ADC B     ",
            0x89 => "ADC C     ",
            0x8A => "ADC D     ",
            0x8B => "ADC E     ",
            0x8C => "ADC H     ",
            0x8D => "ADC L     ",
            0x8E => "ADC M     ",
            0x8F => "ADC A     ",

            // SUB Instructions
            0x90 => "SUB B     ",
            0x91 => "SUB C     ",
            0x92 => "SUB D     ",
            0x93 => "SUB E     ",
            0x94 => "SUB H     ",
            0x95 => "SUB L     ",
            0x96 => "SUB M     ",
            0x97 => "SUB A     ",

            0x98 => "SBB B     ",
            0x99 => "SBB C     ",
            0x9A => "SBB D     ",
            0x9B => "SBB E     ",
            0x9C => "SBB H     ",
            0x9D => "SBB L     ",
            0x9E => "SBB M     ",
            0x9F => "SBB A     ",

            // ANA
            0xA0 => "ANA B     ",
            0xA1 => "ANA C     ",
            0xA2 => "ANA D     ",
            0xA3 => "ANA E     ",
            0xA4 => "ANA H     ",
            0xA5 => "ANA L     ",
            0xA6 => "ANA M     ",
            0xA7 => "ANA A     ",

            // XRA
            0xA8 => "XRA B     ",
            0xA9 => "XRA C     ",
            0xAA => "XRA D     ",
            0xAB => "XRA E     ",
            0xAC => "XRA H     ",
            0xAD => "XRA L     ",
            0xAE => "XRA M     ",
            0xAF => "XRA A     ",

            // ORA Instructions  0xB(reg)
            0xB0 => "ORA B     ",
            0xB1 => "ORA C     ",
            0xB2 => "ORA D     ",
            0xB3 => "ORA E     ",
            0xB4 => "ORA H     ",
            0xB5 => "ORA L     ",
            0xB6 => "ORA M     ",
            0xB7 => "ORA A     ",

            // CMP
            0xB8 => "CMP B     ",
            0xB9 => "CMP C     ",
            0xBA => "CMP D     ",
            0xBB => "CMP E     ",
            0xBC => "CMP H     ",
            0xBD => "CMP L     ",
            0xBE => "CMP M     ",
            0xBF => "CMP A     ",

            0xC0 => "RNZ       ",
            0xC1 => "POP BC    ",
            0xC2 => "JNZ       ",
            0xC3 => "JMP       ",
            0xC4 => "CNZ       ",
            0xC5 => "PUSH B    ",
            0xC6 => "ADI       ",
            0xC7 => "RST 0     ",
            0xC8 => "RZ        ",
            0xC9 => "RET       ",

            0xCA => "JZ        ",
            0xCB => "JMP       ",
            0xCC => "CZ        ",
            0xCD => "CALL      ",
            0xCE => "ACI       ",
            0xCF => "RST 1     ",

            0xD0 => "RNC       ",
            0xD1 => "POP DE    ",
            0xD2 => "JNC       ",
            0xD3 => "OUT       ",
            0xD4 => "CNC       ",
            0xD5 => "PUSH D    ",
            0xD6 => "SUI       ",
            0xD7 => "RST 2     ",
            0xD8 => "RC        ",
            0xD9 => "RET       ",

            0xDA => "JC        ",
            0xDB => "IN        ",
            0xDC => "CC        ",
            0xDD => "CALL      ",
            0xDE => "SBI       ",
            0xDF => "RST 3     ",

            0xE0 => "RPO       ",
            0xE1 => "POP HL    ",
            0xE2 => "JPO       ",
            0xE3 => "XTHL      ",
            0xE4 => "CPO       ",
            0xE5 => "PUSH H    ",
            0xE6 => "ANI       ",
            0xE7 => "RST 4     ",
            0xE8 => "RPE       ",
            0xE9 => "PCHL      ",

            0xEA => "JPE       ",
            0xEB => "XCHG      ",
            0xEC => "CPE       ",
            0xED => "CALL      ",
            0xEE => "XRI       ",
            0xEF => "RST 5     ",

            0xF0 => "RP        ",
            0xF1 => "POP PSW   ",
            0xF2 => "JP        ",
            0xF3 => "DI        ",
            0xF4 => "CP        ",
            0xF5 => "PUSH PSW  ",
            0xF6 => "ORI       ",
            0xF7 => "RST 6     ",
            0xF8 => "RM        ",
            0xF9 => "SPHL      ",

            0xFA => "JM        ",
            0xFB => "EI        ",
            0xFC => "CM        ",
            0xFD => "CALL      ",
            0xFE => "CPI       ",
            0xFF => "RST 7     ",
            _ => ("Unknown opcode"),
        }
    }
}


