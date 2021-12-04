use std::fs::File;
use std::io::{self, BufRead};


enum Direction {
    Forward(i32),
    Backward(i32),
    Up(i32),
    Down(i32),
}

fn part1() {
    let f = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(f).lines();

    let mut data: Vec<Direction> = Vec::new();
    for l in lines {
        let a = l.unwrap();
        let v: Vec<&str> = a.split(' ').collect();
        let v1 = v[1].parse::<i32>().unwrap();

        match v[0] {
            "forward"  => data.push(Direction::Forward(v1)),
            "backward" => data.push(Direction::Backward(v1)),
            "up"       => data.push(Direction::Up(v1)),
            "down"     => data.push(Direction::Down(v1)),
            _          => ()
        }
    }

    //
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for d in data {
        match d {
            Direction::Forward(x) => {
                position += x;
                depth += aim * x;
            },
            Direction::Backward(x) => position -= x,
            Direction::Up(x) => aim -= x,
            Direction::Down(x) => aim += x,
        }
    }

    println!("Position: {}", position);
    println!("Depth: {}", depth);

    println!("Answer: {}", position * depth);
}

fn main() {
    part1();
}
