use super::sdl2;

use super::cpu::Cpu;
use super::display::Display;
use super::memory::Memory;
use super::keypad::Keypad;

pub struct Interconnect {
    // pub cpu: Cpu,
    pub display: Display,
    pub memory: Box<Memory>,
    pub keypad: Keypad,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let ctx = sdl2::init().unwrap();

        Interconnect {
            // cpu: Cpu::new(),
            display: Display::new(&ctx),
            memory: Box::new(Memory::new()),
            keypad: Keypad::new(&ctx),
            // TODO audio

        }
    }
}
