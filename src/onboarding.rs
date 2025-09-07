use anyhow::Result;
use std::io::{stdin, stdout, Write};

pub fn pick_piece() -> (usize, usize) {
    println!("choose piece: ");
    _ = stdout().flush();

    return get_cell(); // -> (row,col)
}

pub fn pick_target() -> (usize, usize) {
    println!("choose target: ");
    _ = stdout().flush();

    return get_cell(); // -> (row,col)
}

fn get_cell() -> (usize, usize) {
    loop {
        let vec: Vec<String> = get_input(); // -> [row, col]

        let row = get_row(vec[0].trim().to_string()).expect("enter usized type of number");
        let col: usize = get_col(vec[1].trim().to_string());

        // pipe

        return (row, col);
    }
}

fn get_input() -> Vec<String> {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("err in the readline");
    let array: Vec<&str> = input.split("").collect();
    let array = array[1..3].to_vec();
    let array = vec![array[1].to_string(), array[0].to_string()];
    array
}

fn get_col(input: String) -> usize {
    let valid_columns = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let mut i: usize = 0;

    let column: usize = loop {
        if input == valid_columns[i].to_string() {
            break i;
        }

        i += 1;
        if i > 7 {
            break 99; // ain't valid
        }
    };

    return column;
}
fn get_row(input: String) -> Result<usize> {
    let row: usize = input.parse()?;
    Ok(row - 1)
}
