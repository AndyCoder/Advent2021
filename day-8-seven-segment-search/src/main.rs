use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let lines: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut count = 0;
    for line in lines.iter() {
        let _signals = line.split("|").nth(0).unwrap().split_whitespace();
        let output = line.split("|").nth(1).unwrap().split_whitespace();
        for x in output {
            match x.len() {
                2 | 3 | 4 | 7 => count = count + 1,
                _ => continue,
            }
        }
    }

    let elapsed = now.elapsed();
    println!("{}, {:?}", count, elapsed);
    std::process::exit(0)
}
