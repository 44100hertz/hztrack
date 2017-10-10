extern crate sdl2;

mod base32;
mod mixer;
mod display;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl);
    display::run(&sdl);
}
