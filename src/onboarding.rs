use crate::move_validator::rules::is_in_board;
use std::io::{stdin, stdout, Write};

pub fn pick_piece() -> (usize, usize) {
    println!("choose piece: ");
    _ = stdout().flush();
    loop {
        let from = (get_row(), get_pos());
        if is_in_board(from) {
            return from;
        }
    }
}

pub fn pick_target() -> (usize, usize) {
    println!("choose target: ");
    _ = stdout().flush();
    loop {
        let to = (get_row(), get_pos());
        if is_in_board(to) {
            return to;
        }
    }
}

fn get_row() -> usize {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("err in the readline");
        let input: usize = match input.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!("enter usized type of number");
                continue;
            }
        };
        return input - 1;
    }
}

fn get_pos() -> usize {
    let columns = ["a", "b", "c", "d", "e", "f", "g", "h"];
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("err in the readline");

        for i in 0..8 {
            if input.trim() == columns[i].to_string() {
                return i;
            }
        }

        println!("choose column in range a-h")
    }
}
