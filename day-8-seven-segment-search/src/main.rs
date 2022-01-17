use std::fs;
use std::time::Instant;
use std::collections::HashMap;
use std::convert::TryFrom;

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
    println!("part 1 = {}, {:?}", count, elapsed);

    let mut outputs: Vec<usize> = Vec::new();
    for line in lines.iter() {
        let signals: Vec<String> = line.split("|").nth(0).unwrap()
            .split_whitespace().map(|s| s.to_string()).collect();
        let output: Vec<String> = line.split("|").nth(1).unwrap()
            .split_whitespace().map(|s| s.to_string()).collect();
        let mut digits: HashMap<usize, String> = HashMap::new();
        for x in signals.iter() {
            match x.len() {
                2 => digits.insert(1, x.to_string()),
                3 => digits.insert(7, x.to_string()),
                4 => digits.insert(4, x.to_string()),
                7 => digits.insert(8, x.to_string()),
                _ => continue,
            };
        }
        for x in signals.iter() {
            match x.len() {
                5 => { // 2, 3, 5
                    if contains(&x, digits.get(&1).unwrap()) == 2 {
                        digits.insert(3, x.to_string())
                    } else if contains(&x, digits.get(&4).unwrap()) == 3 {
                        digits.insert(5, x.to_string())
                    } else {
                        digits.insert(2, x.to_string())
                    };
                },
                6 => { // 0, 6, 9
                    if contains(&x, digits.get(&4).unwrap()) == 4 {
                        digits.insert(9, x.to_string())
                    } else if contains(&x, digits.get(&1).unwrap()) == 2 {
                        digits.insert(0, x.to_string())
                    } else {
                        digits.insert(6, x.to_string())
                    };
                },
                _ => continue,
            };
        }
        let mut out: usize = 0;
        for (i, x) in output.iter().enumerate() {
            for d in digits.keys() {
                let s = digits.get(d).unwrap();
                if x.len() == s.len() && contains(x, s) == x.len() {
                    let power: u32 = 3 - u32::try_from(i).unwrap();
                    out = out + d * usize::pow(10, power);
                }
            }
        }
        outputs.push(out);
    }

    let sum: usize = outputs.iter().sum();
    let elapsed = now.elapsed();
    println!("part 2 = {}, {:?}", sum, elapsed);
    std::process::exit(0)
}

fn contains(left: &str, right: &str) -> usize {
    let mut output: usize = 0;
    for c in left.chars() {
        if right.contains(c) {
            output = output + 1;
        }
    }
    output
}