use crate::board::Board;

pub fn is_in_board(pos: (usize, usize)) -> bool {
    fn is_in(pos: usize) -> bool {
        return pos < 8;
    }

    if !is_in(pos.0) {
        println!("{} - uncorrect row", pos.0 + 1);
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
    // 1. check limits
    if !is_in_board(from) || !is_in_board(to) {
        println!("impossible positision");
        return false;
    }

    // 2. is the "from" a piece
    if board.get_cell(from) == 0 {
        println!("{},{} - it is not a piece", from.0 + 1, from.1);
        return false;
    }

    // 3. is it player's piece
    if !(board.get_cell(from) == player || board.get_cell(from) == player + 1 || board.get_cell(from) == player - 1) {
        println!("{},{} - choose your piece", from.0, from.1);
        return false;
    }

    // 4. is the target occupated by player already
    if board.get_cell(to) == player {
        println!("That cell is already occupated by you");
        return false;
    }

    // 5. check is it a diagonal move
    let row_diff = (to.0 as isize - from.0 as isize).abs();
    let col_diff = (to.1 as isize - from.1 as isize).abs();
    
    if !(row_diff == col_diff && row_diff > 0) {
        println!("isn't a diagonal move");
        return false;
    }

    true
}
