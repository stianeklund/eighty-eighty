use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

pub struct Memory {
    pub memory: [u8; 65536],
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:?}", val)
    }
}

impl fmt::UpperHex for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = self;
        write!(f, "{:04X}", val)
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 65536],
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
    pub fn write_memory(&mut self, addr: u16) {
        // (self.read_byte(addr + 2) as u16) << 8 | self.read_byte(addr + 1) as u16
      (self.memory[addr as usize + 2] as u16) << 8 | (self.memory[addr as usize + 1] as u16);
    }


    pub fn read_high(&mut self, addr: u16) -> u8 {
        (self.memory[addr as usize + 2])
    }

    pub fn read_low(&mut self, addr: u16) -> u8 {
        // (self.memory[addr as usize + 2] as u16) << 8
        self.memory[addr as usize + 1]
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
}

