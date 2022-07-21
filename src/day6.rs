use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use regex::Regex;

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day6.txt").unwrap());

    let mut grid = [false; 1_000_000];

    let pattern = Regex::new(r"([a-z]) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let cap = pattern.captures(&line).unwrap();
        let type_ = cap.index(1).chars().next().unwrap();
        let x_start = cap.index(2).parse::<usize>().unwrap();
        let x_end = cap.index(4).parse::<usize>().unwrap();
        let y_start = cap.index(3).parse::<usize>().unwrap();
        let y_end = cap.index(5).parse::<usize>().unwrap();

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                grid[x + y * 1000] = match type_ {
                    'f' => false,
                    'n' => true,
                    'e' => !grid[x + y * 1000],
                    _ => unreachable!()
                }
            }
        }
    }
    let mut count = 0;
    grid.iter().for_each(|&x| count += x as i32);
    print!("{}\n", count);
}

fn second() {
    let reader = BufReader::new(File::open("data/day6.txt").unwrap());

    let mut grid = [0; 1_000_000];

    let pattern = Regex::new(r"([a-z]) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    for line in reader.lines() {
        let line = line.unwrap();
        let cap = pattern.captures(&line).unwrap();
        let type_ = cap.index(1).chars().next().unwrap();
        let x_start = cap.index(2).parse::<usize>().unwrap();
        let x_end = cap.index(4).parse::<usize>().unwrap();
        let y_start = cap.index(3).parse::<usize>().unwrap();
        let y_end = cap.index(5).parse::<usize>().unwrap();

        for y in y_start..=y_end {
            for x in x_start..=x_end {
                grid[x + y * 1000] += match type_ {
                    'f' => if grid[x + y * 1000] != 0 { -1 } else { 0 },
                    'n' => 1,
                    'e' => 2,
                    _ => unreachable!()
                }
            }
        }
    }
    let mut count = 0;
    grid.iter().for_each(|&x| count += x as i32);
    print!("{}\n", count);
}