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

#[derive(Debug)]
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


const DIM: usize = 1000;
type Grid = [[i8; DIM]; DIM];

fn generate_steps(a: i32, b: i32) -> Vec<i32> {
    if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

fn is_vertical(c: &CoordPair) -> bool {
    c.0.x == c.1.x
}
fn is_horizontal(c: &CoordPair) -> bool {
    c.0.y == c.1.y
}

fn print_grid(grid: &Grid) {
    for x in 0..DIM {
        for y in 0..DIM {
            print!("{}", grid[x][y]);
        }
        println!("");
    }
}

fn update_grid(grid: &mut Grid, lines: &Vec<CoordPair>) {
    for l in lines {
        if is_vertical(l) || is_horizontal(l) {
            for x in generate_steps(l.0.x, l.1.x) {
                for y in generate_steps(l.0.y, l.1.y) {
                    grid[x as usize][y as usize] += 1;
                }
            }
        } else {
            // Diagonal
            let xs = generate_steps(l.0.x, l.1.x);
            let ys = generate_steps(l.0.y, l.1.y);

            let coords = xs.iter().zip(ys.iter());
            for (x, y) in coords {
                grid[*x as usize][*y as usize] += 1;
            }
        }
    }
}

fn part1() {
    let data = parse_data();
    let mut grid: Grid = [[0; DIM]; DIM];

    update_grid(&mut grid, &data);

    // Count
    let mut count = 0;
    for x in 0..DIM {
        for y in 0..DIM {
            if grid[x as usize][y as usize] > 1 {
                count += 1;
            }
        }
    }
    println!("Count: {}", count);
    //print_grid(&grid);
}

fn main() {
    part1();
}

