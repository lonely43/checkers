mod moves;
mod rules;

use crate::board::Board;

#[allow(dead_code, unused)]
pub fn is_valid_move(board: &Board, from: (usize, usize), to: (usize, usize), player: i8) -> bool {
    return rules::check_basic_rules(board, from, to, player) && moves::is_valid_piece_move(board, from, to);
}
#[allow(dead_code, unused)]
pub fn get_all_valid_moves(board: &Board, player: i8) -> () { // -> Vec<(i8,i8)>
    // logic
}
