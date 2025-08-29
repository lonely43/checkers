use crate::game::Game;
use crate::onboarding::{pick_piece, pick_target};

mod board;
mod game;
mod move_validator;
mod onboarding;

fn main() {
    let mut game = Game::new();

    loop {
        game.render();

        let from: (usize, usize) = pick_piece();
        let to: (usize, usize) = pick_target();

        match game.make_move(from, to) {
            Ok(_) => game.render(),
            Err(_) => continue,
        }

        if game.is_game_over() {
            // temporarily
            break;
        }
    }

    game.render();
}
