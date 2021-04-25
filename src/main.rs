extern crate sdl2;

mod game;
mod matrix;
mod mysdl;

use crate::mysdl::MySdl;
use game::Game;
use game::Speed;
use sdl2::gfx::framerate::FPSManager;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

pub fn main() {
    println!("Hello world!");

    let mut sdl = MySdl::start_sdl();
    let display_index = sdl.canvas.window().display_index().unwrap();
    let framerate = sdl
        .video
        .current_display_mode(display_index)
        .unwrap()
        .refresh_rate;

    println!("Target fps: {}", framerate);

    let mut fps = FPSManager::new();
    fps.set_framerate(framerate as u32).unwrap();

    let mut game = Game::new(1000, 1000);
    game.randomize();

    let game = Arc::new(game);
    let game_clone = game.clone();

    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = barrier.clone();

    thread::spawn(move || {
        let game = game_clone;
        let barrier = barrier_clone;

        loop {
            let controls = game.controls_as_copy();

            if !controls.running {
                break;
            }

            if !controls.paused {
                let timer_tick = Instant::now();
                game.tick();
                let time_tick = timer_tick.elapsed().as_millis();

                println!("Tick took {}", time_tick);

                game.finalize_tick();
            }

            match controls.speed() {
                Speed::Unlimited => {}
                Speed::Lockstep => {
                    barrier.wait();
                }
                Speed::Limited(limit) => {
                    thread::sleep(Duration::from_millis(limit));
                }
            }
        }
    });

    loop {
        let controls = game.controls_as_copy();

        if !controls.running {
            break;
        }

        let timer = Instant::now();
        game.draw(&mut sdl);
        let time_draw = timer.elapsed().as_millis();

        println!("Draw took {}", time_draw);

        game.handle_events(&mut sdl);

        match controls.speed() {
            Speed::Lockstep => {
                barrier.wait();
            }
            _ => {}
        }

        fps.delay();
    }
}
