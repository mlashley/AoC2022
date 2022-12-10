use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Sub;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn main() {
    // Test
    debug_assert!(part1("./input_sample.txt".into()) == 13);

    // Part 1
    let part1 = part1("./input.txt".into());
    println!("Part1: {}", part1);
    debug_assert!(part1 == 6745); // Keep part 1 working.

    debug_assert!(part2("./input_sample.txt".into()) == 1);
    debug_assert!(part2("./input_sample2.txt".into()) == 36);

    let part2 = part2("./input.txt".into());
    println!("Part2: {}", part2);
    debug_assert!(part2 == 2793); // Keep part 2 working.
}

fn part1(input_filename: String) -> usize {
    let mut head_coord = Point::new(0, 0);
    let mut tail_coord = Point::new(0, 0);
    let mut tail_history = Vec::new();
    tail_history.push(tail_coord);

    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            let a: Vec<&str> = line.split(' ').collect();
            if let Ok(cnt) = a[1].parse::<u32>() {
                // println!("{:?} for {:?}", a[0], cnt);
                for _i in 0..cnt {
                    match a[0] {
                        "R" => head_coord.x += 1,
                        "U" => head_coord.y += 1,
                        "L" => head_coord.x -= 1,
                        "D" => head_coord.y -= 1,
                        _ => println!("Fuck"),
                    }
                    tail_coord.toward(head_coord);
                    tail_history.push(tail_coord);
                }
            }
        }
    } else {
        println!("Failed to read file");
    }
    tail_history.sort();
    tail_history.dedup();
    tail_history.len()
}

fn part2(input_filename: String) -> usize {
    let mut snake = [Point::new(0, 0); 10];

    let mut tail_history = Vec::new();
    tail_history.push(snake[9]);

    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            let a: Vec<&str> = line.split(' ').collect();
            if let Ok(cnt) = a[1].parse::<u32>() {
                // println!("{:?} for {:?}", a[0], cnt);
                for _i in 0..cnt {
                    match a[0] {
                        "R" => snake[0].x += 1,
                        "U" => snake[0].y += 1,
                        "L" => snake[0].x -= 1,
                        "D" => snake[0].y -= 1,
                        _ => println!("Fuck"),
                    }
                    // println!("Head {} for {}/{}",a[0],_i+1,cnt);
                    // print_snake(snake);

                    for i in 0..9 {
                        snake[i + 1].toward(snake[i]);
                        // println!("Elem {} {} for {}/{}",i,a[0],_i+1,cnt);
                        // print_snake(snake);
                    }
                    // print_snake(snake);
                    tail_history.push(snake[9]);
                }
            }
        }
    } else {
        println!("Failed to read file");
    }
    tail_history.sort();
    tail_history.dedup();
    tail_history.len()
}

fn print_snake(snake: [Point; 10]) {
    // for the big example
    for y in (-5..16).rev() {
        'x: for x in -11..15 {
    // for the small example
    // for y in (0..5).rev() {
    //     'x: for x in 0..6 {

            for (i, s) in snake.iter().enumerate() {
                if s.is_at(x, y) {
                    print!("{}", i);
                    continue 'x;
                }
            }
            if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn toward(&mut self, other: Self) -> Self {
        let v = other - *self;
        if v.x > -2 && v.x < 2 && v.y > -2 && v.y < 2 {
            // println!(
            //     "{:?} and {:?} => {:?} overlap or adjacent - do nothing",
            //     *self, other, v
            // )
        } else if v.x.abs() == 2 && v.y.abs() == 2 {
            // println!(
            //     "{:?} and {:?} => {:?} move diagonal",
            //     *self, other, v
            // );
            self.y += v.y / 2;
            self.x += v.x / 2;
        } else if v.x.abs() == 2 {
            // println!(
            //     "{:?} and {:?} => {:?} move on (at least) x",
            //     *self, other, v
            // );
            self.x += v.x / 2;
            self.y += v.y; // this is either zero or -1/+1 for the diag
        } else if v.y.abs() == 2 {
            // println!(
            //     "{:?} and {:?} => {:?} move on (at least) y",
            //     *self, other, v
            // );
            self.y += v.y / 2;
            self.x += v.x; // this is either zero or -1/+1 for the diag
        }
        *self
    }

    fn is_at(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y == y
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
