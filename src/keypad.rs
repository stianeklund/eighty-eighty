use minifb::Key;
use cpu::Registers;

pub struct Keypad {
    p1_start: u8,
    p2_start: u8,
    credit: u8,
    fire: u8,
    left: u8,
    right: u8,
}

pub struct State {
    up: bool,
    down: bool,
    changed: bool
}
pub trait Input {
    fn key_value(&self) -> Keypad;
    fn key_down(&mut self, key: Key);
    fn key_up(&mut self, key: Key);
    fn handle_input(&mut self, key: Key);
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            p1_start: 0x04,
            p2_start: 0x2,
            credit: 0x1,
            fire: 0x10,
            left: 0x20,
            right: 0x40,
        }
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
    fn key_value(&self) -> Keypad {
        Keypad {
            p1_start: 0x04,
            p2_start: 0x2,
            credit: 0x1,
            fire: 0x10,
            left: 0x20,
            right: 0x40,
        }
    }
    fn key_down(&mut self, key: Key) {
        // let keypad = Keypad::new();
        let keypad = self.key_value();
        let mut state = State::new();

        match key {
            Key::Enter => self.port_1_in |= keypad.p1_start,
            Key::C => self.port_1_in     |= keypad.credit,
            Key::Space => self.port_1_in |= keypad.fire,
            Key::Key2 => self.port_2_in  |= keypad.p2_start,
            Key::Left => self.port_1_in  |= keypad.left,
            Key::Right => self.port_1_in |= keypad.right,
            _ => eprintln!("Key not implemented"),
        }

        state.up = false;
        state.down = true;
        state.changed = true;
    }
    fn key_up(&mut self, key: Key) {
        let mut state = State::new();
        let keypad = self.key_value();

        match key {
            Key::Enter => self.port_1_in &= keypad.p1_start,
            Key::C => self.port_1_in     &= keypad.credit,
            Key::Space => self.port_1_in &= keypad.fire,
            Key::Key2 => self.port_2_in  &= keypad.p2_start,
            Key::Left => self.port_1_in  &= keypad.left,
            Key::Right => self.port_1_in &= keypad.right,
            _ => eprintln!("Key not implemented"),
        }

        state.up = true;
        state.down = false;
        state.changed = true;
    }
    fn handle_input(&mut self, key: Key) {
        let mut state = State::new();

        if !state.changed && state.up {
            self.key_up(key);
            state.changed = true;
            state.up = false;
        }
        if state.changed && !state.up {
            self.key_down(key);
            state.changed = true;
            state.down = false;
        }
        println!("Key press:{:?}", key);
    }
}

