#[derive(Debug)]
pub struct Board {
    cells: [[i8; 8]; 8],
}

#[allow(dead_code)]
impl Board {
    pub fn new() -> Self {
        let mut cells = [[0; 8]; 8];

        for row in 0..8 {
            for col in 0..8 {
                if (row + col) % 2 == 1 {
                    if row < 3 {
                        // black piece
                        cells[row][col] = -1;
                    } else if row > 4 {
                        // white piece
                        cells[row][col] = 1;
                    }
                }
            }
        }

        Board { cells }
    }

    pub fn get_cells(&self) -> [[i8; 8]; 8] {
        self.cells
    }

    pub fn get_cell(&self, pos: (usize, usize)) -> i8 {
        self.cells[pos.0][pos.1]
    }

    pub fn set_cell(&mut self, pos: (usize, usize), new_value: i8) {
        self.cells[pos.0][pos.1] = new_value;
    }

    pub fn render(&self) {
        println!("    a  b  c  d  e  f  g  h   ");
        println!("  +------------------------+");

        for (row, cells_in_col) in self.cells.iter().enumerate() {
            print!("{} |", row);

            for (col, cell) in cells_in_col.iter().enumerate() {
                let symbol = match cell {
                    1 => " ● ",
                    -1 => " ○ ",
                    _ => {
                        if (row + col) % 2 == 0 {
                            "░░░"
                        } else {
                            "   "
                        }
                    }
                };
                print!("{}", symbol);
            }

            println!("| {}", row);
        }

        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h   ");
    }

    pub fn debug_render(&self) {
        println!("    a  b  c  d  e  f  g  h   ");
        println!("  +------------------------+");

        for (row, cells_in_col) in self.cells.iter().enumerate() {
            print!("{} |", 8 - row);

            for (col, cell) in cells_in_col.iter().enumerate() {
                let symbol = match cell {
                    1 => " 1 ",
                    -1 => "-1 ",
                    _ => {
                        if (row + col) % 2 == 0 {
                            "░░░"
                        } else {
                            "   "
                        }
                    }
                };
                print!("{}", symbol);
            }

            println!("| {}", 8 - row);
        }

        println!("  +------------------------+");
        println!("    a  b  c  d  e  f  g  h   ");
    }
}
