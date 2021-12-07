use std::fs::File;
use std::io::{self, BufRead};


fn read_lines() -> Vec<String> {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut data: Vec<String> = Vec::new();
    for l in lines {
        data.push(l.unwrap());
    }
    data
}

fn parse_data() -> Vec<i32> {
    let l = &read_lines()[0];
    let data: Vec<&str> = l.split(",").collect();
    data.into_iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn part1() {
    let mut state: Vec<i32> = parse_data();
    
    for x in 0..80 {
        let mut new_state: Vec<i32> = vec![];
        for s in state {
            if s > 0 {
                new_state.push(s - 1);
            } else {
                new_state.push(6); // Self 
                new_state.push(8); // Child
            }
        }

        state = new_state;
    }

    println!("Count: {}", state.len());
}

fn main() {
    part1();
}

