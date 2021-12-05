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

fn parse_data() -> Vec<[i8; 12]> {
    let lines = read_lines();

    let mut data: Vec<[i8; 12]> = Vec::new();
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

    data
}

fn part1() {
    let data = parse_data();

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
}

fn part2() {
    let data: Vec<[i8; 12]> = parse_data();

    // Oxygen generator rating
    let mut data_ogr = data.clone();
    for x in 0..12 {
        // Calculate new totals for the remaining filtered data
        let mut total_ones = 0;
        let mut total_zeroes = 0;
        for d in &data_ogr {
            if d[x] == 1 { total_ones += 1; } else { total_zeroes += 1; }
        }

        println!("DEBUG OGR ({}): {} ({}, {})", x, data_ogr.len(), total_ones, total_zeroes);
        if total_ones >= total_zeroes {
            data_ogr = data_ogr.into_iter().filter(|d| d[x] == 1).collect();
        } else {
            data_ogr = data_ogr.into_iter().filter(|d| d[x] == 0).collect();
        }

        if data_ogr.len() == 1 { break; }
    }

    // CO2 scrubber
    let mut data_co2 = data.clone();
    for x in 0..12 {
        // Calculate new totals for the remaining filtered data
        let mut total_ones = 0;
        let mut total_zeroes = 0;
        for d in &data_co2 {
            if d[x] == 1 { total_ones += 1; } else { total_zeroes += 1; }
        }

        println!("DEBUG CO2 ({}): {} ({}, {})", x, data_co2.len(), total_ones, total_zeroes);
        if total_ones < total_zeroes {
            data_co2 = data_co2.into_iter().filter(|d| d[x] == 1).collect();
        } else {
            data_co2 = data_co2.into_iter().filter(|d| d[x] == 0).collect();
        }

        if data_co2.len() == 1 { break; }
    }

    fn print_binarr(d: &[i8; 12]) -> String {
        let mut s = "".to_string();
        for x in 0..12 {
            if d[x] == 1 { s.push_str("1"); } else { s.push_str("0"); }
        }
        s
    }

    println!("Oxygen generator rating: {}", print_binarr(&data_ogr[0]));
    println!("CO2 scrubber rating: {}", print_binarr(&data_co2[0]));

    // OGR: 2039
    // CO2: 3649
}

fn main() {
    //part1();
    part2();
}

