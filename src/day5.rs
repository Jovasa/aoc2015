use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    first();
    second();
}

fn first() {
    let reader = BufReader::new(File::open("data/day5.txt").unwrap());

    let mut nice_strings = 0;

    for line in reader.lines() {
        let mut vowels = 0;
        let mut previous = '_';
        let mut got_double = false;
        let mut got_invalid = false;
        for c in line.unwrap().chars() {
            match c {
                'a' | 'e' | 'i' | 'u' | 'o' => vowels += 1,
                _ => {}
            }
            got_double |= previous == c;
            match previous {
                'a' => got_invalid |= c == 'b',
                'c' => got_invalid |= c == 'd',
                'p' => got_invalid |= c == 'q',
                'x' => got_invalid |= c == 'y',
                _ => {}
            }
            previous = c;
        }
        if vowels >= 3 && got_double && !got_invalid {
            nice_strings += 1;
        }
    }
    println!("{}", nice_strings);
}

fn second() {
    let reader = BufReader::new(File::open("data/day5.txt").unwrap());

    let mut nice_strings = 0;

    for line in reader.lines() {
        let mut first_rule = false;
        let mut second_rule = false;
        let mut pairs = HashSet::new();
        let string = line.unwrap();
        let vec = string.split("").collect_vec();
        vec.clone().windows(3).for_each(
            |x| second_rule |= x[0] == x[2]
        );
        let mut buffer = ('_', '_');
        vec.windows(2).for_each(
            |x| {
                let first = match x[0].chars().next() {
                    Some(a) => a,
                    None => '_'
                };
                let second = match x[1].chars().next(){
                    Some(a) => a,
                    None => '_'
                };
                let t = (first, second);
                first_rule |= pairs.contains(&t);
                pairs.insert(buffer);
                buffer = t;
            }
        );
        if first_rule && second_rule {nice_strings += 1;}
    }
    println!("{}", nice_strings);
}