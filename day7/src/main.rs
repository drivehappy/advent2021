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
    //let l = "16,1,2,0,4,2,7,1,2,14";
    let data: Vec<&str> = l.split(",").collect();
    data.into_iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

fn calculate_optimal_fuel_cost(positions: &[i32]) -> (i32, i32) {
    let min = *(positions.into_iter().min().unwrap());
    let max = *(positions.into_iter().max().unwrap());

    fn calc_fuel_cost(pos_diffs: &[i32]) -> i32 {
        pos_diffs.iter().sum()
    }

    let mut optimal = 1000000000;
    let mut diff = 0;
    for i in min ..= max {
        let mut position_diffs: Vec<i32> = Vec::<i32>::with_capacity(positions.len());
        for p in positions {
            position_diffs.push((p - i).abs());
        }

        let f = calc_fuel_cost(&position_diffs);
        if f < optimal {
            optimal = f;
            diff = i;
        }
    }

    (optimal, diff)
}

fn part1() {
    let positions = parse_data();

    let (optimal, diff) = calculate_optimal_fuel_cost(&positions);
    println!("Optimal: {}, Diff: {}", optimal, diff);
}

fn main() {
    part1();
}

