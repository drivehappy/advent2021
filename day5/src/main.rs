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


struct Coord {
    x: i32,
    y: i32,
}
type CoordPair = (Coord, Coord);

fn parse_data() -> Vec<CoordPair> {
    let lines = read_lines();

    let coord: Vec<(Coord, Coord)> =
        lines
        .into_iter()
        .map(|l| {
            let n: Vec<&str> = l.split(" -> ").collect();
            let fst: Vec<&str> = n[0].split(',').collect();
            let snd: Vec<&str> = n[1].split(',').collect();

            ( Coord { x: fst[0].parse::<i32>().unwrap(), y: fst[1].parse::<i32>().unwrap() }
            , Coord { x: snd[0].parse::<i32>().unwrap(), y: snd[1].parse::<i32>().unwrap() }
            )
        })
        .collect();

    coord
}


type Grid = [[i8; 1000]; 1000];

fn is_vertical(c: &CoordPair) -> bool {
    c.0.x == c.1.x
}
fn is_horizontal(c: &CoordPair) -> bool {
    c.0.y == c.1.y
}

fn update_grid(grid: &mut Grid, lines: &Vec<CoordPair>) {
    use std::cmp::{min, max};

    for l in lines {
        if !is_vertical(l) && !is_horizontal(l) {
            continue;
        }

        let x_min = min(l.0.x, l.1.x);
        let x_max = max(l.0.x, l.1.x);
        let y_min = min(l.0.y, l.1.y);
        let y_max = max(l.0.y, l.1.y);

        for x in x_min..=x_max {
            for y in y_min..=y_max {
                grid[x as usize][y as usize] += 1;
            }
        }
    }
}

fn part1() {
    let data = parse_data();
    let mut grid: Grid = [[0; 1000]; 1000];

    update_grid(&mut grid, &data);

    // Count
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x as usize][y as usize] > 1 {
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
}

fn main() {
    part1();
}

