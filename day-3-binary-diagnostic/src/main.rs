use std::fs;

fn main() {
    let mut input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect();

    input.sort();

    let width: usize = input[1].chars().count();
    let length: usize = input.len();
    let mut sums: Vec<usize> = vec![0; width];
    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();

    for line in input.iter() {
        let mut sums_copy: Vec<usize> = vec![0; width];
        sums_copy.copy_from_slice(&sums);
        for (i, n) in sums_copy.iter().enumerate() {
            sums[i] = n + usize::from_str_radix(&line.chars().nth(i).unwrap().to_string(), 2).unwrap()
        }
    }

    for x in sums.iter() {
        if x > &(length / 2) {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let g: usize = usize::from_str_radix(&gamma, 2).unwrap();
    let e: usize = usize::from_str_radix(&epsilon, 2).unwrap();
    
    println!("part 1: {} * {} = {}", gamma, epsilon, g * e);

    let mut oxy_seq: String = gamma.chars().nth(0).unwrap().to_string();
    let oxy_gen_rating: String;
    loop {
        let filtered_input: Vec<&String> = input.iter().filter(|x| match x.find(&oxy_seq) {
            Some(0) => true,
            _ => false,
        }).collect();
        let len: usize = filtered_input.len();
        if len == 1 {
            oxy_gen_rating = filtered_input.get(0).unwrap().to_string();
            break;
        } else {
            if sum_at_pos(&filtered_input, oxy_seq.len()) > (len-1) / 2 {
                oxy_seq.push('1')
            } else {
                oxy_seq.push('0')
            }
        }
    };

    let mut co2_seq: String = epsilon.chars().nth(0).unwrap().to_string();
    let co2_scrub_rating: String;
    loop {
        let filtered_input: Vec<&String> = input.iter().filter(|x| match x.find(&co2_seq) {
            Some(0) => true,
            _ => false,
        }).collect();
        let len: usize = filtered_input.len();
        if len == 1 {
            co2_scrub_rating = filtered_input.get(0).unwrap().to_string();
            break;
        } else {
            if sum_at_pos(&filtered_input, co2_seq.len()) > (len-1) / 2 {
                co2_seq.push('0')
            } else {
                co2_seq.push('1')
            }
        }
    };

    let o: usize = usize::from_str_radix(&oxy_gen_rating, 2).unwrap();
    let c: usize = usize::from_str_radix(&co2_scrub_rating, 2).unwrap();

    println!("part 2: {} * {} = {}", oxy_gen_rating, co2_scrub_rating, o * c);

    std::process::exit(0)
}

fn sum_at_pos(input: &Vec<&String>, pos: usize) -> usize {
    let mut sum: usize = 0;
    for line in input.iter() {
        sum = sum + usize::from_str_radix(&line.chars().nth(pos).unwrap().to_string(), 2).unwrap()
    };
    sum
}