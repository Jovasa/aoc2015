use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day24.txt").unwrap());

    let mut total = 0;
    let values = reader.lines().map(|x| {
        let x = x.unwrap();
        let t = x.parse::<i64>().unwrap();
        total += t;
        t
    }).collect_vec();
    total /= 4;
    for i in 4..values.len() {
        let mut min_qt = i64::MAX;
        values.iter().permutations(i).for_each(|x| {
            let t: i64 = x.iter().fold(0, |acc, &r| acc+r );
            if total == t {
                min_qt = min_qt.min(x.iter().fold(1, |acc, &r| acc * r));
                println!("{} {}", min_qt, i);
            }
        });
        if min_qt != i64::MAX {
            println!("{}", min_qt);
            break;
        }
    }
}