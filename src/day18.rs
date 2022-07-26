use std::fs::File;
use std::io::{BufRead, BufReader};


fn get_neighbour_alive_count(grid: &[bool], x: i32, y: i32) -> u32 {
    let mut count = 0;
    for y1 in (y-1).max(0)..=(y+1).min(99) {
        for x1 in (x-1).max(0)..=(x+1).min(99) {
            if x1 == x && y1 == y {continue}
            count += grid[(x1 + y1 * 100) as usize] as u32;
        }
    }
    count
}


fn main() {
    let reader = BufReader::new(File::open("data/day18.txt").unwrap());

    let mut grid = Vec::new();

    reader
        .lines()
        .for_each(|x| x
            .unwrap()
            .chars()
            .for_each(|y| grid.push(y == '#')));

    for _ in 0..100 {
        grid[0] = true;
        grid[99] = true;
        grid[9900] = true;
        grid[9999] = true;
        let mut temp = vec![false; 100*100];
        for y in 0..100 {
            for x in 0..100 {
                let count = get_neighbour_alive_count(&grid, x, y);
                let index: usize = (x + y * 100) as usize;
                temp[index] = count == 3 || (count == 2 && grid[index]);
            }
        }
        grid = temp;
    }
    grid[0] = true;
    grid[99] = true;
    grid[9900] = true;
    grid[9999] = true;
    println!("{}", grid.iter().fold(0, |acc, &v| acc + (v as u32)));
}