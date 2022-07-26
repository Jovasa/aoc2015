fn get_present_count(i: u64) -> u64 {
    (2..=i).filter(|x| i % x == 0).fold(10, |acc, n| acc + n * 10)
}

fn get_present_count2(i: u64) -> u64 {
    ((i / 50)..=i).filter(|x| i % x == 0).fold(0, |acc, n| acc + n * 11)
}


fn main() {
    for i in 700_000.. {
        if get_present_count2(i) >= 34000000 {
            println!("{}", i);
            break;
        }
        if i % 10000 == 0 {
            println!("Still going {}", i);
        }
    }
}