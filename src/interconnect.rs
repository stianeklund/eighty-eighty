use super::cpu::Cpu;
use super::display::Display;
use super::memory::Memory;
use super::sdl2;

pub struct Interconnect {
    // pub cpu: Cpu,
    // pub display: Display,
    pub mem: Memory,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        // let sdl_ctx = sdl2::init().unwrap();

        Interconnect {
            // cpu: Cpu::new(),
            // display: Display::new(&sdl_ctx),
            mem: Memory::new(),
            // TODO input
            // TODO audio

        }
    }
}
