use crate::board::Board;
#[allow(dead_code, unused)]
pub fn check_basic_rules(board: &Board, from: (usize, usize), to: (usize, usize), player: i8) -> bool {
    // check limits

    // is it piece
    if board.get_cell(from) == 0 {
        println!("{},{} - it is the empty cell", from.0, from.1);
        return false;
    }

    // is it player's piece
    if !(board.get_cell(from) == player || board.get_cell(from) == player+1 || board.get_cell(from) == player-1) {
        println!("{},{} - choose your piece", from.0, from.1);
        return false;
    }

    //check is it a diagonal move

    true
}
