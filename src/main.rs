extern crate sdl2;

use std::sync::{Mutex, Arc};

mod base32;
mod mixer;
mod display;

use mixer::control::*;
use mixer::command::Command;

fn main() {
    let ctrl = {
        let mut ctrl = Controller::new();
        for i in 1..100 {
            ctrl.sequence.push_back(
                Field{
                    cmd: Some(Command::from_str("2ff")),
                    note: Some(i as u8),
                });
        };
        Arc::new(Mutex::new(ctrl))
    };
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    display::run(&sdl, ctrl.clone());
}
