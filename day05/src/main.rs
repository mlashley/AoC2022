use ansi_escapes;
use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{thread, time};

fn main() {
    let p1 = solve("./input.txt", true);
    println!("Part1: {}", p1);
    assert!(p1 == "LBLVVTVLP");
    let p2 = solve("./input.txt", false);
    println!("Part2: {}", p2);
    assert!(p2 == "TPFFBDRJD");
}

fn solve(filename: &str, is_part1: bool) -> String {
    if let Ok(lines) = read_lines(filename) {
        let layout_re = Regex::new(r"^(\[[A-Z]\]|\s\s\s\s)").unwrap();
        let digit_re = Regex::new(r"(\d+)").unwrap();

        let mut layoutdone = false;

        let mut board: Vec<VecDeque<char>> = Vec::new();
        for line in lines.flatten() {
            if !layoutdone && layout_re.is_match(line.as_str()) {
                let c = line.chars().collect::<Vec<char>>();
                let l = line.len() + 1;
                if board.is_empty() {
                    // Initialize size as linelen+1 / 4
                    for _i in 0..l / 4 {
                        board.push(VecDeque::new());
                    }
                }
                for i in (1..l).step_by(4) {
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
                    // println!("{}Move {} from {} to {}", ansi_escapes::ClearScreen,count, from, to);
                    if is_part1 {
                        move_cargo(&mut board, from, to, count);
                    } else {
                        move_cargo_part2(&mut board, from, to, count);
                    }
                    // print_board(&board);
                    // thread::sleep(time::Duration::from_millis(100));
                }
            }
        }
        return board_top(&board);
    }
    panic!("Couldn't read file");
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

fn move_cargo_part2(board: &mut [VecDeque<char>], from: usize, to: usize, count: usize) {
    let mut intermediate = VecDeque::new();
    for _c in 0..count {
        intermediate.push_front(board[from - 1].pop_back().unwrap());
    }
    for _c in 0..count {
        board[to - 1].push_back(intermediate.pop_front().unwrap());
    }
}

fn board_top(board: &[VecDeque<char>]) -> String {
    board.iter().map(|x| x[x.len() - 1]).collect::<String>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
