use crate::cpu::Registers;
use minifb::{Key, KeyRepeat, Window};
use std::borrow::Borrow;

pub struct Keypad {

    // Port 2 Bit 7 Coin info displayed in demo screen if 0x00
    coin_info: u8,

    credit: u8,   // Bit 0 1 if coin deposited
    tilt: u8,     // Tilt check (not used)

    // DIP switch:
    dip3: u8,     // Bit 0 (0x00 = 3 ships, 0x10 = 5 ships)
    dip5: u8,     // Bit 1 (0x01 = 4 ships, 0x11 = 6 ships)
    dip6: u8,     // Bit 3 (0x00 = extra ship at 1500, 0x01 = extra ship at 1000)

    // Player 1:
    p1_start: u8, // Port 1 Bit 1 (0x01 if pressed)
    p1_fire: u8,  // Port 0 Bit 4 (0x01 if pressed)
    p1_left: u8,  // Port 0 Bit 5 (0x01 if pressed)
    p1_right: u8, // Port 0 Bit 6 (0x01 if pressed)

    // Player 2:
    p2_start: u8, // Port 1 Bit 1 (0x01 if pressed)
    p2_fire: u8,  // Port 2 Bit 4 (0x01 if pressed)
    p2_left: u8,  // Port 2 Bit 5 (0x01 if pressed)
    p2_right: u8, // Port 2 Bit 6 (0x01 if pressed)
}

pub trait Input {
    fn key_value(&self) -> &Keypad;
    fn key_down(&mut self, reg: &mut Registers, window: &Window);
    fn key_up(&mut self, reg: &mut Registers, window: &Window);
    fn reset_ports(&self, reg: &mut Registers);
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            coin_info: 0x80,
            p1_start: 0x04,
            p1_fire: 0x10,
            p1_left: 0x20,
            p1_right: 0x40,
            p2_fire: 0,
            p2_start: 0x02,
            p2_left: 0,
            p2_right: 0,
            credit: 0x01,
            tilt: 0,
            dip3: 0,
            dip5: 0,
            dip6: 0,
        }
    }
}

impl Input for Keypad {
    fn key_value(&self) -> &Keypad {
        self.borrow()
    }
    fn key_down(&mut self, reg: &mut Registers, window: &Window) {
        if window.is_open() {
            window
                .get_keys_pressed(KeyRepeat::Yes)
                .unwrap()
                .iter()
                .for_each(|keys| match keys {
                    Key::Enter => reg.port_1_in |= self.p1_start,
                    Key::C => reg.port_1_in |= self.credit,
                    Key::Space => reg.port_1_in |= self.p1_fire,
                    Key::Key2 => reg.port_2_in |= self.p2_start,
                    Key::Key3 => reg.port_2_in |= self.coin_info,
                    Key::Left => reg.port_1_in |= self.p1_left,
                    Key::Right => reg.port_1_in |= self.p1_right,
                    Key::Escape => std::process::exit(0),
                    _ => eprintln!("Key: {:?} not implemented", *keys),
                });
        }
    }

    fn key_up(&mut self, reg: &mut Registers, window: &Window) {
        // TODO Improve handling
        // Problem here is likely that the keys pressed are not the same
        // as in `key_down()`

        if window.is_open() {
            window
                .get_keys()
                .unwrap()
                .iter()
                .for_each(|keys| match keys {
                    Key::Enter => reg.port_1_in &= !self.p1_start,
                    Key::C => reg.port_1_in &= !self.credit,
                    Key::Space => reg.port_1_in &= !self.p1_fire,
                    Key::Key2 => reg.port_2_in &= !self.p2_start,
                    Key::Key3 => reg.port_2_in &= !self.coin_info,
                    Key::Left => reg.port_1_in &= !self.p1_left,
                    Key::Right => reg.port_1_in &= !self.p1_right,
                    _ => eprintln!("Key: {:?} not implemented", *keys),
                });
        }
    }

    fn reset_ports(&self, reg: &mut Registers) {
        reg.port_1_in &= !self.credit;
        reg.port_1_in &= !self.p1_left;
        reg.port_1_in &= !self.p1_right;
        reg.port_1_in &= !self.p1_fire;
        reg.port_1_in &= !self.p1_start;
        reg.port_2_in &= !self.p2_start;
        reg.port_2_in &= !self.coin_info;
    }
}
