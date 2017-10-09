extern crate sdl2;
use sdl2::pixels::Color;

pub fn run(sdl: sdl2::Sdl) {
    let video_subsys = sdl.video().unwrap();
    let win = video_subsys.window("rusttracker", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build().unwrap();
    let mut canvas = win.into_canvas()
        .accelerated()
        .present_vsync()
        .target_texture()
        .build().unwrap();
    let tex_creator = canvas.texture_creator();
    let font = tex_creator.create_texture_from_surface(
        sdl2::surface::Surface::load_bmp("res/font.bmp").unwrap()).unwrap();
    loop {
        canvas.set_draw_color(Color{r:0, g:0, b:255, a:255});
        canvas.clear();
        canvas.copy(&font, None, None).unwrap();
        canvas.present();
    }
}
