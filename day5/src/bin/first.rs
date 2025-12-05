use std::env;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (fresh_ranges_lines, available_lines) = contents.split_once("\n\n").unwrap();

    let fresh_ranges = fresh_ranges_lines
        .lines()
        .map(|line| {
            let (begin, end) = line.split_once('-').unwrap();
            let begin = begin.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            begin..=end
        })
        .collect::<Vec<RangeInclusive<u64>>>();
    let available = available_lines
        .lines()
        .map(|line| line.parse::<u64>().unwrap());

    let available_fresh: u64 = available
        .map(|id| fresh_ranges.iter().any(|range| range.contains(&id)) as u64)
        .sum();

    println!("{}", available_fresh);
}
