use std::sync::{MutexGuard};
use sdl2::keyboard::Scancode;

use mixer::control::*;
use super::sequence::*;

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
                      mut seq: MutexGuard<Sequence>) {
        let note_offset = self.octave * 12;
        let note = |n: u8, mut seq: MutexGuard<Sequence>| {
            let total = n + note_offset;
            if total < 12*10 {
                seq.set_note(Note::On(total));
            }
        };
        match sc {
            // octave 0
            Scancode::Z => note(0, seq),
            Scancode::S    => note(1, seq),
            Scancode::X => note(2, seq),
            Scancode::D    => note(3, seq),
            Scancode::C => note(4, seq),
            Scancode::V => note(5, seq),
            Scancode::G    => note(6, seq),
            Scancode::B => note(7, seq),
            Scancode::H    => note(8, seq),
            Scancode::N => note(9, seq),
            Scancode::J    => note(10, seq),
            Scancode::M => note(11, seq),
            // octave 1
            Scancode::Q => note(12, seq),
            Scancode::Num2 => note(13, seq),
            Scancode::W => note(14, seq),
            Scancode::Num3 => note(15, seq),
            Scancode::E => note(16, seq),
            Scancode::R => note(17, seq),
            Scancode::Num5 => note(18, seq),
            Scancode::T => note(19, seq),
            Scancode::Num6 => note(20, seq),
            Scancode::Y => note(21, seq),
            Scancode::Num7 => note(22, seq),
            Scancode::U => note(23, seq),
            // octave 2
            Scancode::I => note(24, seq),
            Scancode::Num9 => note(25, seq),
            Scancode::O => note(26, seq),
            Scancode::Num0 => note(27, seq),
            Scancode::P => note(28, seq),

            Scancode::Num1  => seq.set_note(Note::Off),
            Scancode::Grave => seq.set_note(Note::Hold),

            Scancode::PageUp    => self.octave_up(),
            Scancode::PageDown  => self.octave_down(),

            Scancode::Up    => seq.move_cursor(0, -1),
            Scancode::Down  => seq.move_cursor(0, 1),
            Scancode::Left  => seq.move_cursor(-1, 0),
            Scancode::Right => seq.move_cursor(1, 0),

            Scancode::Insert => seq.insert(),
            Scancode::Delete => seq.remove(),

            Scancode::Space => seq.play = !seq.play,

            _ => {}
        };
    }
}


