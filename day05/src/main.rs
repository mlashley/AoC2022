use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let layout_re = Regex::new(r"^(\[[A-Z]\]|\s\s\s\s)").unwrap();
        let digit_re = Regex::new(r"(\d+)").unwrap();

        let mut layoutdone = false;

        let mut board: Vec<VecDeque<char>> = Vec::new();
        for line in lines.flatten() {
            if !layoutdone && layout_re.is_match(line.as_str()) {
                let c = line.chars().collect::<Vec<char>>();
                let l = line.len();
                if board.is_empty() {
                    // Initialize size as linelen+1 / 4
                    for _i in 0..(l + 1) / 4 {
                        board.push(VecDeque::new());
                    }
                }
                println!("l:{} boardlen:{}", l, board.len());

                for i in (1..l).step_by(4) {
                    print!("{}[{}] ", c[i], (i - 1) / 4);
                    if c[i] != ' ' {
                        board[(i - 1) / 4].push_front(c[i]);
                    }
                }
            }

            if digit_re.is_match(line.as_str()) {
                if !layoutdone {
                    // NB this nicely skips the column numbers.
                    println!("Starting board:");
                    print_board(&board);
                    layoutdone = true;
                } else {
                    let digits = digit_re
                        .find_iter(line.as_str())
                        .map(|s| s.as_str().parse().unwrap())
                        .collect::<Vec<usize>>();
                    let (count, from, to) = (digits[0], digits[1], digits[2]);
                    println!("Move {} from {} to {}", count, from, to);
                    move_cargo(&mut board, from, to, count);
                    print_board(&board);
                }
            }
        }
        board_top(&board);
    }
}



//fn print_board(board: &Vec<VecDeque<char>>) { // cargo-clippy says the below is 'better' - I need to understand why.
fn print_board(board: &[VecDeque<char>]) {
    for (i, row) in board.iter().enumerate() {
        print!("{}: ", i + 1);
        for elem in row {
            print!("{} ", elem);
        }
        println!();
    }
}

fn move_cargo(board: &mut [VecDeque<char>], from: usize, to: usize, count: usize) {
    for _c in 0..count {
        let intermediate = board[from - 1].pop_back().unwrap();
        board[to - 1].push_back(intermediate);
    }
}

fn board_top(board: &Vec<VecDeque<char>>) {
    for row in board {
        print!("{}", row[row.len() - 1]);
    }
    println!();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
