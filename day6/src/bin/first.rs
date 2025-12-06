use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let lines = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|string| string.to_string())
        .collect::<Vec<String>>();
    let mut numbers: Vec<Vec<&str>> = vec![vec![""; 0]; lines.len() - 1];

    let mut parsed_chars = 0;
    while parsed_chars < lines[0].len() {
        let mut max_i = 0;
        for line in lines.iter() {
            let mut i = parsed_chars;
            let mut line_it = line.chars().skip(parsed_chars);
            loop {
                let opt_c = line_it.next();
                match opt_c {
                    Some(' ') => {
                        break;
                    }
                    Some(_c) => {
                        i += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
            max_i = max_i.max(i);
        }

        for (i, line) in lines.iter().enumerate() {
            if i == lines.len() - 1 {
                break;
            }
            let num_line = &mut numbers[i];
            num_line.push(&line[parsed_chars..max_i]);
        }
        parsed_chars = max_i + 1;
    }

    let mut operations = vec!['+'; numbers[0].len()];
    let mut i = 0;
    for c in lines[lines.len() - 1].chars() {
        if c != ' ' {
            operations[i] = c;
            i += 1;
        }
    }

    let mut total = 0;
    for j in 0..operations.len() {
        let op = operations[j];
        let mut result = 0;
        if op == '*' {
            result = 1;
        }
        for i in 0..numbers.len() {
            let num = numbers[i][j]
                .trim_start()
                .trim_end()
                .parse::<i64>()
                .unwrap();
            if op == '+' {
                result += num;
            } else {
                result *= num;
            }
        }
        total += result;
    }

    println!("{}", total);
}
