const SCREEN_W: usize = 800;
const SCREEN_H: usize = 600;

pub struct MySdl {
    pub context: sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::WindowCanvas,
    pub scr_w: usize,
    pub scr_h: usize,
    pub scale: f32,
}

impl MySdl {
    pub fn start_sdl() -> MySdl {
        let context = sdl2::init().unwrap();
        let event_pump = context.event_pump().unwrap();
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo", SCREEN_W as u32, SCREEN_H as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.present();

        return MySdl {
            context,
            event_pump,
            canvas,
            scr_w: SCREEN_W,
            scr_h: SCREEN_H,
            scale: 1.0,
        };
    }
}
