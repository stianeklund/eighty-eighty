use super::minifb::{Key, Scale, WindowOptions, Window};

use super::cpu::ExecutionContext;
use super::display::Display;
// use super::keypad::Keypad;
use super::memory::Memory;

pub struct Interconnect<'a> {
    pub cpu: ExecutionContext<'a>,
    pub display: Display,
    // pub memory: Memory,
    // pub keypad: Keypad,
}

impl <'a>Interconnect<'a> {
    pub fn new() -> Interconnect<'a>{
        // let ctx = sdl2::init().unwrap();

        Interconnect {
            cpu: ExecutionContext::new(),
            display: Display::new(),
            // memory: Memory::new(),
            // keypad: Keypad::new(),
            // TODO audio

        }
    }
}
