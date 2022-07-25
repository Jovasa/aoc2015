use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn check(k: &str, v: i32, second: i32) -> bool {
    if k == "cats:" || k == "trees:" {
        return second > v;
    }
    if k == "pomeranians:" || k == "goldfish:" {
        return second < v;
    }
    v == second
}

fn main() {
    let reader = BufReader::new(File::open("data/day16.txt").unwrap());
    let mut correct_data = HashMap::new();
    correct_data.insert("children:", 3);
    correct_data.insert("cats:", 7);
    correct_data.insert("samoyeds:", 2);
    correct_data.insert("pomeranians:", 3);
    correct_data.insert("akitas:", 0);
    correct_data.insert("vizslas:", 0);
    correct_data.insert("goldfish:", 5);
    correct_data.insert("trees:", 3);
    correct_data.insert("cars:", 2);
    correct_data.insert("perfumes:", 1);

    for line in reader.lines() {
        let line =line.unwrap();
        let temp = line.split(" ").collect_vec();
        let first: i32 = temp[3].strip_suffix(",").unwrap().parse().unwrap();
        let second: i32 = temp[5].strip_suffix(",").unwrap().parse().unwrap();
        let third   :i32 = temp[7].parse().unwrap();
        if check(temp[2],correct_data[temp[2]], first) &&
            check(temp[4],correct_data[temp[4]], second)  &&
            check(temp[6],correct_data[temp[6]], third)  {
            println!("{}", temp[1]);
        }
    }
}