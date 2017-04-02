pub struct Memory {
    pub memory: Box<[u8; 65536]>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: Box::new([0; 65536]),
        }
    }

    pub fn read_byte(&mut self, addr: u8) -> u8 {
        self.memory[addr as usize]
    }

    pub fn read_word(&mut self, addr: u8) -> u16 {
    (self.read_byte(addr + 1) as u16) << 8 | self.read_byte(addr) as u16
    }

    pub fn read_short(&mut self, addr: usize) -> u16 {
        (self.memory[addr] | self.memory[addr]) as u16
    }


    pub fn write_byte(&mut self, addr: u8, mut byte: u16) {
        byte = self.memory[addr as usize & 0xFFFF] as u16;
    }

    pub fn write_word(&mut self, addr: u8, word: u16) {
        self.write_byte(addr, word & 0xFF);
        self.write_byte(addr + 1, (word >> 8) & 0xFF);
    }
}
