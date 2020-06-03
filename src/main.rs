mod snake;
mod drawing;

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut game = snake::game::Game::new();
    
    let mut previous_update = 0;
    loop {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();
        let elapsed_ms = now - previous_update;

        if elapsed_ms > 16 {
            previous_update = now;

            game.update();
        }
    }
}

fn draw() {
    for y in 0..30 {
        for x in 0..30 {
            print!(" ");
        }
        print!("\n");
    }
}