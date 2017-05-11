use super::minifb::{Key, Scale, WindowOptions, Window};

use super::cpu::{ExecutionContext, Registers};
use super::display::Display;
// use super::keypad::Keypad;
use super::memory::Memory;

pub struct Interconnect<'a> {
    pub cpu: ExecutionContext<'a>,
    pub display: Display,
    pub memory: Memory,
    // pub keypad: Keypad,
}

impl <'a>Interconnect<'a> {
    pub fn new() -> Interconnect<'a>{
        // let ctx = sdl2::init().unwrap();
        let mut memory = Memory::new();
        let mut registers = Registers::new();

        Interconnect {
            cpu: ExecutionContext::new(&mut memory, &mut registers),
            display: Display::new(),
            memory: Memory::new(),
            // keypad: Keypad::new(),
            // TODO audio

        }
    }
}
