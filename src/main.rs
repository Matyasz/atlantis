use std::io::{self, BufRead};
use serde_json::{self, Value};

fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let data: Value = serde_json::from_str(&line.unwrap()).unwrap();

        println!("{}", data["1"]);
    }
}
