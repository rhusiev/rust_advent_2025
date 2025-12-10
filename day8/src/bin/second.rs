use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
struct DistanceBetween {
    point_i: usize,
    point_j: usize,
    distance_sq: i64,
}
impl Eq for DistanceBetween {}
impl PartialOrd for DistanceBetween {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for DistanceBetween {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance_sq
            .partial_cmp(&self.distance_sq)
            .unwrap_or(Ordering::Equal)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let nodes_coords = contents
        .lines()
        .map(|line| {
            let mut coords = line
                .split(',')
                .map(|num_str| num_str.parse::<i64>().unwrap());
            let (x, y, z) = (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            );
            (x, y, z)
        })
        .collect::<Vec<(i64, i64, i64)>>();
    let mut min_heap = BinaryHeap::new();
    nodes_coords
        .iter()
        .enumerate()
        .for_each(|(i, (x1, y1, z1))| {
            nodes_coords
                .iter()
                .enumerate()
                .skip(i + 1)
                .for_each(|(j, (x2, y2, z2))| {
                    min_heap.push(DistanceBetween {
                        point_i: i,
                        point_j: j,
                        distance_sq: (x1 - x2).pow(2) + (y1 - y2).pow(2) + (z1 - z2).pow(2),
                    });
                });
        });

    let mut clusters = vec![1; nodes_coords.len()];
    for i in 0..clusters.len() {
        clusters[i] = i;
    }
    let mut num_clusters = nodes_coords.len();
    let mut last_prod = 0;
    while num_clusters > 1 {
        let pair = min_heap.pop().unwrap();
        let cluster_i = clusters[pair.point_i];
        let cluster_j = clusters[pair.point_j];
        if cluster_i == cluster_j {
            continue;
        }
        num_clusters -= 1;
        last_prod = nodes_coords[pair.point_i].0 * nodes_coords[pair.point_j].0;
        for j in 0..clusters.len() {
            if clusters[j] == cluster_j {
                clusters[j] = cluster_i;
            }
        }
    }
    println!("{}", last_prod);
}
