extern crate sdl2;

mod game;
mod matrix;
mod mysdl;

use crate::mysdl::MySdl;
use game::Game;
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier, RwLock};
use std::thread;
use std::time::Instant;

pub fn main() {
    println!("Hello world!");

    let mut game = Arc::new(RwLock::new(Game::new(1000, 1000)));
    let mut game_clone = game.clone();

    let mut sdl = MySdl::start_sdl();

    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = barrier.clone();

    //let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            let game_r = game_clone.read().unwrap();
            let paused = game_r.controls().paused;

            std::thread::sleep(std::time::Duration::from_millis(game_r.speed()));

            if !game_r.controls().running {
                break;
            }

            if !paused {
                let timer_tick = Instant::now();
                game_r.tick();
                let time_tick = timer_tick.elapsed().as_millis();
                println!("Tick took {}", time_tick);
            }

            drop(game_r);

            if !paused {
                let mut game_w = game_clone.write().unwrap();
                game_w.finalize_tick();
            }

            barrier_clone.wait(); // lockstep
        }
    });

    loop {
        let game_r = game.read().unwrap();

        let timer = Instant::now();
        game_r.draw(&mut sdl);
        let time_draw = timer.elapsed().as_millis();
        println!("Draw took {}", time_draw);

        if !game_r.controls().running {
          break;
        }

        game_r.handle_events(&mut sdl);
        drop(game_r);


        barrier.wait(); // lockstep

        let time_all = timer.elapsed().as_millis();
        println!("All took {}", time_all);



        // TODO: limit to 60 fps
    }
}
