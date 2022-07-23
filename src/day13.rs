use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    let reader = BufReader::new(File::open("data/day13.txt").unwrap());

    let mut changes = HashMap::new();
    let mut people = HashSet::new();

    for line in reader.lines() {
        let string = line.unwrap();
        let line = string.strip_suffix(".").unwrap();
        let temp = line.split(" ").collect_vec();
        let first = temp[0].to_string();
        let second = temp[10].to_string();
        let change = temp[3].parse::<i32>().unwrap() * if temp[2] == "gain" { 1 } else { -1 };

        people.insert(first.to_owned());
        people.insert(second.to_owned());

        changes
            .entry(first.to_owned())
            .or_insert(HashMap::new())
            .insert(second.to_owned(), change);
    }

    // let first = people.iter().next().unwrap().to_owned();
    //
    // people.remove(&first);

    let first = "Me".to_owned();
    for second in &people {
        changes
            .entry(first.to_owned())
            .or_insert(HashMap::new())
            .insert(second.to_owned(), 0);
        changes
            .entry(second.to_owned())
            .or_insert(HashMap::new())
            .insert(first.to_owned(), 0);
    }

    let mut maximum = i32::MIN;

    people
        .iter()
        .permutations(people.len())
        .for_each(
            |x|
                {
                    let (happiness, last) = x
                        .iter()
                        .fold(
                            (0, &first),
                            |(mut acc, previous), &curr| {
                                acc += changes[previous][curr];
                                acc += changes[curr][previous];
                                (acc, curr)
                            });
                    let temp1 = changes[last][&first];
                    let temp2 = changes[&first][last];
                    let a = happiness + temp1 + temp2;
                    maximum = maximum.max(a);
                }
        )
    ;
    println!("{}", maximum);
}