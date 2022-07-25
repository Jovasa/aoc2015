use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;


fn count_things(ingredients: &Vec<Vec<i32>>, left: i32, index: usize) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    if index == ingredients.len() - 1 {
        out.push(ingredients[index].iter().map(|x| x * left).collect_vec());
    }
    else {
        for i in 0..=left {
            let t = count_things(ingredients, left - i, index + 1);
            for thing in t {
                out.push(
                    thing
                        .iter()
                        .zip(ingredients[index].iter())
                        .map(|(x, y)| x + y * i)
                        .collect()
                );
            }
        }

    }
    out
}


fn main() {
    let reader = BufReader::new(File::open("data/day15.txt").unwrap());

    let stuff = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            x
                .split(" ")
                .filter(|&p| match p.parse::<i32>() {
                    Ok(_) => true,
                    Err(_) => {
                        p.ends_with(",")
                    }
                })
                .map(|p| p.split(",").next().unwrap().parse::<i32>().unwrap()).collect_vec()
        }).collect_vec();

    let mut best = 0;
    let mut five_hundred_calories = 0;

    for t in count_things(&stuff, 100, 0) {
        let temp = t.iter().take(4).fold(1, |acc, new| acc.max(0) * new.max(&0));
        best = best.max(
            temp
        );
        if t[4] == 500 {
            five_hundred_calories = five_hundred_calories.max(temp);
        }
    }
    println!("{} {}", best, five_hundred_calories);
}