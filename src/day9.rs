use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day9.txt").unwrap());

    let mut towns = Vec::new();
    let mut distances = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let temp = line.split(" ").collect_vec();
        let first = temp[0].to_string();
        let second = temp[2].to_string();
        let dist = temp[4].parse::<u32>().unwrap();

        if !towns.contains(&first) {
            towns.push(first.to_string());
        }
        if !towns.contains(&second) {
            towns.push(second.to_string());
        }
        distances
            .entry(first.to_owned())
            .or_insert(HashMap::new())
            .insert(second.to_owned(), dist);
        distances.entry(second).or_insert(HashMap::new()).insert(first, dist);
    }

    let mut minimum = u32::MAX;
    let mut maximum = u32::MIN;
    towns
        .iter()
        // Technically this method causes magnitude of towns.len() too many
        // iterations since (0, 1, 2), (2,0,1) and (1, 2, 0) are all the same route,
        // however there are so we in this exercise that it does not matter
        .permutations(towns.len())
        .map(
            |x|
                x.iter().fold(
                    (0, ""),
                    |cur, &next|
                        {
                            let (distance_so_far, current) = cur;
                            if current == "" {
                                (0, next)
                            } else {
                                let d = &distances[current][next];
                                (distance_so_far + d, next)
                            }
                        },
                )
        )
        // It should be possible to updates these inside the map by changing it to
        // for_each but there is no significant reason to optimize it
        .for_each(
        |x| {
            minimum = minimum.min(x.0);
            maximum = maximum.max(x.0);
        }
    );
    println!("{} {}", minimum, maximum);
}