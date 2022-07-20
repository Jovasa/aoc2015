use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());

    let mut total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut minimum = i32::MAX;
        line
            .split("x")
            .map(|x| x.parse().unwrap())
            .combinations(2)
            .for_each(|x: Vec<i32>| {
                let i = 2 * x[0] * x[1];
                total += i;
                minimum = minimum.min(i / 2);
            });
        total += minimum;
    }
    println!("{}", total);
}