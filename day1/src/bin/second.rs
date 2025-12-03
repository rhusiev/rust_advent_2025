use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers = contents.lines().map(|line| {
        let mut linechars = line.chars();
        let sign: i32 = {
            if linechars.next().unwrap() == 'R' {
                1
            } else {
                -1
            }
        };
        let number: i32 = linechars.collect::<String>().parse::<i32>().unwrap();
        sign * number
    });

    let mut current: i32 = 50;

    let count: i32 = numbers
        .map(|num| {
            let single_diff = num % 100;
            let new = current + single_diff;
            let single_cross = (new <= 0 || new >= 100) && current != 0;
            println!("cur {:4} num {:4} new {:4} cross {:5} add {:4}", current, num, new, single_cross, (num - single_diff).abs().div_euclid(100));
            current = new.rem_euclid(100);
            (num - single_diff).abs().div_euclid(100) + single_cross as i32
        })
        .sum();

    println!("{}", count);
}
