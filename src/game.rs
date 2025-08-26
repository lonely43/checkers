use crate::{board::Board, move_validator::is_valid_move};

pub struct Game {
    board: Board,
    player: i8, // 1 - white; -1 - black
}

impl Game {
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            player: 1, // white starts
        }
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) {
        if !is_valid_move(&self.board, from, to) {
            println!("Impossible move");
        }

        self.board.set_cell(from, 0);
        self.board.set_cell(to, self.player);

        self.change_turn();
    }

    pub fn is_game_over(&self) -> bool {
        false
    }

    fn change_turn(&mut self) {
        self.player = -self.player;
    }

    pub fn render(&self) {
        self.board.render();

        match self.player {
            1 => println!("●'s move: "),
            -1 => println!("○'s move: "),
            _ => println!("{}'s move: ", self.player),
        }
    }
}
