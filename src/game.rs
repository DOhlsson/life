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
    camera: Point,
    zoom: f32,
    pub state: GameState,
}

pub struct GameState {
    pub paused: bool,
    pub running: bool,
    speed: usize,
    movecam: bool,
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
        let mut matrix = Matrix::new(cols, rows);
        let camera = Point::new(0, 0);

        let state = GameState {
            paused: false,
            speed: 0,
            running: true,
            movecam: false,
        };

        for x in 0..cols {
            for y in 0..rows {
                matrix.set(x as i32, y as i32, rng.gen_bool(0.5));
            }
        }

        return Game {
            sdl,
            cols,
            rows,
            data: matrix,
            camera,
            zoom: 1.0,
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
                Event::MouseWheel {y: 1, ..} => {
                    self.zoom *= 1.1;
                    self.sdl.canvas.set_scale(self.zoom, self.zoom).unwrap();
                }
                Event::MouseWheel {y: -1, ..} => {
                    self.zoom /= 1.1;
                    self.sdl.canvas.set_scale(self.zoom, self.zoom).unwrap();
                }
                Event::MouseMotion {xrel, yrel, ..} => {
                    if self.state.movecam {
                        self.camera = self.camera.offset(xrel, yrel);
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

    pub fn draw(&mut self) {
        self.sdl.canvas.set_draw_color(Color::BLACK);
        self.sdl.canvas.clear(); // investigate this

        for (i, b) in self.data.get_iter().enumerate() {
            let game_x = (i % self.rows) as i32;
            let game_y = (i / self.cols) as i32;

            let x = game_x * 10 + self.camera.x;
            let y = game_y * 10 + self.camera.y;

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
