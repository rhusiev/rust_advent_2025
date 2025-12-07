use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines().map(|string| {string.chars().map(|c| {if c == 'S' {1} else if c == '^' {-1} else {0}}).collect::<Vec<i64>>()}).collect::<Vec<Vec<i64>>>();
    for i in 0..lines.len()-1 {
        for j in 0..lines[i].len() {
            let current = lines[i][j];
            if current > 0 {
                if lines[i + 1][j] == -1 {
                    lines[i + 1][j + 1] += current;
                    lines[i + 1][j - 1] += current;
                } else {
                    lines[i + 1][j] += current;
                }
            }
        }
    }
    println!("{}", lines.last().unwrap().iter().sum::<i64>());
}
