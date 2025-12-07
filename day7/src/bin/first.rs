use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut lines = contents.lines().map(|string| {string.chars().collect::<Vec<char>>()}).collect::<Vec<Vec<char>>>();
    let start = contents.find('S').unwrap();
    lines[0][start] = '|';
    let mut count = 0;
    for i in 0..lines.len()-1 {
        for j in 0..lines[i].len() {
            if lines[i][j] == '|' {
                if lines[i + 1][j] == '^' {
                    lines[i + 1][j + 1] = '|';
                    lines[i + 1][j - 1] = '|';
                    count += 1;
                } else {
                    lines[i + 1][j] = '|';
                }
            }
        }
    }
    println!("{}", count);
}
