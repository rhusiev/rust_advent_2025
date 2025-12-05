use std::env;
use std::fs;
use std::ops::RangeInclusive;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let (fresh_ranges_lines, _available_lines) = contents.split_once("\n\n").unwrap();

    let mut fresh_ranges = fresh_ranges_lines
        .lines()
        .map(|line| {
            let (begin, end) = line.split_once('-').unwrap();
            let begin = begin.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();
            begin..=end
        })
        .collect::<Vec<RangeInclusive<u64>>>();

    fresh_ranges.sort_by(|range1, range2| {
        range1.start().cmp(range2.start())
    });

    let mut i = 0;
    while i < fresh_ranges.len() {
        let mut end = *fresh_ranges[i].end();
        let j = i + 1;
        while j < fresh_ranges.len() {
            if *fresh_ranges[j].start() <= end {
                let next_end = *fresh_ranges[j].end();
                end = end.max(next_end);
                fresh_ranges.remove(j);
            } else {
                break;
            }
        }
        fresh_ranges[i] = *fresh_ranges[i].start()..=end;
        i += 1;
    }

    let fresh_count: u64 = fresh_ranges.iter().map(|range| {
        range.end() - range.start() + 1
    }).sum();

    println!("{:?}", fresh_count);
}
