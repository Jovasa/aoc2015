use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::OpCode::{LSHIFT, MOVE, NOT, OR, RSHIFT};

#[derive(Copy, Clone)]
enum OpCode {
    NOT,
    LSHIFT,
    RSHIFT,
    AND,
    OR,
    MOVE,
}

struct Operation {
    op: OpCode,
    a: String,
    b: Option<String>,
    out: String,
}


fn get_value(operations: &HashMap<String, Operation>, memory: &mut HashMap<String, u16>, register: &str) -> u16 {
    if memory.contains_key(register) {
        return memory[register];
    }
    let op = &operations[register];
    let first = match &op.a.parse::<u16>() {
        Ok(i) => i.to_owned(),
        Err(_) => get_value(operations, memory, op.a.as_str())
    };
    let second = match &op.b {
        Some(t) => {
            match t.parse::<u16>() {
                Ok(i) => i,
                Err(_) => get_value(operations, memory, t.as_str())
            }
        }
        None => 0
    };

    let temp = match op.op {
        MOVE => first,
        NOT => !first,
        OR => first | second,
        OpCode::AND => first & second,
        LSHIFT => first << second,
        RSHIFT => first >> second
    }.to_owned();
    memory.insert(register.to_string(), temp);
    temp
}


fn main() {
    let reader = BufReader::new(File::open("data/day7.txt").unwrap());

    let mut operations = HashMap::new();

    for line in reader.lines() {
        let line: Vec<String> = line.unwrap().split(" ").map(|x| x.to_string()).collect();
        match line.len() {
            3 => {
                let out = line[2].to_string();
                operations.insert(
                    out.clone(),
                    Operation { op: MOVE, a: line[0].to_string(), b: None, out },
                );
            }
            4 => {
                let out = line[3].to_string();
                operations.insert(
                    out.clone(),
                    Operation { op: NOT, a: line[1].to_string(), b: None, out },
                );
            }
            5 => {
                let op = match line[1].as_str() {
                    "OR" => OR,
                    "AND" => OpCode::AND,
                    "LSHIFT" => LSHIFT,
                    "RSHIFT" => RSHIFT,
                    _ => unreachable!()
                };
                let out = line[4].to_string();
                operations.insert(
                    out.clone(),
                    Operation { op, a: line[0].to_string(), b: Some(line[2].to_string()), out },
                );
            }
            _ => unreachable!()
        }
    }

    let mut memory: HashMap<String, u16> = HashMap::new();

    let value = get_value(&operations, &mut memory, "a");
    println!("{}", value);

    operations.insert(
        "b".to_string(),
        Operation { op: MOVE, a: value.to_string(), b: None, out: "b".to_string() }
    );
    memory.clear();
    println!("{}", get_value(&operations, &mut memory, "a"));
}