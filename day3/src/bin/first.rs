use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let max_joltage: u32 = contents.lines().map(
        |line| {
            let (first_i, first_digit) = line.chars().take(line.len() - 1).enumerate().max_by(|(ia, a), (ib, b)| {a.cmp(b).then(ib.cmp(ia))}).unwrap();
            let second_digit = line.chars().skip(first_i + 1).max().unwrap();
            format!("{}{}", first_digit, second_digit).parse::<u32>().unwrap()
        }
    ).sum();
    println!("{}", max_joltage);
}
