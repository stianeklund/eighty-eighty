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
        ExecutionContext::new(self).step();

    }

    // Step once when pressing a key
    pub fn run_tests(&mut self) {
        ExecutionContext::new(self).execute_tests();

    }
}
