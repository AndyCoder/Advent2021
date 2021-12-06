use std::fs;
use std::collections::HashMap;

fn main() {
    let input: Vec<u8> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split(",")
        .map(|s| u8::from_str_radix(s, 10).unwrap())
        .collect();

    let mut counts: HashMap<u8, u64> = HashMap::new();

    for n in input.iter() {
        let count = counts.entry(*n).or_insert(0);
        *count += 1;
    }

    let final_state = tick(&counts, 256);

    let mut sum = 0;
    for x in final_state.values() {
        sum = sum + x;
    }
    println!("{}", sum);
    std::process::exit(0)
}

fn tick(initial_state: &HashMap<u8, u64>, days: u32) -> HashMap<u8, u64> {
    let mut next_state: HashMap<u8, u64> = initial_state.clone();
    for _ in 1..=days {
        let x0 = next_state.get(&0).or(Some(&0)).unwrap().to_owned();
        let x1 = next_state.get(&1).or(Some(&0)).unwrap().to_owned();
        let x2 = next_state.get(&2).or(Some(&0)).unwrap().to_owned();
        let x3 = next_state.get(&3).or(Some(&0)).unwrap().to_owned();
        let x4 = next_state.get(&4).or(Some(&0)).unwrap().to_owned();
        let x5 = next_state.get(&5).or(Some(&0)).unwrap().to_owned();
        let x6 = next_state.get(&6).or(Some(&0)).unwrap().to_owned();
        let x7 = next_state.get(&7).or(Some(&0)).unwrap().to_owned();
        let x8 = next_state.get(&8).or(Some(&0)).unwrap().to_owned();
        next_state.insert(8, x0);
        next_state.insert(7, x8);
        next_state.insert(6, x0 + x7);
        next_state.insert(5, x6);
        next_state.insert(4, x5);
        next_state.insert(3, x4);
        next_state.insert(2, x3);
        next_state.insert(1, x2);
        next_state.insert(0, x1);
    }
    next_state
}