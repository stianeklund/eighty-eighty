use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

pub struct Memory {
    pub memory: Box<[u8; 65536]>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: Box::new([0; 65536]),
        }
    }

    pub fn read_byte(&mut self, byte: u8) -> u8 {
        self.memory[byte as usize]
    }

    pub fn write_byte(&mut self, addr: u8, mut byte: u16) {
        byte = self.memory[addr as usize & 0xFFFF] as u16;
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        // (self.read_byte(addr + 2) as u16) << 8 | self.read_byte(addr + 1) as u16
        (self.memory[addr as usize + 2] as u16) << 8 | (self.memory[addr as usize + 1] as u16)
    }

    pub fn write_word(&mut self, addr: u8, word: u16) {
        self.write_byte(addr, word & 0xFF);
        self.write_byte(addr + 1, (word >> 8) & 0xFF);
    }

    #[allow(exceeding_bitshifts)]
    pub fn read_short(&mut self, addr: usize) -> u16 {
        // TODO Investigate whether this is correct..
        (self.memory[addr + 1] << 8 | self.memory[addr]) as u16
    }

    // Reads the memory address and returns a 16 bit integer, for self.pc / sp instructions
    pub fn read_rp(&mut self, addr: usize) -> u16 {
        self.memory[addr] as u16
    }

    // Useful to read values out of memory to assign to 8 bit registers
    pub fn read(&mut self, addr: usize) -> u8 {
        self.memory[addr] as u8
}
    pub fn write(&mut self, addr: usize, val: u8) {
        self.memory[addr] = val;
    }

    pub fn read_or(&mut self, reg: usize) -> u8 {
        self.memory[reg | reg]
    }

    pub fn load_bin(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("File open failed");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read file");
        let buf_len = buf.len();
        for i in 0..buf_len {
            self.memory[i] = buf[i];
        }
        println!("Loaded binary");
    }

    pub fn render_vram(&mut self) {
        // 0x2400 is the beginning of VRAM
        let mut base: u8 = 0x2400;

        for j in 0..224 {
            let src = 0x2400 + (j << 5);
            for i in 0..32 {
                if self.memory[i] & 1u8.wrapping_shl(j) != 0 {
                    self.memory[src as usize] ^= 0xFFFF;

                } else {
                    self.memory[src as usize] ^= 0x0000;
                }
            }
        }
    }
}
