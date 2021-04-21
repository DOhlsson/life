use crate::matrix::Matrix;
use crate::mysdl::MySdl;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use rand::{thread_rng, Rng};

use std::rc::Rc;

const ALIVE: Color = Color::RGB(0xEE, 0xEE, 0xEE);
const DEAD: Color = Color::RGB(0x11, 0x11, 0x11);
const SPEEDS: [u64; 5] = [0, 100, 500, 1000, 5000];

pub struct Game {
    sdl: MySdl,
    cols: usize,
    rows: usize,
    data: Rc<Matrix>,
    next_data: Rc<Matrix>,
    camera: Camera,
    controls: Controls,
    pub paused: bool,
    pub running: bool,
}

// TODO: invert pos
struct Camera {
    pos: Point,
    float_pos: (f32, f32),
    pub zoom: f32,
}

pub struct Controls {
    speed: usize,
    movecam: bool,
    mouse: Point,
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
}

impl Controls {
    pub fn speed(&self) -> u64 {
        return SPEEDS[self.speed];
    }
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Game {
        let sdl = MySdl::start_sdl();

        let mut rng = thread_rng();
        let mut data = Matrix::new(cols, rows);
        let mut next_data = Matrix::new(cols, rows);
        let camera = Camera::new();

        let controls = Controls {
            speed: 0,
            movecam: false,
            mouse: Point::new(0, 0),
        };

        // Mapgen
        for x in 0..cols {
            for y in 0..rows {
                data.set(x as i32, y as i32, rng.gen_bool(0.5));
            }
        }

        return Game {
            sdl,
            cols,
            rows,
            data: Rc::new(data),
            next_data: Rc::new(next_data),
            camera,
            controls,
            paused: false,
            running: true,
        };
    }

    pub fn speed(&self) -> u64 {
        return self.controls.speed();
    }

    pub fn handle_events(&mut self) {
        for event in self.sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Bye!");
                    self.running = false;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(new_w, new_h),
                    ..
                } => {
                    println!("Resized {} {}", new_w, new_h);
                    // sdl.scr_w = new_w as usize;
                    // sdl.scr_h = new_h as usize;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    self.paused = !self.paused;

                    if self.paused {
                        println!("Paused");
                    } else {
                        println!("Unpaused");
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Plus),
                    ..
                } => {
                    if self.controls.speed > 0 {
                        self.controls.speed -= 1;
                        println!("New speed {}", self.controls.speed());
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Minus),
                    ..
                } => {
                    if self.controls.speed < 4 {
                        self.controls.speed += 1;
                        println!("New speed {}", self.controls.speed());
                    }
                }
                Event::MouseWheel { y, .. } => {
                    Game::scroll_zoom(&mut self.camera, &self.controls.mouse, y);
                    // self.camera = Game::scroll_zoom(&mut self.zoom, &self.camera, &self.state.mouse, y);
                    self.sdl
                        .canvas
                        .set_scale(self.camera.zoom, self.camera.zoom)
                        .unwrap();
                }
                Event::MouseMotion {
                    xrel, yrel, x, y, ..
                } => {
                    self.controls.mouse = Point::new(x, y);

                    if self.controls.movecam {
                        self.camera.offset(
                            -xrel as f32 / self.camera.zoom,
                            -yrel as f32 / self.camera.zoom,
                        );
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    self.controls.movecam = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    self.controls.movecam = false;
                }
                _ => {}
            }
        }
    }

    // TODO: can this function be made cleaner?
    fn scroll_zoom(camera: &mut Camera, mouse: &Point, scroll: i32) {
        let prev_zoom = camera.zoom;

        if scroll == 1 {
            camera.zoom *= 1.2;
        } else if scroll == -1 {
            camera.zoom /= 1.2;
        }

        let off_x = mouse.x as f32 / prev_zoom - mouse.x as f32 / camera.zoom;
        let off_y = mouse.y as f32 / prev_zoom - mouse.y as f32 / camera.zoom;

        camera.offset(off_x, off_y);
    }

    pub fn draw(&mut self) {
        self.sdl.canvas.set_draw_color(Color::BLACK);
        self.sdl.canvas.clear(); // investigate this

        for (i, b) in self.data.get_iter().enumerate() {
            let game_x = (i % self.rows) as i32;
            let game_y = (i / self.cols) as i32;

            let x = game_x * 10 - self.camera.pos().x;
            let y = game_y * 10 - self.camera.pos().y;

            let rect = Rect::new(x, y, 9, 9);

            if *b {
                self.sdl.canvas.set_draw_color(ALIVE);
            } else {
                self.sdl.canvas.set_draw_color(DEAD);
            }

            self.sdl.canvas.fill_rect(rect).unwrap();
        }

        self.sdl.canvas.present();
    }

    pub fn tick(&mut self) {
        let next_data = Rc::get_mut(&mut self.next_data).unwrap();

        for x in 0..self.cols {
            for y in 0..self.rows {
                let x = x as i32;
                let y = y as i32;

                let mut sum = 0;
                let alive = self.data.get(x, y);

                sum += self.data.get(x + 1, y + 1) as i32;
                sum += self.data.get(x, y + 1) as i32;
                sum += self.data.get(x - 1, y + 1) as i32;

                sum += self.data.get(x + 1, y) as i32;
                sum += self.data.get(x - 1, y) as i32;

                sum += self.data.get(x + 1, y - 1) as i32;
                sum += self.data.get(x, y - 1) as i32;
                sum += self.data.get(x - 1, y - 1) as i32;

                if alive && sum >= 2 && sum <= 3 {
                    next_data.set(x, y, true);
                } else if !alive && sum == 3 {
                    next_data.set(x, y, true);
                } else {
                    next_data.set(x, y, false);
                }
            }
        }

        let old_data = self.data.clone();
        self.data = self.next_data.clone();
        self.next_data = old_data;
    }
}
