use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("input")
        .expect("unable to read input file");

    let re = Regex::new(r"\b(\d+),(\d+) -> (\d+),(\d+)\b").unwrap();

    let mut vents: HashMap<(i32, i32), i32> = HashMap::new();

    for cap in re.captures_iter(&input) {
        let x1 = i32::from_str_radix(&cap[1], 10).unwrap();
        let y1 = i32::from_str_radix(&cap[2], 10).unwrap();
        let x2 = i32::from_str_radix(&cap[3], 10).unwrap();
        let y2 = i32::from_str_radix(&cap[4], 10).unwrap();

        if x1 == x2 {
            let start = match y1 < y2 {
                true => y1,
                false => y2
            };
            let end = i32::abs(y2 - y1) + start;
            for i in start..=end {
                let count = vents.entry((x1, i)).or_insert(0);
                *count += 1;
            }
        } else if y1 == y2 {
            let start = match x1 < x2 {
                true => x1,
                false => x2
            };
            let end = i32::abs(x2 - x1) + start;
            for i in start..=end {
                let count = vents.entry((i, y1)).or_insert(0);
                *count += 1;
            }
        } 
    }

    let mut sum = 0;
    for x in vents.values() {
        if *x > 1 {
            sum = sum + 1;
        }
    }

    println!("{}", sum);
    std::process::exit(0)
}
