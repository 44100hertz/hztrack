use std::sync::{Mutex, Arc};
use sdl2::keyboard::Scancode;
use mixer::control::Controller;

pub struct Keyboard {
    octave: u8,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { octave: 4, }
    }
    pub fn octave_up(&mut self) {
        if self.octave < 8 { self.octave += 1; }
    }
    pub fn octave_down(&mut self) {
        if self.octave > 0 { self.octave -= 1; }
    }
    pub fn handle_key(&mut self, sc: Scancode,
                      ctrl: Arc<Mutex<Controller>>) {
        let note_offset = self.octave * 12;
        let note = |n: u8| {
            let total = n + note_offset;
            if total >= 0 && total < 12*10 {
                ctrl.lock().unwrap().set_note(Some(n + note_offset));
            }
        };
        match sc {
            Scancode::Z => note(0),
            Scancode::S    => note(1),
            Scancode::X => note(2),
            Scancode::D    => note(3),
            Scancode::C => note(4),
            Scancode::V => note(5),
            Scancode::G    => note(6),
            Scancode::B => note(7),
            Scancode::H    => note(8),
            Scancode::N => note(9),
            Scancode::J    => note(10),
            Scancode::M => note(11),

            Scancode::Q => note(12),
            Scancode::Num2 => note(13),
            Scancode::W => note(14),
            Scancode::Num3 => note(15),
            Scancode::E => note(16),
            Scancode::R => note(17),
            Scancode::Num5 => note(18),
            Scancode::T => note(19),
            Scancode::Num6 => note(20),
            Scancode::Y => note(21),
            Scancode::Num7 => note(22),
            Scancode::U => note(23),
            Scancode::I => note(24),
            Scancode::Num9 => note(25),
            Scancode::O => note(26),
            Scancode::Num0 => note(27),
            Scancode::P => note(28),

            Scancode::PageUp => self.octave_up(),
            Scancode::PageDown => self.octave_down(),

            _ => {}
        };
    }
}


