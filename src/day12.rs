use std::fs;
use serde_json::Value;

fn parse_number(data: &Value) -> i64{
    let mut value = 0;
    if data.is_number() {
        value = data.as_i64().unwrap();
    }
    else if data.is_array() {
        for i in data.as_array().unwrap() {
            value += parse_number(i);
        }
    }
    else if data.is_object() {
        for (_k, v )in data.as_object().unwrap() {
            if v.is_string() && v.as_str().unwrap() == "red" {
                return 0;
            }
            value += parse_number(v);
        }
    }
    value
}

fn main() {
    let contents = fs::read_to_string("data/day12.txt").unwrap();

    let v: Value = serde_json::from_str(&*contents).unwrap();

    println!("{}", parse_number(&v));
}