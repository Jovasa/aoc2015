use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn main() {
    let reader = BufReader::new(File::open("data/day1.txt").unwrap());

    let mut floor = 0;
    let temp = '(' as u8;
    reader.bytes().enumerate().for_each(|(i, x)| {
        if x.unwrap() == temp { floor += 1 } else { floor -= 1 }
        if floor == -1 { println!("{}", i + 1) }
    });
    println!("{}", floor);
}