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

fn part1() {
    let lines = read_lines();

    let mut data: Vec<[i8; 12]> = Vec::new();
    let total_line_count = lines.len();
    for l in lines {
        fn parse(s: &str) -> i8 {
            if &s[..1] == "1" { 1 } else { 0 }
        }
        
        data.push(
            [ parse(&l[0..1])
            , parse(&l[1..2])
            , parse(&l[2..3])
            , parse(&l[3..4])
            , parse(&l[4..5])
            , parse(&l[5..6])
            , parse(&l[6..7])
            , parse(&l[7..8])
            , parse(&l[8..9])
            , parse(&l[9..10])
            , parse(&l[10..11])
            , parse(&l[11..12])
            ]);
    }

    // Compute column bit counts
    let mut total_ones = [0; 12];
    let mut total_zeroes = [0; 12];
    for d in data {
        for x in 0..12 {
            if d[x] == 1 { total_ones[x] += 1; } else { total_zeroes[x] += 1; }
        }
    }

    //
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    for x in 0..12 {
        if total_ones[x] >= total_zeroes[x] {
            gamma.push_str("1");
            epsilon.push_str("0");
        } else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }
    }
    
    println!("Gamma: {}", gamma);
    println!("Epsilon: {}", epsilon);

    // Note: Manually converted binary -> dec
    // Gamma: 1565
    // Ep:    2530
    // Mul:   3959450
}

fn main() {
    part1();
}
