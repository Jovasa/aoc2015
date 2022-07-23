use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use regex::Regex;

fn main() {
    let reader = BufReader::new(File::open("data/day14.txt").unwrap());

    let pattern = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();

    let mut reindeer = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let cap = pattern.captures(&line).unwrap();
        let name = cap.index(1).to_owned();
        let speed = cap.index(2).parse::<i32>().unwrap();
        let active = cap.index(3).parse::<i32>().unwrap();
        let rest = cap.index(4).parse::<i32>().unwrap();
        reindeer.push((name, speed, active, rest, 0, 0));
    }

    let goal = 2503;

    for (_, s, a, r, _, _ ) in &reindeer {
        let mut time = 0;
        let mut finish = 0;
        while time < goal {
            finish += a.min(&(goal - time)) * s;
            time += a + r;
        }
        println!("{}", finish);
    }

    for il in 0..2503 {
        let mut maximum = i32::MIN;
        reindeer.iter_mut().enumerate().for_each(|(x, i)| {
            let (n, s, a, r, p, sc) = i;
            let period = (*a + *r) as usize;
            if (il % period) < *a as usize {
                i.4 += *s;
            }
            maximum = maximum.max(i.4);
        });
        reindeer.iter_mut().for_each(|i|  if i.4 == maximum { i.5 += 1 } )
    }
    for i in reindeer {
        println!("{}", i.5);

    }
}