pub fn from_char(w: char) -> Result<u8, &'static str> {
    if w as u32 > 127 {
        return Err("character out of base32 range.");
    }
    match BASE32[w as usize] {
        0 => Err("character not found in conversion table."),
        c => Ok(c),
    }
}

const BASE32: [u8; 128] = [
    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,    0,    0,    0,    0,
    0,    0,    0,    0,    0,    0,    0,    0,
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
    b'8', b'9', 0,    0,    0,    0,    0,    0,
    0,    b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'1', b'J', b'K', b'L', b'M', b'N', b'0',
    b'P', b'Q', b'R', b'5', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'2', 0,    0,    0,    0,    0,
    0,    b'A', b'B', b'C', b'D', b'E', b'F', b'G',
    b'H', b'1', b'J', b'K', b'1', b'M', b'N', b'0',
    b'P', b'Q', b'R', b'5', b'T', b'U', b'V', b'W',
    b'X', b'Y', b'2', 0,    0,    0,    0,    0,
];
