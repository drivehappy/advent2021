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

    // Build a small lookup table for the Part2 fuel costs, assumes some max value from input
    let mut fuel_cost_lookup: [i32; 2000] = [0; 2000];
    let mut curr_cost: i32 = 0;
    for i in 0 .. 2000 {
        curr_cost += i;
        fuel_cost_lookup[i as usize] = curr_cost;
    }

    //
    fn calc_fuel_cost(pos_diffs: &[i32]) -> i32 {
        pos_diffs.iter().sum()
    }

    let mut optimal = 1000000000;
    let mut diff = 0;
    for i in min ..= max {
        let mut position_diffs: Vec<i32> = Vec::<i32>::with_capacity(positions.len());
        for p in positions {
            let distance = (p - i).abs();

            //let cost_part1 = distance;
            let cost_part2 = fuel_cost_lookup[distance as usize];
            position_diffs.push(cost_part2);
        }

        let f = calc_fuel_cost(&position_diffs);
        if f < optimal {
            optimal = f;
            diff = i;
        }
    }

    (optimal, diff)
}

fn main() {
    let positions = parse_data();

    let (optimal, diff) = calculate_optimal_fuel_cost(&positions);
    println!("Optimal: {}, Diff: {}", optimal, diff);
}

