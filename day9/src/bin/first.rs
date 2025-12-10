use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let coords = contents.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();
        (x, y)
    }).collect::<Vec<(i64, i64)>>();
    let largest_area = coords.iter().map(|(x1, y1)| {
        coords.iter().map(|(x2, y2)| {
            let area = ((x1 - x2 + 1) * (y1 - y2 + 1)).abs();
            area
        }).max().unwrap()
    }).max().unwrap();

    println!("{}", largest_area);
}
