use std::env;
use std::fs;

fn is_inside_shape(x1: f64, y1: f64, x2: f64, y2: f64, coords: &Vec<(f64, f64)>) -> bool {
    let x_nw = x1.min(x2) + 0.5;
    let y_nw = y1.min(y2) + 0.5;
    let x_ne = x1.max(x2) - 0.5;
    let y_ne = y1.min(y2) + 0.5;
    let x_se = x1.max(x2) - 0.5;
    let y_se = y1.max(y2) - 0.5;
    let x_sw = x1.min(x2) + 0.5;
    let y_sw = y1.max(y2) - 0.5;

    let (mut nw, mut ne, mut se, mut sw) = (0, 0, 0, 0);

    let mut i = 0;
    let start_from_second = coords[0].0 == coords[1].0;
    if start_from_second {
        i += 1;
    }

    while i < coords.len() - 1 {
        if coords[i].1 < y_nw
            && (coords[i].0 > x_nw && coords[i + 1].0 < x_nw
                || coords[i].0 < x_nw && coords[i + 1].0 > x_nw)
        {
            nw += 1;
        }
        if coords[i].1 < y_ne
            && (coords[i].0 > x_ne && coords[i + 1].0 < x_ne
                || coords[i].0 < x_ne && coords[i + 1].0 > x_ne)
        {
            ne += 1;
        }
        if coords[i].1 > y_se
            && (coords[i].0 > x_se && coords[i + 1].0 < x_se
                || coords[i].0 < x_se && coords[i + 1].0 > x_se)
        {
            se += 1;
        }
        if coords[i].1 > y_sw
            && (coords[i].0 > x_sw && coords[i + 1].0 < x_sw
                || coords[i].0 < x_sw && coords[i + 1].0 > x_sw)
        {
            sw += 1;
        }
        if coords[i].1 > y_nw
            && coords[i].1 < y_sw
            && (coords[i].0 < x_nw && coords[i + 1].0 > x_nw
                || coords[i].0 > x_nw && coords[i + 1].0 < x_nw
                || coords[i].0 < x_ne && coords[i + 1].0 > x_ne
                || coords[i].0 > x_ne && coords[i + 1].0 < x_ne)
        {
            return false;
        }
        if i > 0 {
            if coords[i - 1].0 > x_nw
                && coords[i - 1].0 < x_ne
                && (coords[i - 1].1 < y_nw && coords[i].1 > y_nw
                    || coords[i - 1].1 > y_nw && coords[i].1 < y_nw
                    || coords[i - 1].1 < y_sw && coords[i].1 > y_sw
                    || coords[i - 1].1 > y_sw && coords[i].1 < y_sw)
            {
                return false;
            }
        }
        i += 2;
    }

    let last = coords.len() - 1;
    if start_from_second {
        if coords[last].1 < y_ne
            && (coords[last].0 > x_nw && coords[0].0 < x_nw
                || coords[last].0 < x_nw && coords[0].0 > x_nw)
        {
            nw += 1;
        }
        if coords[last].1 < y_ne
            && (coords[last].0 > x_ne && coords[0].0 < x_ne
                || coords[last].0 < x_ne && coords[0].0 > x_ne)
        {
            ne += 1;
        }
        if coords[last].1 > y_se
            && (coords[last].0 > x_se && coords[0].0 < x_se
                || coords[last].0 < x_se && coords[0].0 > x_se)
        {
            se += 1;
        }
        if coords[last].1 > y_sw
            && (coords[last].0 > x_sw && coords[0].0 < x_sw
                || coords[last].0 < x_sw && coords[0].0 > x_sw)
        {
            sw += 1;
        }
        if coords[0].1 > y_nw
            && coords[0].1 < y_sw
            && (coords[0].0 < x_nw && coords[last].0 > x_nw
                || coords[0].0 > x_nw && coords[last].0 < x_nw
                || coords[0].0 < x_ne && coords[last].0 > x_ne
                || coords[0].0 > x_ne && coords[last].0 < x_ne)
        {
            return false;
        }
    } else {
        if coords[last].0 > x_nw
            && coords[last].0 < x_ne
            && (coords[last].1 < y_nw && coords[0].1 > y_nw
                || coords[last].1 > y_nw && coords[0].1 < y_nw
                || coords[last].1 < y_sw && coords[0].1 > y_sw
                || coords[last].1 > y_sw && coords[0].1 < y_sw)
        {
            return false;
        }
    }
    return (sw % 2 != 0) && (se % 2 != 0) && (ne % 2 != 0) && (nw % 2 != 0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let coords = contents
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<f64>().unwrap();
            let y = y.parse::<f64>().unwrap();
            (x, y)
        })
        .collect::<Vec<(f64, f64)>>();
    let largest_area = coords
        .iter()
        .map(|(x1, y1)| {
            coords
                .iter()
                .map(|(x2, y2)| {
                    let area = ((x1 - x2).abs() + 1.0) * ((y1 - y2).abs() + 1.0);
                    if is_inside_shape(*x1, *y1, *x2, *y2, &coords) {
                        area as i64
                    } else {
                        0
                    }
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{}", largest_area);
}
