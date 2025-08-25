mod board;

use board::Board;

#[allow(dead_code)]
fn main() {
    let board: Board = Board::new();

  //  board.render();

    board.debug_render();
}
