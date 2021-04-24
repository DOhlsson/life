const SCREEN_W: usize = 800;
const SCREEN_H: usize = 600;

use sdl2::rect::Point;

pub struct MySdl {
    pub context: sdl2::Sdl,
    pub event_pump: sdl2::EventPump,
    pub video: sdl2::VideoSubsystem,
    pub canvas: sdl2::render::WindowCanvas,
    pub camera: Camera,
    pub scr_w: usize,
    pub scr_h: usize,
    pub scale: f32,
}

pub struct Camera {
    pos: Point,
    float_pos: (f32, f32),
    pub zoom: f32,
}

impl MySdl {
    pub fn start_sdl() -> MySdl {
        let context = sdl2::init().unwrap();
        let event_pump = context.event_pump().unwrap();
        let video = context.video().unwrap();
        let camera = Camera::new();

        let window = video
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
            video,
            canvas,
            camera,
            scr_w: SCREEN_W,
            scr_h: SCREEN_H,
            scale: 1.0,
        };
    }
}

impl Camera {
    pub fn new() -> Camera {
        return Camera {
            pos: Point::new(0, 0),
            float_pos: (0.0, 0.0),
            zoom: 1.0,
        };
    }

    pub fn pos(&self) -> Point {
        return self.pos;
    }

    pub fn fpos(&self) -> (f32, f32) {
        return self.float_pos;
    }

    pub fn set(&mut self, x: f32, y: f32) {
        self.float_pos.0 = x;
        self.float_pos.1 = y;

        self.pos = Point::new(x as i32, y as i32);
    }

    pub fn offset(&mut self, x: f32, y: f32) {
        self.float_pos.0 += x;
        self.float_pos.1 += y;

        self.pos = Point::new(self.float_pos.0 as i32, self.float_pos.1 as i32);
    }

    pub fn scroll_zoom(&mut self, mouse: &Point, scroll: i32) {
        let prev_zoom = self.zoom;

        if scroll == 1 {
            self.zoom *= 1.2;
        } else if scroll == -1 {
            self.zoom /= 1.2;
        }

        let off_x = mouse.x as f32 / prev_zoom - mouse.x as f32 / self.zoom;
        let off_y = mouse.y as f32 / prev_zoom - mouse.y as f32 / self.zoom;

        self.offset(off_x, off_y);
    }
}
