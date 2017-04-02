 //
use super::display::Display;
use super::memory::Memory;
use super::sdl2;

pub struct Interconnect {
    pub display: Display,
    pub memory: Memory,
}

impl Interconnect {
    pub fn new() -> Interconnect {
        let sdl_ctx = sdl2::init().unwrap();

        Interconnect {
            display: Display::new(&sdl_ctx),
            memory: Memory::new(),
            // TODO input
            // TODO audio

        }
    }
}
