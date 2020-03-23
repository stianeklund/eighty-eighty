use minifb::{Key, Window};
use crate::cpu::Registers;
use std::borrow::Borrow;

pub struct Keypad {
    // DIP settings
    coin_info: u8, //  Port 2 Bit 7 Coin info displayed in demo screen if 0x00
    credit: u8,    // Bit 0 1 if coin deposited
    tilt: u8,      // Tilt check
    dip3: u8,      // Bit 0 (0x00 = 3 ships, 0x10 = 5 ships)
    dip5: u8,      // Bit 1 (0x01 = 4 ships, 0x11 = 6 ships)
    dip6: u8,      // Bit 3 (0x00 = extra ship at 1500, 0x01 = extra ship at 1000)

    // Player 1
    p1_start: u8,  // Port 1 Bit 1 (0x01 if pressed)

    // Port 0  Bits
    // Port 0 Bit 3 (always 0x01)
    p1_fire: u8,   // Port 0 Bit 4 (0x01 if pressed)
    p1_left: u8,   // Port 0 Bit 5 (0x01 if pressed)
    p1_right: u8,  // Port 0 Bit 6 (0x01 if pressed)

    // Player 2
    p2_shot: u8,   // Port 2 Bit 4 (0x01 if pressed)
    p2_start: u8,  // Port 1 Bit 1 (0x01 if pressed)
    p2_right: u8,  // Port 2 Bit 6 (0x01 if pressed)
    p2_left: u8,   // Port 2 Bit 5 (0x01 if pressed)

}

pub struct State {
    up: bool,
    down: bool,
    changed: bool
}
pub trait Input {
    fn key_value(&self) -> &Keypad;
    fn key_down(&mut self, key: Key);
    fn key_up(&mut self, key: Key);
    fn handle_input(&mut self, key: Key);
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            coin_info: 0x80,
            p1_start: 0x04,
            p1_fire: 0x10,
            p1_left: 0x20,
            p1_right: 0x40,
            p2_shot: 0,
            p2_start: 0x2,
            p2_left: 0,
            p2_right: 0,
            credit: 0x1,
            tilt: 0,
            dip3: 0,
            dip5: 0,
            dip6: 0,
        }
    }
    pub fn poll_input(registers: &mut Registers, window: &Window) {
        window.get_keys().map(|keys| {
            for t in keys {
                match t {
                    Key::D => registers.debug = true,
                    Key::E => registers.debug = false,
                    Key::C | Key::Enter | Key::Space | Key::Key2 | Key::Key3 |
                    Key::Left | Key::Right => Input::handle_input(registers, t),
                    Key::Escape => ::std::process::exit(0),
                    _ => eprintln!("Input key not handled"),
                }
            }
        });
    }
}
impl State {
    fn new() -> State {
        State {
            up: true,
            down: false,
            changed: false,
        }
    }
}
impl State {
    fn status(&mut self) -> bool {
        self.changed
    }
}

impl Input for Registers {
    fn key_value(&self) -> &Keypad { self.key_value().borrow() }
    fn key_down(&mut self, key: Key) {
        let keypad = self.key_value();
        let mut state = State::new();

        match key {
            Key::Enter => self.port_1_in |= keypad.p1_start,
            Key::C     => self.port_1_in |= keypad.credit,
            Key::Space => self.port_1_in |= keypad.p1_fire,
            Key::Key2  => self.port_2_in |= keypad.p2_start,
            Key::Key3  => self.port_2_in |= keypad.coin_info,
            Key::Left  => self.port_1_in |= keypad.p1_left,
            Key::Right => self.port_1_in |= keypad.p1_right,
            _ => eprintln!("Key not implemented"),
        }
        println!("Key down");

        state.up = false;
        state.down = true;
        state.changed = true;
    }
    fn key_up(&mut self, key: Key) {
        let mut state = State::new();
        let keypad = self.key_value();

        match key {
            Key::Key3  => self.port_2_in &= !keypad.coin_info,
            Key::Enter => self.port_1_in &= keypad.p1_start,
            Key::Key2  => self.port_2_in &= keypad.p2_start,
            Key::C     => self.port_1_in &= keypad.credit,
            Key::Space => self.port_1_in &= keypad.p1_fire,
            Key::Left  => self.port_1_in &= keypad.p1_left,
            Key::Right => self.port_1_in &= keypad.p1_right,
            _ => eprintln!("Key not implemented"),
        }

        println!("Key up");

        state.up = true;
        state.down = false;
        state.changed = true;
    }
    fn handle_input(&mut self, key: Key) {
        let mut state = State::new();

        if !state.changed && !state.down {
            self.key_down(key);
            state.changed = false;
            state.down = false;
            state.up = true;

            println!("Key press:{:?}", key);
        }
        if !state.changed && state.up {
            self.key_up(key);
            state.changed = false;
            state.up = false;
            state.down = true;
        }
    }
}


