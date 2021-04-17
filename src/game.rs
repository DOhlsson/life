use crate::mysdl::MySdl;
use crate::matrix::Matrix;

use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use rand::{thread_rng, Rng};

const ALIVE: Color = Color::RGB(0xEE, 0xEE, 0xEE);
const DEAD: Color  = Color::RGB(0x11, 0x11, 0x11);
const SPEEDS: [u64; 5]  = [0, 100, 500, 1000, 5000];

pub struct Game {
    sdl: MySdl,
    cols: usize,
    rows: usize,
    data: Matrix,
    camera: Camera,
    pub state: GameState,
}

// TODO: invert pos
struct Camera {
    pos: Point,
    float_pos: (f32, f32),
    pub zoom: f32,
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

// Should this exist? Should it be renamed? GameControls?
pub struct GameState {
    pub paused: bool,
    pub running: bool,
    speed: usize,
    movecam: bool,
    mouse: Point,
}

impl GameState {
    pub fn speed(&self) -> u64 {
        return SPEEDS[self.speed];
    }
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Game {
        let sdl = MySdl::start_sdl();

        let mut rng = thread_rng();
        let mut data = Matrix::new(cols, rows);
        let camera = Camera::new();

        let state = GameState {
            paused: false,
            speed: 0,
            running: true,
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
            data,
            camera,
            state,
        };
    }

    pub fn handle_events(&mut self) {
        for event in self.sdl.event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    println!("Bye!");
                    self.state.running = false;
                }
                Event::Window {win_event: WindowEvent::Resized(new_w, new_h), ..} => {
                    println!("Resized {} {}", new_w, new_h);
                    // sdl.scr_w = new_w as usize;
                    // sdl.scr_h = new_h as usize;
                }
                Event::KeyDown {keycode: Some(Keycode::P), ..} => {
                    self.state.paused = !self.state.paused;

                    if self.state.paused {
                        println!("Paused");
                    } else {
                        println!("Unpaused");
                    }
                }
                Event::KeyDown {keycode: Some(Keycode::Plus), ..} => {
                    if self.state.speed > 0 {
                        self.state.speed -= 1;
                        println!("New speed {}", self.state.speed());
                    }
                }
                Event::KeyDown {keycode: Some(Keycode::Minus), ..} => {
                    if self.state.speed < 4 {
                        self.state.speed += 1;
                        println!("New speed {}", self.state.speed());
                    }
                }
                Event::MouseWheel {y, ..} => {
                    Game::scroll_zoom(&mut self.camera, &self.state.mouse, y);
                    // self.camera = Game::scroll_zoom(&mut self.zoom, &self.camera, &self.state.mouse, y);
                    self.sdl.canvas.set_scale(self.camera.zoom, self.camera.zoom).unwrap();
                }
                Event::MouseMotion {xrel, yrel, x, y, ..} => {
                    self.state.mouse = Point::new(x, y);

                    if self.state.movecam {
                        self.camera.offset(-xrel as f32 / self.camera.zoom, -yrel as f32 / self.camera.zoom);
                    }
                }
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                    self.state.movecam = true;
                }
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {
                    self.state.movecam = false;
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
        let mut new_matrix = Matrix::new(self.cols, self.rows);

        for x in 0..self.cols {
            for y in 0..self.rows {
                let x = x as i32;
                let y = y as i32;

                let mut sum = 0;
                let alive = self.data.get(x, y);

                sum += self.data.get(x+1, y+1) as i32;
                sum += self.data.get(x, y+1) as i32;
                sum += self.data.get(x-1, y+1) as i32;

                sum += self.data.get(x+1, y) as i32;
                sum += self.data.get(x-1, y) as i32;

                sum += self.data.get(x+1, y-1) as i32;
                sum += self.data.get(x, y-1) as i32;
                sum += self.data.get(x-1, y-1) as i32;

                if alive && sum >= 2 && sum <= 3 {
                    new_matrix.set(x, y, true);
                } else if !alive && sum == 3 {
                    new_matrix.set(x, y, true);
                } else {
                    new_matrix.set(x, y, false);
                }
            }
        }

        self.data = new_matrix;
    }
}
