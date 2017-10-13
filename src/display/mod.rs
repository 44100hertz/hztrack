use std::sync::{Mutex, Arc};
use mixer::control::Controller;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::Window;
use sdl2::rect::*;

const CHAR_W: i32 = 8;
const CHAR_H: i32 = 8;

pub struct Artist<'tex> {
    canvas: Canvas<Window>,
    font: Texture<'tex>,
    scale: u32,
}

impl<'tex> Artist<'tex> {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color{r:0, g:0, b:255, a:255});
        self.canvas.clear();
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn write(&mut self, mut x: i32, y: i32, chars: &str) {
        for c in chars.as_bytes() {
            let src = Rect::new(
                (*c as i32 % 16) * CHAR_W,
                (*c as i32 / 16) * CHAR_H,
                CHAR_W as u32,
                CHAR_H as u32);
            let dest = Rect::new(
                x * self.scale as i32,
                y * self.scale as i32,
                CHAR_W as u32 * self.scale,
                CHAR_H as u32 * self.scale);
            self.canvas.copy(&self.font, Some(src), Some(dest)).unwrap();
            x += CHAR_W;
        }
    }
}

pub fn run(sdl: &sdl2::Sdl, ctrl: Arc<Mutex<Controller>>) {
    let video_subsys = sdl.video().unwrap();
    let win = video_subsys.window("rusttracker", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build().unwrap();
    let canvas = win.into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build().unwrap();
    let tex_creator = canvas.texture_creator();

    use sdl2::surface::Surface;
    let font = tex_creator.create_texture_from_surface(
        Surface::load_bmp("res/font.bmp").unwrap())
        .unwrap();

    let mut artist = Artist{
        canvas: canvas,
        font: font,
        scale: 3,
    };

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,
                _ => {},
            }
        }
        let mut y = 0;
        artist.clear();
        for ref field in ctrl.lock().unwrap().sequence.iter() {
            artist.write(0, y, &field.string());
            y += CHAR_H;
        }
        artist.present();
    }
}
