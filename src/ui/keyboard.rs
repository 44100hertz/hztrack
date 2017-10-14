use std::sync::{MutexGuard};
use sdl2::keyboard::Scancode;

use mixer::control::*;

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
    pub fn handle_key(&mut self, sc:Scancode,
                      mut ctrl: MutexGuard<Controller>) {
        let note_offset = self.octave * 12;
        let note = |n: u8, mut ctrl: MutexGuard<Controller>| {
            let total = n + note_offset;
            if total < 12*10 {
                ctrl.set_note(Note::On(total));
            }
        };
        match sc {
            // octave 0
            Scancode::Z => note(0, ctrl),
            Scancode::S    => note(1, ctrl),
            Scancode::X => note(2, ctrl),
            Scancode::D    => note(3, ctrl),
            Scancode::C => note(4, ctrl),
            Scancode::V => note(5, ctrl),
            Scancode::G    => note(6, ctrl),
            Scancode::B => note(7, ctrl),
            Scancode::H    => note(8, ctrl),
            Scancode::N => note(9, ctrl),
            Scancode::J    => note(10, ctrl),
            Scancode::M => note(11, ctrl),
            // octave 1
            Scancode::Q => note(12, ctrl),
            Scancode::Num2 => note(13, ctrl),
            Scancode::W => note(14, ctrl),
            Scancode::Num3 => note(15, ctrl),
            Scancode::E => note(16, ctrl),
            Scancode::R => note(17, ctrl),
            Scancode::Num5 => note(18, ctrl),
            Scancode::T => note(19, ctrl),
            Scancode::Num6 => note(20, ctrl),
            Scancode::Y => note(21, ctrl),
            Scancode::Num7 => note(22, ctrl),
            Scancode::U => note(23, ctrl),
            // octave 2
            Scancode::I => note(24, ctrl),
            Scancode::Num9 => note(25, ctrl),
            Scancode::O => note(26, ctrl),
            Scancode::Num0 => note(27, ctrl),
            Scancode::P => note(28, ctrl),

            Scancode::Num1 => ctrl.set_note(Note::Off),

            Scancode::PageUp => self.octave_up(),
            Scancode::PageDown => self.octave_down(),

            Scancode::Up    => ctrl.scroll(-1),
            Scancode::Down  => ctrl.scroll(1),

            Scancode::Space => ctrl.play = !ctrl.play,

            _ => {}
        };
    }
}


