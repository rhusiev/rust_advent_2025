use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let ranges = contents.split(',').map(|range| {
        let mut range = range.split('-');
        let start = range.next().unwrap().parse::<u64>().unwrap();
        let end = range.next().unwrap();
        let end = end
            .strip_suffix('\n')
            .unwrap_or(end)
            .parse::<u64>()
            .unwrap();
        (start, end)
    });
    let mut invalid_sum: u64 = 0;
    for (start, end) in ranges {
        for num in start..=end {
            let num_string = num.to_string();
            let len = num_string.len();
            for i in 1..=len / 2 {
                if !(len % i == 0) {
                    continue;
                }
                let mut num_it = num_string.chars();
                let pattern: String = num_it.by_ref().take(i).collect();
                let mut i_matches = true;
                loop {
                    let next = num_it.by_ref().take(i).collect::<String>();
                    if next.len() == 0 {
                        break;
                    }
                    if next != pattern {
                        i_matches = false;
                    }
                }
                if i_matches {
                    invalid_sum += num;
                    break;
                }
            }
        }
    }

    println!("{}", invalid_sum);
}
