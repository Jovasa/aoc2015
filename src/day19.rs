use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;


fn reduce_structure(subs: &HashMap<String, String>, current_word: &str, depth: i32) {
    for (k, v) in subs {
        for (m, t) in current_word.match_indices(k) {
            let mut c = current_word.to_owned();
            c.replace_range(m..m + t.len(), v);
            if c == "e" {
                println!("{}", depth);
            }
            else {
                reduce_structure(subs, &c, depth + 1);
            }
        }
    }
}


fn main() {
    let reader = BufReader::new(File::open("data/day19.txt").unwrap());

    let mut substitutions = HashMap::new();
    let mut reverse_substitutions = HashMap::new();
    let mut original = "".to_owned();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(" => ").collect_vec();
        if parts.len() == 2 {
            substitutions
                .entry(parts[0].to_owned())
                .or_insert(Vec::new())
                .push(parts[1].to_owned());
            reverse_substitutions
                .insert(parts[1].to_owned(), parts[0].to_owned());
        } else if parts.len() == 1 {
            original = parts[0].to_owned();
        }
    }

    let mut total = HashSet::new();
    for (k, v) in substitutions {
        for (m, t) in original.match_indices(&k) {
            for p in &v {
                let mut c = original.to_owned();
                c.replace_range(m..m + t.len(), &p);
                total.insert(c);
            }
        }
    }
    print!("{}\n", total.len());

    reduce_structure(&reverse_substitutions, &original, 1);
}