use crate::board::Board;

pub fn is_in_board(pos: (usize, usize)) -> bool {
    if !(pos.1 < 8) {
        println!("{} - uncorrect column", pos.1);
        println!("type a correct position around a-h: ");
        return false;
    }
    if !(pos.0 < 8) {
        println!("{} - uncorrect row", pos.0 + 1);
        println!("type a correct position around 1-8: ");
        return false;
    }

    return true;
}

#[allow(dead_code, unused)]
pub fn check_basic_rules(board: &Board, from: (usize, usize), to: (usize, usize), player: i8) -> bool {
    let valid_columns = ["a", "b", "c", "d", "e", "f", "g", "h"];

    // 1. check limits
    if !is_in_board(from) || !is_in_board(to) {
        println!("{}{} - impossible position", valid_columns[from.1], from.0 + 1);
        return false;
    }

    // 2. is the "from" a piece
    if board.get_cell(from) == 0 {
        println!("{}{} - is not a piece", valid_columns[from.1], from.0 + 1);
        return false;
    }

    // 3. is it player's piece
    if !(board.get_cell(from) == player || board.get_cell(from) == player + 1 || board.get_cell(from) == player - 1) {
        println!("{}{} - you don't own that piece", valid_columns[from.1], from.0 + 1);
        return false;
    }

    // 4. is the target occupated by player already
    if board.get_cell(to) == player {
        println!("{}{} is already occupated by you", valid_columns[from.1], from.0 + 1);
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
