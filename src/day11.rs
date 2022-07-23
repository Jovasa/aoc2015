use std::collections::HashSet;
use itertools::Itertools;

fn is_valid_password(pw: &[char]) -> bool {
    let mut second_rule = false;
    let mut pairs = HashSet::new();
    let mut buffer = ('_', '_');
    let mut diffs = Vec::new();
    let mut previous_pair = -2;
    pw.windows(2).enumerate().for_each(
        |(i, x)| {
            let first = x[0];
            let second = x[1];
            if first != '_' && second != '_' {
                diffs.push(first as i32 - second as i32);
            }
            let t = (first, second);
            if first == second {
                if previous_pair + 1 != i as i32 {
                    pairs.insert(buffer);
                    previous_pair = i as i32;
                }
            }
            buffer = t;
        }
    );
    diffs.iter().fold(0, |prev, &curr| {
        second_rule |= prev == -1 && curr == -1;
        curr
    });
    pairs.len() >= 2 && second_rule
}


fn main() {
    let mut pwd = "vzbxkghb".chars().collect_vec();
    increment_password(&mut pwd);
    println!("{}", pwd.iter().collect::<String>());
    increment_password(&mut pwd);
    println!("{}", pwd.iter().collect::<String>());
}

fn increment_password(pwd: &mut Vec<char>) {
    loop {
        for i in (0..pwd.len()).rev() {
            if pwd[i] == 'z' {
                pwd[i] = 'a';
            } else {
                if pwd[i] == 'i' || pwd[i] == 'l' || pwd[i] == 'o' {
                    pwd[i] = ((pwd[i] as u8) + 2) as char;
                } else {
                    pwd[i] = ((pwd[i] as u8) + 1) as char;
                }
                break;
            }
        }
        if is_valid_password(pwd) {
            return;
        }
    }
}