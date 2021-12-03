use std::fs::File;
use std::io::{self, BufRead};

fn part1() {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut data: Vec<i32> = Vec::new();
    for l in lines {
        let v = l.unwrap().parse::<i32>().unwrap();
        data.push(v);
    }

    let mut increase_count = 0;
    let mut prev_val: &i32 = &data[0];
    for v in data.iter().skip(1) {
        if v > prev_val {
            increase_count += 1;
        }
        prev_val = v;
    }

    println!("Count: {}", increase_count);
}

fn part2() {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut data: Vec<i32> = Vec::new();
    for l in lines {
        let v = l.unwrap().parse::<i32>().unwrap();
        data.push(v);
    }

    let mut window_sums: Vec<i32> = Vec::new();
    let w = data.windows(3);

    for v in w {
        window_sums.push(v[0] + v[1] + v[2]);
    }

    let mut increase_count = 0;
    let mut prev_val: &i32 = &window_sums[0];
    for v in window_sums.iter().skip(1) {
        if v > prev_val {
            increase_count += 1;
        }
        prev_val = v;
    }

    println!("Window Increase Count: {}", increase_count);
}

fn main() {
    //part1();
    part2();
}
