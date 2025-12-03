use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let numbers = contents.lines().map(|line| {
        let mut linechars = line.chars();
        let sign: i16 = { if linechars.next().unwrap() == 'R' { 1 } else { -1 } };
        let number: i16 = linechars.collect::<String>().parse::<i16>().unwrap();
        sign * number
    });

    let mut current: i16 = 50;
    
    let count: i16 = numbers.map(|num| {
        current = (current + num) % 100;
        (current == 0) as i16
    }).sum();

    println!("{}", count);
}
