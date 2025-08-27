use crate::board::Board;

fn is_in_board(pos: usize) -> bool{
    return pos < 8
}

#[allow(dead_code, unused)]
pub fn check_basic_rules(board: &Board, from: (usize, usize), to: (usize, usize), player: i8) -> bool {
    // check limits
    if !is_in_board(from.0) && !is_in_board(from.1) && !is_in_board(to.0) && !is_in_board(to.1) {
        println!("type a correct position");
        return false;
    }

    // is it piece
    if board.get_cell(from) == 0 {
        println!("{},{} - it is the empty cell", from.0, from.1);
        return false;
    }

    if board.get_cell(to) == player{
        println!("That cell is already occupated by you");
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
