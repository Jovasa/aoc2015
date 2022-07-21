use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let reader = BufReader::new(File::open("data/day3.txt").unwrap());

    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut santa_pos = (0, 0);
    let mut robo_santa_pos = (0, 0);

    reader.lines().next().unwrap().unwrap().chars().enumerate().for_each(|(i, x)|
        {
            match x {
                '>' => if i % 2 == 0 { santa_pos.0 += 1; } else { robo_santa_pos.0 += 1 },
                '<' => if i % 2 == 0 { santa_pos.0 -= 1; } else { robo_santa_pos.0 -= 1 },
                '^' => if i % 2 == 0 { santa_pos.1 += 1; } else { robo_santa_pos.1 += 1 },
                'v' => if i % 2 == 0 { santa_pos.1 -= 1; } else { robo_santa_pos.1 -= 1 },
                _ => unreachable!(),
            }
            visited.insert(santa_pos);
            visited.insert(robo_santa_pos);
        }
    );

    println!("{}", visited.len());
}