use crate::board::Board;

pub fn is_in_board(pos: (usize, usize)) -> bool {
    fn is_in(pos: usize) -> bool {
        return pos < 8;
    }

    if !is_in(pos.0) {
        println!("{} - uncorrect row", pos.0+1);
        println!("type a correct position around 0-7: ");
        return false;
    }
    if !is_in(pos.1) {
        println!("{} - uncorrect column", pos.1);
        println!("type a correct position around a-h: ");
        return false;
    }

    return true;
}

#[allow(dead_code, unused)]
pub fn check_basic_rules(board: &Board, from: (usize, usize), to: (usize, usize), player: i8) -> bool {
    // check limits
    if !is_in_board(from) && !is_in_board(to) {
        println!("impossible positision");
        return false;
    }

    // is from a piece
    if board.get_cell(from) == 0 {
        println!("{},{} - it is not a piece", from.0+1, from.1);
        return false;
    }

    // is it player's piece
    if !(board.get_cell(from) == player || board.get_cell(from) == player + 1 || board.get_cell(from) == player - 1) {
        println!("{},{} - choose your piece", from.0, from.1);
        return false;
    }

    // is target valid
    if board.get_cell(to) == player {
        println!("That cell is already occupated by you");
        return false;
    }

    //check is it a diagonal move

    true
}
