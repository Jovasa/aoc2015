use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day2.txt").unwrap());

    let mut paper_area = 0;
    let mut ribbon_length = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut minimum = i32::MAX;
        let dimensions: Vec<i32> = line
            .split("x")
            .map(|x| x.parse().unwrap()).sorted().collect();
        dimensions
            .iter()
            .combinations(2)
            .for_each(|x: Vec<&i32>| {
                let i = 2 * x[0] * x[1];
                paper_area += i;
                minimum = minimum.min(i / 2);
            });
        paper_area += minimum;

        ribbon_length += dimensions[0] * 2 + dimensions[1] * 2;
        ribbon_length += dimensions.iter().fold(1, |acc, &x| acc * x);
    }
    println!("{}", paper_area);
    println!("{}", ribbon_length);
}