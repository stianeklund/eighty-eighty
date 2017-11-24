use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;

pub struct Memory {
    pub memory: Vec<u8>,
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
        Memory { memory: vec![0; 0x1_0000] }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize & 0xFFFF]
    }

    // Read immediate value
    pub fn read_imm(&mut self, addr: u16) -> u16 {
        (self.memory[addr as usize + 2] as u16) << 8 | u16::from(self.memory[addr as usize + 1])
    }
    pub fn read_word(&mut self, addr: u16) -> u16 {
        (self.memory[addr as usize + 1] as u16) << 8 | u16::from(self.memory[addr as usize])
    }

    pub fn read_high(&mut self, addr: u16) -> u8 {
        (self.memory[addr as usize + 2])
    }

    pub fn read_low(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize + 1]
    }

    pub fn read(&mut self, addr: u16) -> u16 { u16::from(self.memory[addr as usize])  }

    pub fn load_bin(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("Couldn't load binary");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read binary");
        self.memory[..buf.len()].clone_from_slice(&buf[..]);
        println!("Loaded: {:?} Bytes: {:?}", path, buf.len());
    }
    pub fn load_tests(&mut self, file: &str) {
        let path = Path::new(file);
        let mut file = File::open(&path).expect("Couldn't load binary");
        let mut buf = Vec::new();

        file.read_to_end(&mut buf).expect("Failed to read binary");
        // Tests are loaded at 0x0100
        self.memory[0x0100..(buf.len() + 0x0100)].clone_from_slice(&buf[..]);
        println!("Test loaded: {:?} Bytes: {:?}", path, buf.len());
    }
}
