use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fmt;
use std::ops::{Index, IndexMut};

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

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        &mut self.memory[index as usize]
    }
}
impl Index<u16> for Memory {
    type Output = u8;
    fn index(&self, index: u16) ->  &u8 {
        &self.memory[index as usize]
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { memory: vec![0; 0x1_0000] }
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize & 0xFFFF]
    }

    pub fn read_word(&mut self, addr: u16) -> u16 {
        (self.read_byte(addr.wrapping_add(1)) as u16) << 8 | self.read_byte(addr) as u16
    }

    pub fn read_hb(&mut self, addr: u16) -> u8 {
        self.read_byte(addr.wrapping_add(2))
    }

    pub fn read_lb(&mut self, addr: u16) -> u8 {
        self.read_byte(addr.wrapping_add(1))
    }

    pub fn read(&self, addr: u16) -> u16 { u16::from(self.memory[addr as usize])  }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self.write_byte(addr, word as u8);
        self.write_byte((addr.wrapping_add(1)), (word >> 8) as u8);
    }
    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize & 0xFFFF] = byte
    }
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
