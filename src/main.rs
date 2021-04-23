extern crate sdl2;

mod game;
mod matrix;
mod mysdl;

use crate::mysdl::MySdl;
use game::Game;
use std::sync::mpsc::channel;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Instant;

pub fn main() {
    println!("Hello world!");

    let mut game = Arc::new(Game::new(1000, 1000));

    let mut sdl = MySdl::start_sdl();

    let barrier = Arc::new(Barrier::new(2));
    let barrier_gameticks = barrier.clone();

    let (tx, rx) = channel();

    thread::spawn(move || {
        while game.controls().running {
            tx.send(game.clone()).unwrap();
            if !game.controls().paused {
                let timer_tick = Instant::now();
                game.tick();
                let time_tick = timer_tick.elapsed().as_millis();
                println!("Tick took {}", time_tick);
            }

            barrier_gameticks.wait(); // wait for render thread to complete render
            barrier_gameticks.wait(); // wait for render thread to complete handling events

            if !game.controls().paused {
                let game = Arc::get_mut(&mut game).unwrap();
                game.finalize_tick();
            }

            barrier_gameticks.wait();

            std::thread::sleep(std::time::Duration::from_millis(game.speed()));
        }
    });

    'running: loop {
        let game = rx.recv().unwrap();

        let timer = Instant::now();
        game.draw(&mut sdl);
        let time_draw = timer.elapsed().as_millis();
        println!("Draw took {}", time_draw);

        if !game.controls().running {
            break 'running;
        }

        barrier.wait(); // wait for tick

        game.handle_events(&mut sdl);
        drop(game);

        barrier.wait(); // we are done

        barrier.wait(); // wait for tick to finalize

        let time_all = timer.elapsed().as_millis();

        println!("All took {}", time_all);

        // TODO: limit to 60 fps
    }
}
