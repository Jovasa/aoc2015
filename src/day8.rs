use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day8.txt").unwrap());

    let escape_patterns = vec!["/", "\\\"", "\\x"];
    let reduce = vec![1, 1, 3];
    let increase = vec![2, 2, 1];

    let mut total_score = 0;
    let mut total_score2 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        // Replace escaped backslash with forward slash to deal with \\x
        let temp = line.replace("\\\\", "/");
        let t = escape_patterns
            .iter()
            .map(|x| temp.matches(x).count())
            .zip(reduce.iter())
            .fold(0, |a,(b, c) | a + b * c);

        let c = escape_patterns
            .iter()
            .map(|x| temp.matches(x).count())
            .zip(increase.iter())
            .fold(0, |a,(b, c) | a + b * c);

        total_score +=  t + 2;
        total_score2 +=  c + 4;
    }

    print!("{} {}\n", total_score, total_score2)
}