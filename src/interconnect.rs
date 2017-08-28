use super::minifb::{Key, Scale, WindowOptions, Window};
use std::env;

use super::cpu::{ExecutionContext, Registers};
use super::display::Display;
use super::memory::Memory;

pub struct Interconnect {
    pub registers: Registers,
    // pub display: Display,
    pub memory: Memory,
    // pub keypad: Keypad,
}

impl Interconnect {
    //pub fn new(memory: &'a mut Memory, registers: &'a mut Registers) -> Self {
    pub fn new() -> Self {

        let registers = Registers::new();
        //let display = Display::new();
        let memory = Memory::new();

        Interconnect {
            registers: registers,
         //    display: display,
            memory: memory,
        }
    }
    pub fn execute_cpu(&mut self) {
        ExecutionContext::new(self).step(1);
    }
}
