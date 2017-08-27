use super::minifb::{Key, Scale, WindowOptions, Window};
use std::env;

use super::cpu::{ExecutionContext, Registers};
use super::display::Display;
// use super::keypad::Keypad;
use super::memory::Memory;

pub struct Interconnect {
    //pub cpu: ExecutionContext<'a>,
    pub registers: Registers,
    pub display: Display,
    pub memory: Memory,
    // pub keypad: Keypad,
}

impl Interconnect {
    //pub fn new(memory: &'a mut Memory, registers: &'a mut Registers) -> Self {
    pub fn new() -> Self {
        // let ctx = sdl2::init().unwrap();
        // let mut memory = Memory::new();
        // let mut registers = Registers::new();

        Interconnect {
            // cpu: ExecutionContext::new(memory, registers),
            display: Display::new(),
            memory: Memory::new(),
            registers: Registers::new(),
            // keypad: Keypad::new(),
            // TODO audio
        }
    }
}

