pub fn from_char(c: char) -> u8 {
    let wide: char = match c {
        '0' | 'O' | 'o' => '0',
        '1' | 'I' | 'l' | 'i' => '1',
        '2' | 'Z' | 'z' => '2',
        '5' | 'S' | 's' => '5',
        '6' | 'b' => '6',
        '8' | 'B' => '8',
        '9' | 'g' => '9',
        _ => c.to_uppercase().nth(0).unwrap(),
    };
    wide as u8
}
