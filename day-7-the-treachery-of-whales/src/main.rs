use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input: Vec<i32> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split(",")
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    let mut sum = 0;
    for n in input.iter() {
        sum = sum + n;
    }
    let mean = sum / input.len() as i32;

    let mut least_fuel: u32 = u32::MAX;
    for i in (mean-1)..=(mean+1) {
        let mut fuel:u32 = 0;
        for n in input.iter() {
            let distance = i32::abs(n - i);
            fuel = fuel + ((distance * (distance + 1)) / 2) as u32;
        }
        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    let elapsed = now.elapsed();
    println!("{}, {:?}", least_fuel, elapsed);
    std::process::exit(0)
}
