
/* pub struct Keypad {
    pump: EventPump
}*/

// TODO Use a generic Emu state instead
pub enum State {
    Exit,
    Continue,
    Break,
    Step,
}

/* impl Keypad {
    pub fn new(ctx: &Sdl) -> Self {
        Keypad {
            pump: ctx.event_pump().unwrap(),
        }
    }

    // Poll for scancodes
    pub fn key_press(&mut self) -> State {


        for event in self.pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    return State::Exit;
                },

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return State::Exit;
                },

                // Lets break to do some debugging
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    return State::Break;
                },

                // For stepping CPU instructions
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                return State::Step;
                },
                _ => {}
            }
        }
        State::Continue
    }
}*/
