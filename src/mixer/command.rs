use base32;
use mixer::Mixer;

pub struct Command {
    id: u8,
    data: u8,
}
pub fn from_str(raw: &str) -> Command {
    let mut iter = raw.chars();
    Command {
        id: base32::from_char(iter.next().unwrap()),
        data: u8::from_str_radix(iter.as_str(), 16).unwrap(),
    }
}
impl Command {
    pub fn execute(&self, m: &mut Mixer) {
        match self.id as char {
            '2' => {
                if self.data < 32 {
                    m.tick_rate = self.data
                } else {
                    m.bpm = self.data }},
            _ => eprintln!("invalid command!"),
        }
    }
    pub fn string(&self) -> String {
        format!("{}{:X}", self.id as char, self.data)
    }
    pub fn print(&self) {
        println!("{}", self.string());
    }
}
