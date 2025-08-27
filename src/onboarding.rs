use std::io::{stdin};

pub fn pick_piece() -> (usize, usize) {
    (get_pos(), get_pos())
}

pub fn pick_target() -> (usize, usize) {
    (get_pos(), get_pos())
}

fn get_pos() -> usize {
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("err in the readline");
        let input: usize = match input.trim().parse() {
            Ok(result) => result,
            Err(_) => {
                println!(" usized type of number");
                continue;
            }
        };
        return input;
    }
}
