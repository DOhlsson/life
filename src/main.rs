extern crate sdl2;

mod game;
mod matrix;
mod mysdl;

use crate::mysdl::MySdl;
use game::Game;
use sdl2::gfx::framerate::FPSManager;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

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

    let game = Arc::new(Game::new(1000, 1000));
    let game_clone = game.clone();

    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = barrier.clone();

    thread::spawn(move || {
        let game = game_clone;
        let barrier = barrier_clone;

        loop {
            let paused = game.controls().paused;

            std::thread::sleep(std::time::Duration::from_millis(game.speed()));

            if !game.controls().running {
                break;
            }

            if !paused {
                let timer_tick = Instant::now();
                game.tick();
                let time_tick = timer_tick.elapsed().as_millis();
                println!("Tick took {}", time_tick);
            }

            if !paused {
                game.finalize_tick();
            }

            // barrier.wait(); // lockstep
        }
    });

    loop {
        let timer = Instant::now();
        game.draw(&mut sdl);
        let time_draw = timer.elapsed().as_millis();
        println!("Draw took {}", time_draw);

        if !game.controls().running {
            break;
        }

        game.handle_events(&mut sdl);

        // barrier.wait(); // lockstep

        let time_all = timer.elapsed().as_millis();
        println!("All took {}\n", time_all);

        fps.delay();
    }
}
