use mixer::command::Command;
use std::collections::VecDeque;

pub struct Field {
    pub cmd: Option<Command>,
    pub note: Option<u8>,
}

pub struct Controller {
    pub sequence: VecDeque<Field>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller { sequence: VecDeque::new() }
    }
}
