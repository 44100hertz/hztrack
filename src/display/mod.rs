extern crate sdl2;
use sdl2::video::Window;
use sdl2::render::Canvas;

pub struct Display {
    canvas: Canvas<Window>,
}
impl Display {
    pub fn new(sdl: &sdl2::Sdl) -> Display {
        let video_subsys = sdl.video().unwrap();
        let win = video_subsys.window("rusttracker", 800, 600)
            .position_centered()
            .opengl()
            .resizable()
            .build().unwrap();
        Display {
            canvas: win.into_canvas()
                .accelerated()
                .present_vsync()
                .target_texture()
                .build().unwrap(),
        }
    }
}
