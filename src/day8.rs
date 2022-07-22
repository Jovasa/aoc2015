use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day8.txt").unwrap());

    let escape_patterns = vec!["\\\\", "\\\"", "\\x"];
    let reduce = vec![1, 1, 3];

    let mut total_score = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let t = escape_patterns
            .iter()
            .map(|x| line.matches(x).count())
            .zip(reduce.iter())
            .fold(0, |a,(b, c) | a + b * c);
        println!("{} {} {}", t + 2, total_score, line);
        total_score += t + 2;
    }

    print!("{}\n", total_score)
}