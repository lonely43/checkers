// use std::io::{stdin, stdout, Write};

use game::Game;

mod board;
mod game;
mod move_validator;

#[allow(dead_code)]
fn main() {
    let mut game = Game::new();

    loop {
        game.render();

        {
            // testing
            game.make_move((5, 0), (4, 1));
            game.render();
            game.make_move((2, 1), (3, 2));
        }

        if !game.is_game_over() {
            // temporarily | checking if the game is over
            break;
        }
    }

    game.render();
}
