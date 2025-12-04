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
    let mut total_count: usize = 0;

    loop {
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
        }

        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j].1 < 4 {
                    count += 1;
                    grid[i][j].0 = '.';
                }
                grid[i][j].1 = 0;
            }
        }
        if count == 0 {
            break;
        }
        total_count += count;
    }
    println!("{}", total_count);
}
