use std::fs;

fn main() {
    let input: Vec<u32> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| u32::from_str_radix(s, 10).expect("non-integer input"))
        .collect();

    let mut last: Option<u32> = Option::None;
    let mut count: u32 = 0;
    for i in input.windows(3) {
        match last {
            None => last = Some(i.to_owned()),
            Some(j) => {
                if i > &j {
                    count = count + 1
                };
                last = Some(i.to_owned())
            }
        }
    };
    println!("{}", count);
    std::process::exit(0)
}
