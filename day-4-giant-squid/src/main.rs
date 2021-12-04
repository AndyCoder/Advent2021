use std::fs;

#[derive(Clone)]
struct Board {
    spaces: [Space; 25],
}

impl Board {
    fn new(input: String) -> Board {
        let mut spaces = [Space { value: 0 as u32, marked: false }; 25];
        for (i, x) in input.split_whitespace().enumerate() {
            spaces[i] = Space {
                value: u32::from_str_radix(x, 10).unwrap(),
                marked: false,
            }
        }
        Board {
            spaces: spaces,
        }
    }

    fn get_index(&self, row: usize, column: usize) -> Space {
        self.spaces[row * 5 + column]
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for x in self.spaces.iter() {
            if !x.marked {
                sum = sum + x.value
            }
        }
        sum 
    }

    fn score(&self, last: u32) -> u32 {
        self.sum_unmarked() * last
    }

    fn is_winner(&self) -> bool {
        for i in 0..5 {
            let mut row_wins = true;
            for j in 0..5 {
                if !self.get_index(i, j).marked {
                    row_wins = false;
                }
            }
            if row_wins {
                return true;
            }
        }
        for i in 0..5 {
            let mut col_wins = true;
            for j in 0..5 {
                if !self.get_index(j, i).marked {
                    col_wins = false;
                }
            }
            if col_wins {
                return true;
            }
        }
        false
    }

    fn mark(&mut self, draw: u32) {
        for space in self.spaces.iter_mut() {
            if space.value == draw {
                space.marked = true
            }
        }
    }
}

#[derive(Copy,Clone)]
struct Space {
    value: u32,
    marked: bool,
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();

    let draws: Vec<u32> = input.get(0).unwrap()
        .split(",")
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .collect();

    let mut boards: Vec<Board> = input.iter().skip(1)
        .map(|s| Board::new(s.to_string()))
        .collect();

    for (turn, draw) in draws.iter().enumerate() {
        for board in boards.iter_mut() {
            board.mark(*draw);
        }
        let filtered_boards: Vec<&Board> = boards.iter().filter(|x| !x.is_winner()).collect();
        println!("draw = {}, turn = {}, boards = {}", draw, turn, filtered_boards.len());
        if filtered_boards.len() == 1 {
            let mut final_board: Board = filtered_boards[0].clone();
            for (n, draw) in draws.iter().skip(turn + 1).enumerate() {
                final_board.mark(*draw);
                if final_board.is_winner() {
                    println!("score = {}, draw = {}, turn = {}", final_board.score(*draw), draw, turn + n + 1);
                    std::process::exit(0)
                }
            }
        }
    }
}
