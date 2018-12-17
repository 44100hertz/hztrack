use sdl2::keyboard::Scancode;

use track::Note;

pub fn to_hex(sc: Scancode) -> Option<u8> {
    match sc {
        Scancode::Num0 => Some(0),
        Scancode::Num1 => Some(1),
        Scancode::Num2 => Some(2),
        Scancode::Num3 => Some(3),
        Scancode::Num4 => Some(4),
        Scancode::Num5 => Some(5),
        Scancode::Num6 => Some(6),
        Scancode::Num7 => Some(7),
        Scancode::Num8 => Some(8),
        Scancode::Num9 => Some(9),
        Scancode::A    => Some(0xA),
        Scancode::B    => Some(0xB),
        Scancode::C    => Some(0xC),
        Scancode::D    => Some(0xD),
        Scancode::E    => Some(0xE),
        Scancode::F    => Some(0xF),
        _ => None,
    }
}

pub fn to_note(sc: Scancode) -> Note {
    match sc {
        // note: the indentation is for white and black keys!
        Scancode::Z => Note::On(0),
        Scancode::S    => Note::On(1),
        Scancode::X => Note::On(2),
        Scancode::D    => Note::On(3),
        Scancode::C => Note::On(4),
        Scancode::V => Note::On(5),
        Scancode::G    => Note::On(6),
        Scancode::B => Note::On(7),
        Scancode::H    => Note::On(8),
        Scancode::N => Note::On(9),
        Scancode::J    => Note::On(10),
        Scancode::M => Note::On(11),
        Scancode::Q => Note::On(12),
        Scancode::Num2 => Note::On(13),
        Scancode::W => Note::On(14),
        Scancode::Num3 => Note::On(15),
        Scancode::E => Note::On(16),
        Scancode::R => Note::On(17),
        Scancode::Num5 => Note::On(18),
        Scancode::T => Note::On(19),
        Scancode::Num6 => Note::On(20),
        Scancode::Y => Note::On(21),
        Scancode::Num7 => Note::On(22),
        Scancode::U => Note::On(23),
        Scancode::I => Note::On(24),
        Scancode::Num9 => Note::On(25),
        Scancode::O => Note::On(26),
        Scancode::Num0 => Note::On(27),
        Scancode::P => Note::On(28),

        Scancode::Num1 =>  Note::Off,

        _ => Note::Hold,
    }
}
