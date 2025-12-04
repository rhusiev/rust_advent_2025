use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut grid = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|chr| (chr, 0))
                .collect::<Vec<(char, usize)>>()
        })
        .collect::<Vec<Vec<(char, usize)>>>();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for cell in [
                (i, j + 1),
                (i + 1, j),
                (i + 1, j + 1),
                (i + 1, if j == 0 { usize::MAX } else { j - 1 }),
            ] {
                if cell.0 >= grid.len() || cell.1 >= grid[0].len() || cell.1 == usize::MAX {
                    continue;
                }
                if grid[cell.0][cell.1].0 == '@' {
                    grid[i][j].1 += 1;
                }
                if grid[i][j].0 == '@' {
                    grid[cell.0][cell.1].1 += 1;
                }
            }
            if grid[i][j].0 != '@' {
                grid[i][j].1 = 9;
            }
        }
        // println!(
        //     "{}",
        //     grid[i]
        //         .iter()
        //         .flat_map(|(_, num)| { num.to_string().chars().collect::<Vec<char>>() })
        //         .collect::<String>()
        // );
    }
    let count: usize = grid
        .iter()
        .flat_map(|row| row.iter().map(|cell| (cell.1 < 4) as usize))
        .sum();
    println!("{}", count);
}
