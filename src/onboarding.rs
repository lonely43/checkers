use crate::move_validator::rules::is_in_board;
use std::io::{stdin, stdout, Write};

pub fn pick_piece() -> (usize, usize) {
    println!("choose piece: ");
    _ = stdout().flush();

    loop {
        let from: (usize, usize) = get_piece(); // -> (row,col)
        if is_in_board(from) {
            return from;
        }
    }
}

fn get_piece() -> (usize, usize) {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("err in the readline");
        let array: Vec<&str> = input.split("").collect();
        let pos = array[1..3].to_vec();

        // is valid
        if pos.len() > 2 {
            println!("input's length is more than 2");
            continue;
        }

        // column
        let col: String = pos[0].trim().to_string();

        // is valid column
        let valid_columns = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let mut i: usize = 0;
        let col: usize = loop {
            if col == valid_columns[i].to_string() {
                break i;
            }

            i = i+1;
            if i > 8 {
                break 99; // not a valid
            }
        };
        if col > 8 {
            println!("choose column in range a-h");
            continue;
        }
        
        //row 
        let row: usize = match pos[1].trim().to_string().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("enter usized type of number");
                continue;
            }
        };
        let row = row - 1;

        return (row, col);
    }
}

pub fn pick_target() -> (usize, usize) {
    println!("choose target: ");
    _ = stdout().flush();
    loop {
        let to = get_piece();
        if is_in_board(to) {
            return to;
        }
    }
}
