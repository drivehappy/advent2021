use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut data: Vec<i32> = Vec::new();
    for l in lines {
        let v = l.unwrap().parse::<i32>().unwrap();
        data.push(v);
    }

    let mut increase_count = 0;
    let mut prev_val: &i32 = &data[0];
    for (i, v) in data.iter().skip(1).enumerate() {
        if v > prev_val {
            increase_count += 1;
        }
        prev_val = v;
    }

    println!("Count: {}", increase_count);
}
