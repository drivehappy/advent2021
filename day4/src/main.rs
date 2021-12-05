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

struct Board {
    value: [[i8; 5]; 5],
    selected: [[bool; 5]; 5],
}

impl Board {

    fn select(&mut self, i: i8) {
        'outer: for x in 0..5 {
            for y in 0..5 {
                if self.value[x][y] == i {
                    self.selected[x][y] = true;
                    break 'outer;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        // Check rows
        for x in 0..5 {
            let mut row_bingo = true;
            for y in 0..5 {
                row_bingo = row_bingo && self.selected[x][y];
            }

            if row_bingo { return true; }
        }

        // Check cols
        for y in 0..5 {
            let mut col_bingo = true;
            for x in 0..5 {
                col_bingo = col_bingo && self.selected[x][y];
            }

            if col_bingo { return true; }
        }

        false
    }

}

// Assumes the board has bingo, caclulates total of unselected elements
fn board_score(b: &Board) -> i32 {
    let mut score: i32 = 0;
    for x in 0..5 {
        for y in 0..5 {
            if !b.selected[x][y] {
                score += b.value[x][y] as i32;
            }
        }
    }
    score
}

fn part1() {
    let data = read_lines();

    // Parse and build random numbers
    let rand_nums: Vec<&str> = data[0].split(',').collect();
    let mut random_numbers: Vec<i8> = Vec::new();
    for r in rand_nums {
        random_numbers.push(r.parse::<i8>().unwrap());
    }

    // Parse and build boards
    let mut boards: Vec<Board> = Vec::new();
    let board_lines: Vec<&String> = data[1..].into_iter().filter(|x| x.len() > 0).collect();
    for c in board_lines.chunks(5) {
        let mut b: Board = Board { value: [[0; 5]; 5], selected: [[false; 5]; 5] };
        for (i, chk) in c.iter().enumerate() {
            let ints: Vec<i8> = chk.split_whitespace().map(|x| x.parse::<i8>().unwrap()).collect();
            b.value[i] = [ ints[0], ints[1], ints[2], ints[3], ints[4] ];
            b.selected[i] = [ false, false, false, false, false ];
        }

        boards.push(b);
    }

    // Evaluate boards
    'outer: for x in random_numbers {
        // Update board state
        for b in &mut boards {
            b.select(x);
        }

        // Check if bingo
        for b in &boards {
            if b.is_bingo() {
                let bs = board_score(b) * (x as i32);
                println!("Bingo found, score: {}", bs);
                break 'outer;
            }
        }
    }
}

fn main() {
    part1();
}

