extern crate sdl2;

mod game;
mod matrix;
mod mysdl;

use game::Game;

use std::time::Instant;

pub fn main() {
    let mut game = Game::new(1000, 1000);
    run(&mut game);
}

pub fn run(game: &mut Game) {
    println!("Hello world!");

    while game.running {
        game.handle_events();

        if !game.paused {
            let t1 = Instant::now();
            game.draw();
            let time_draw = t1.elapsed().as_millis();

            let t2 = Instant::now();
            game.tick();
            let time_tick = t2.elapsed().as_millis();

            let time_all = t1.elapsed().as_millis();

            println!(
                "draw:{:4}    tick:{:4}    all:{:4}",
                time_draw, time_tick, time_all
            );
        }

        std::thread::sleep(std::time::Duration::from_millis(game.speed()));
    }
}
