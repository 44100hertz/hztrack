// 0123456789ABCDEFGHJKLMNPQRTUVWXY

pub fn from_char(c: char) -> u8 {
    let wide = match c {
        'O' | 'o' => '0',
        'I' | 'l' | 'i' => '1',
        'Z' | 'z' => '2',
        'S' | 's' => '5',
        _ => c.to_uppercase().nth(0).unwrap(),
    };
    wide as u8
}
