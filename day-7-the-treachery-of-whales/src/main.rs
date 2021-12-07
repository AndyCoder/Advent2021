use std::fs;

fn main() {
    let input: Vec<i32> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split(",")
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    let mut min = input[0];
    let mut max = input[0];
    for n in input.iter() {
        if n < &min {
            min = *n;
        } else if n > &max {
            max = *n;
        }
    }

    let mut least_fuel: u32 = u32::MAX;
    for i in min..=max {
        let mut fuel:u32 = 0;
        for n in input.iter() {
            fuel = fuel + (i32::abs(n - i) as u32);
        }
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    println!("{}", least_fuel);
    std::process::exit(0)
}
