use std::fs;

struct Instruction {
    op: Operation,
    arg: isize,
}

enum Operation {
    Forward,
    Up,
    Down,
    InvalidOp,
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect();

    let instructions: Vec<Instruction> = input.iter()
        .map(|s| s.split_whitespace().map(|s| s.to_owned()).collect())
        .map(|split: Vec<String>| Instruction {
            op: match split[0].as_str() {
                "forward" => Operation::Forward,
                "up" => Operation::Up,
                "down" => Operation::Down,
                _ => Operation::InvalidOp,
            },
            arg: split[1].parse::<isize>().unwrap(),
        }).collect();

    let mut h_pos: isize = 0;
    let mut depth: isize = 0;
    for (index, inst) in instructions.iter().enumerate() {
        match inst.op {
            Operation::Forward => h_pos = h_pos + inst.arg,
            Operation::Up => depth = depth - inst.arg,
            Operation::Down => depth = depth + inst.arg,
            Operation::InvalidOp => println!("Invalid operation detected at line {}", index)
        }
    };
    println!("{} * {} = {}", h_pos, depth, h_pos * depth);
    std::process::exit(0)
}
