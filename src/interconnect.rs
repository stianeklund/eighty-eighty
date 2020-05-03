use super::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;

pub struct Interconnect {
    pub cpu: Cpu,
    pub keypad: Keypad,
    pub display: Display,
    pub frame_count: u32,
}

impl Interconnect {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            keypad: Keypad::new(),
            display: Display::new(),
            frame_count: 0,
        }
    }

    pub fn execute_cpu(&mut self) -> u32 {
        let mut cycles_executed: usize = 0;

        while cycles_executed <= 16666 {
            let start_cycles = self.cpu.registers.cycles;
            self.cpu.execute_instruction();
            if self.cpu.registers.debug {
                println!("{:?}", self.cpu.registers);
            }
            cycles_executed += self.cpu.registers.cycles - start_cycles;
            self.cpu.try_interrupt();
        }

        self.frame_count += 1;
        return self.frame_count;
    }

    // Step once when pressing a key
    pub fn run_tests(&mut self) {
        self.cpu.execute_tests();
    }
}
