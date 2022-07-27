use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;


enum OpCode {
    JIO,
    INC,
    TPL,
    JMP,
    JIE,
    HLF,
}


struct Operation{
    op: OpCode,
    register: char,
    offset: i32,
}


fn main() {
    let reader = BufReader::new(File::open("data/day23.txt").unwrap());

    let operations = reader
        .lines()
        .map(|x| {
            let x = x.unwrap();
            let t = x.split(" ").collect_vec();
            match t[0] {
                "jio" => {
                    Operation{op: OpCode::JIO,
                        register: t[1].chars().next().unwrap(),
                        offset: t[2].parse().unwrap()}
                },
                "jie" => {
                    Operation{op: OpCode::JIE,
                        register: t[1].chars().next().unwrap(),
                        offset: t[2].parse().unwrap()}
                },
                "jmp" => {
                    Operation{op: OpCode::JMP,
                        register: '_',
                        offset: t[1].parse().unwrap()}
                },
                "tpl" => {
                    Operation{
                        op: OpCode::TPL, register: t[1].chars().next().unwrap(), offset: 0,
                    }
                },
                "hlf" => {
                    Operation{
                        op: OpCode::HLF, register: t[1].chars().next().unwrap(), offset: 0,
                    }
                },
                "inc" => {
                    Operation{
                        op: OpCode::INC, register: t[1].chars().next().unwrap(), offset: 0,
                    }
                },
                _ => unreachable!()
            }
        }).collect_vec();

    let mut registers = HashMap::new();
    registers.insert('a', 1);
    registers.insert('b', 0);
    let mut i = 0i32;
    while (i as usize) < operations.len() {
        let op = &operations[i as usize];
        match op.op {
            OpCode::JMP => i += op.offset,
            OpCode::JIE => if (registers[&op.register] & 1) == 0 { i += op.offset} else {i += 1},
            OpCode::JIO => if registers[&op.register] == 1 { i += op.offset} else {i += 1},
            OpCode::INC => { *registers.entry(op.register).or_insert(0) += 1; i+=1 }
            OpCode::TPL => { *registers.entry(op.register).or_insert(0) *= 3; i+=1 }
            OpCode::HLF => { *registers.entry(op.register).or_insert(0) >>= 1; i+=1 }
        }
    }
    println!("{}", registers[&'b']);
}