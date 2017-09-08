use super::cpu::{ExecutionContext, Registers};
use super::memory::Memory;

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
}
