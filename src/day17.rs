use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn parser(buckets: &[u32], start: usize, used: u32, depth: u32) -> u32{
    let mut valids = 0;
    for i in start..buckets.len() {
        let temp = used + buckets[i];
        if temp < 150 {
            valids += parser(buckets, i + 1, temp, depth + 1);
        }
        else if temp == 150 && depth == 3 {
            valids += 1;
        }
    }
    valids
}

fn main() {
    let reader = BufReader::new(File::open("data/day17.txt").unwrap());

    let buckets = reader
        .lines()
        .map(|x| x.unwrap().parse::<u32>().unwrap())
        .collect_vec();

    println!("{}", parser(&buckets, 0, 0, 0));
}