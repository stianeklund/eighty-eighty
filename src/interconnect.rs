use super::cpu::{ExecutionContext, Registers};
use super::memory::Memory;
use super::std::io;

pub struct Interconnect {
    pub registers: Registers,
    pub memory: Memory,
}

impl Interconnect {
    pub fn new() -> Self {
        let registers = Registers::new();
        let memory = Memory::new();

        Interconnect {
            registers,
            memory,
        }
    }
    pub fn execute_cpu(&mut self) {
        ExecutionContext::new(self).step(1);

    }

    // Step once when pressing a key
    pub fn step_cpu(&mut self) {
        io::stdin().read_line(&mut String::new()).unwrap();
        ExecutionContext::new(self).step(1);

    }
}
