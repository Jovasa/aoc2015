use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let reader = BufReader::new(File::open("data/day5.txt").unwrap());

    let mut nice_strings = 0;

    for line in reader.lines() {
        let mut vowels = 0;
        let mut previous = '_';
        let mut got_double = false;
        let mut got_invalid = false;
        for c in line.unwrap().chars() {
            match c {
                'a' | 'e' | 'i' | 'u' | 'o' =>  vowels += 1,
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