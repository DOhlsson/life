use super::life_matrix::LifeMatrix;
use super::mysdl::MySdl;
use rand::{thread_rng, Rng};
use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

const ALIVE: Color = Color::RGB(0xEE, 0xEE, 0xEE);
const DEAD: Color = Color::RGB(0x11, 0x11, 0x11);
const SPEEDS: [Speed; 5] = [
    Speed::Unlimited,
    Speed::Lockstep,
    Speed::Limited(100),
    Speed::Limited(500),
    Speed::Limited(1000),
];

pub struct Game {
    cols: usize,
    rows: usize,
    state: RwLock<GameState>,
    controls: Mutex<Controls>,
}

pub struct GameState {
    data: Arc<LifeMatrix>,
    next_data: Mutex<Arc<LifeMatrix>>,
}

#[derive(Copy, Clone)]
pub struct Controls {
    pub running: bool,
    pub paused: bool,
    movecam: bool,
    drawing: Option<bool>,
    speed: usize,
    mouse: Point,
}

#[derive(Copy, Clone, Debug)]
pub enum Speed {
    Unlimited,
    Lockstep,
    Limited(u64),
}

impl Controls {
    pub fn speed(&self) -> Speed {
        return SPEEDS[self.speed];
    }
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Game {
        let mut data = LifeMatrix::new(cols, rows);
        let next_data = LifeMatrix::new(cols, rows);

        let controls = Controls {
            running: true,
            paused: false,
            movecam: false,
            drawing: None,
            speed: 1,
            mouse: Point::new(0, 0),
        };

        let state = GameState {
            data: Arc::new(data),
            next_data: Mutex::new(Arc::new(next_data)),
        };

        return Game {
            cols,
            rows,
            state: RwLock::new(state),
            controls: Mutex::new(controls),
        };
    }

    pub fn randomize(&mut self) {
        let mut rng = thread_rng();

        let mut state = self.state.write().unwrap();
        let data = Arc::get_mut(&mut state.data).unwrap();
        for x in 0..self.cols {
            for y in 0..self.rows {
                data.set(x as i32, y as i32, rng.gen_bool(0.5));
            }
        }
    }

    pub fn controls_as_mutex(&self) -> MutexGuard<'_, Controls> {
        return self.controls.lock().unwrap();
    }

    pub fn controls_as_copy(&self) -> Controls {
        let mutex_lock = self.controls.lock().unwrap();
        let controls_copy: Controls = *mutex_lock;
        drop(mutex_lock);
        return controls_copy;
    }

    pub fn handle_events(&self, sdl: &mut MySdl) {
        let mut controls = self.controls_as_mutex();

        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    println!("Bye!");
                    controls.running = false;
                }
                Event::Window {
                    win_event: WindowEvent::Resized(new_w, new_h),
                    ..
                } => {
                    println!("Resized {} {}", new_w, new_h);
                    sdl.scr_w = new_w as usize;
                    sdl.scr_h = new_h as usize;
                }
                Event::MouseWheel { y, .. } => {
                    sdl.camera.scroll_zoom(&controls.mouse, y);
                    sdl.canvas
                        .set_scale(sdl.camera.zoom, sdl.camera.zoom)
                        .unwrap();
                }
                Event::MouseMotion {
                    xrel, yrel, x, y, ..
                } => {
                    controls.mouse = Point::new(x, y);

                    if controls.movecam {
                        sdl.camera.offset(
                            -xrel as f32 / sdl.camera.zoom,
                            -yrel as f32 / sdl.camera.zoom,
                        );
                    }
                }
                Event::MouseButtonDown { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Right => {
                        controls.movecam = true;
                    }
                    MouseButton::Left => {
                        let camera_pos = sdl.camera.pos();
                        let map_x = ((camera_pos.x as f32
                            + controls.mouse.x as f32 / sdl.camera.zoom)
                            / 10.0) as i32;
                        let map_y = ((camera_pos.y as f32
                            + controls.mouse.y as f32 / sdl.camera.zoom)
                            / 10.0) as i32;

                        let state = self.state.read().unwrap();

                        controls.drawing = Some(!state.data.get(map_x, map_y));
                    }
                    _ => {}
                },
                Event::MouseButtonUp { mouse_btn, .. } => match mouse_btn {
                    MouseButton::Right => {
                        controls.movecam = false;
                    }
                    MouseButton::Left => {
                        controls.drawing = None;
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::P => {
                        controls.paused = !controls.paused;
                        if controls.paused {
                            println!("Paused");
                        } else {
                            println!("Unpaused");
                        }
                    }
                    Keycode::Plus => {
                        if controls.speed > 0 {
                            controls.speed -= 1;
                            println!("New speed: {:?}", controls.speed());
                        }
                    }
                    Keycode::Minus => {
                        if controls.speed < SPEEDS.len() - 1 {
                            controls.speed += 1;
                            println!("New speed: {:?}", controls.speed());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }

            match controls.drawing {
                Some(draw) => {
                    let mut state = self.state.write().unwrap();
                    let camera_pos = sdl.camera.pos();
                    let map_x = ((camera_pos.x as f32 + controls.mouse.x as f32 / sdl.camera.zoom)
                        / 10.0) as i32;
                    let map_y = ((camera_pos.y as f32 + controls.mouse.y as f32 / sdl.camera.zoom)
                        / 10.0) as i32;

                    let data = Arc::get_mut(&mut state.data).unwrap();
                    data.set(map_x, map_y, draw);
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self, sdl: &mut MySdl) {
        sdl.canvas.set_draw_color(Color::BLACK);
        sdl.canvas.clear(); // investigate this

        let state = self.state.read().unwrap();

        let camera_pos = sdl.camera.pos();

        let scr_w = (sdl.scr_w as f32 / sdl.camera.zoom) as i32;
        let scr_h = (sdl.scr_h as f32 / sdl.camera.zoom) as i32;

        for (i, b) in state.data.get_iter().enumerate() {
            let game_x = (i % self.rows) as i32;
            let game_y = (i / self.cols) as i32;

            let x = game_x * 10 - camera_pos.x;
            let y = game_y * 10 - camera_pos.y;

            if x >= -10 && y >= -10 && x <= scr_w && y <= scr_h {
                let rect = Rect::new(x, y, 9, 9);

                if *b {
                    sdl.canvas.set_draw_color(ALIVE);
                } else {
                    sdl.canvas.set_draw_color(DEAD);
                }

                sdl.canvas.fill_rect(rect).unwrap();
            }
        }

        sdl.canvas.present();
    }

    pub fn tick(&self) {
        let state = self.state.read().unwrap();
        let mut next_data = state.next_data.lock().unwrap();
        let next_data = Arc::get_mut(&mut next_data).unwrap();

        for x in 0..self.cols {
            for y in 0..self.rows {
                let x = x as i32;
                let y = y as i32;

                // TODO calculate neighbors as a function of LifeMatrix
                let mut sum = 0;
                let alive = state.data.get(x, y);

                sum += state.data.get(x + 1, y + 1) as i32;
                sum += state.data.get(x, y + 1) as i32;
                sum += state.data.get(x - 1, y + 1) as i32;

                sum += state.data.get(x + 1, y) as i32;
                sum += state.data.get(x - 1, y) as i32;

                sum += state.data.get(x + 1, y - 1) as i32;
                sum += state.data.get(x, y - 1) as i32;
                sum += state.data.get(x - 1, y - 1) as i32;

                if alive && sum >= 2 && sum <= 3 {
                    next_data.set(x, y, true);
                } else if !alive && sum == 3 {
                    next_data.set(x, y, true);
                } else {
                    next_data.set(x, y, false);
                }
            }
        }

        drop(next_data);
    }

    pub fn finalize_tick(&self) {
        let mut state = self.state.write().unwrap();

        // Acquire a MutexGuard for next_data
        // this also acquires an immutable borrow of state, we must drop this later
        let mut next_data = state.next_data.lock().unwrap();

        let new_data = next_data.clone(); // clone for later

        *next_data = state.data.clone(); // write the cloned Arc of state.data into the next_data mutex

        drop(next_data); // drops the immutable borrow of state

        state.data = new_data;
    }
}
