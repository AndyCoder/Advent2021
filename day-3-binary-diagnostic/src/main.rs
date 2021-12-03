use std::fs;

fn main() {
    let input: Vec<String> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect();

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
    
    println!("{} * {} = {}", gamma, epsilon, g * e);
    std::process::exit(0)
}
