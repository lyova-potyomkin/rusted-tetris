use console::*;

mod game;
use game::Game;
use game::tetromino::Direction;

mod events;
use events::Event;

fn main() {
    let mut game = Game::new();
    let event = events::receiver();
    loop {
        match event.recv() {
            Ok(Event::Tick) => game.shift(Direction::Down),
            Ok(Event::Input(key)) => match key {
                'a' => game.shift(Direction::Left),
                'd' => game.shift(Direction::Right),
                'w' => game.turn(),
                's' => game.hard_drop(),
                _ => ()
            }
            _ => println!("Error!\r")
        }
        game.render();
    }
}
