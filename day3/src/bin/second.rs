use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let max_joltage: u64 = contents
        .lines()
        .map(|line| {
            let mut index = 0;
            let mut number = String::new();
            for i in (0..12).rev() {
                let index_digit = line
                    .chars()
                    .skip(index)
                    .take(line.len() - index - i)
                    .enumerate()
                    .max_by(|(ia, a), (ib, b)| a.cmp(b).then(ib.cmp(ia)))
                    .unwrap();
                index += index_digit.0;
                let digit = index_digit.1;
                index += 1;
                number.push(digit);
            }
            number.parse::<u64>().unwrap()
        })
        .sum();
    println!("{}", max_joltage);
}
