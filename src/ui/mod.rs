use std::sync::{Mutex, Arc};
use mixer::control::Controller;

extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::render::*;
use sdl2::video::Window;
use sdl2::rect::*;

mod keyboard;
use self::keyboard::Keyboard;

const CHAR_W: i32 = 8;
const CHAR_H: i32 = 8;

pub struct Artist<'tex> {
    canvas: Canvas<Window>,
    font: Texture<'tex>,
    scale: u32,
}

impl<'tex> Artist<'tex> {
    fn clear(&mut self) {
        self.canvas.set_draw_color(Color{r:0, g:64, b:128, a:255});
        self.canvas.clear();
    }
    fn present(&mut self) {
        self.canvas.present();
    }
    fn print(&mut self, mut x: i32, y: i32, chars: &str) {
        for c in chars.as_bytes() {
            let src = Rect::new(
                (*c as i32 % 16) * CHAR_W,
                (*c as i32 / 16) * CHAR_H,
                CHAR_W as u32,
                CHAR_H as u32);
            let dest = Rect::new(
                x * CHAR_W * self.scale as i32,
                y * CHAR_H * self.scale as i32,
                CHAR_W as u32 * self.scale,
                CHAR_H as u32 * self.scale);
            self.canvas.copy(&self.font, Some(src), Some(dest)).unwrap();
            x += 1;
        }
    }
    fn playback_line(&mut self, row: i32) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.draw_rect(Rect::new(
                0, row * CHAR_H * self.scale as i32,
                12345678, CHAR_H as u32 * self.scale as u32))
            .unwrap();
    }
    fn cursor(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.canvas.set_draw_color(Color::RGB(128, 0, 0));
        self.canvas.fill_rect(Rect::new(
                x * CHAR_W * self.scale as i32 - 1,
                y * CHAR_H * self.scale as i32 - 1,
                w * CHAR_W as u32 * self.scale + 2,
                h * CHAR_H as u32 * self.scale + 2))
            .unwrap();
    }
    fn bg(&mut self, x: u32, y: u32) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.fill_rect(Rect::new(
                0, 0,
                x as u32 * CHAR_W as u32 * self.scale,
                y as u32 * CHAR_H as u32 * self.scale))
            .unwrap();
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

    let font = {
        use sdl2::surface::Surface;
        let mut surf = Surface::load_bmp("res/font.bmp").unwrap();
        surf.set_color_key(true, Color::RGB(0, 0, 0)).unwrap();
        tex_creator.create_texture_from_surface(surf).unwrap()
    };

    let mut artist = Artist{
        canvas: canvas,
        font: font,
        scale: 6,
    };

    let mut event_pump = sdl.event_pump().unwrap();
    let mut keyboard = Keyboard::new();
    'main: loop {
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}  => break 'main,
                Event::KeyDown{scancode, ..} => {
                    keyboard.handle_key(
                        scancode.unwrap(),
                        ctrl.lock().unwrap());
                },
                _ => {},
            }
        }
        artist.clear();
        {
            let c = ctrl.lock().unwrap();
            artist.bg(c.width() as u32, c.height() as u32);
            {
                let (x, y, w, h) = c.cursor();
                artist.cursor(x, y, w, h);
            }
            for (y, ref row) in c.sequence.iter().enumerate() {
                for (x, ref field) in row.iter().enumerate() {
                    artist.print(x as i32 * c.field_w() as i32, y as i32, &field.string());
                }
            }
            artist.playback_line(c.row() as i32);
        }
        artist.present();
    }
}
