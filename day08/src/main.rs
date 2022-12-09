use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
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
    debug_assert!(parse("./input_sample.txt".into()) == (21, 8));
    debug_assert!(parse("./input_malc.txt".into()) == (58, 8));

    // Part 1
    let (part1, part2) = parse("./input.txt".into());
    println!("Part1: {}", part1);
    println!("Part2: {}", part2);
    debug_assert!(part1 == 1798); // Keep part 1 working.
    debug_assert!(part2 == 259308); // Keep part 2 working.
}

fn parse(input_filename: String) -> (u32, u32) {
    let mut grid: Vec<Vec<u32>> = Vec::new();

    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            let p: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
            grid.push(p);
        }
    } else {
        println!("Failed to read file");
    }
    // print_grid(&grid);
    let g2 = grid.clone(); // I should properly learn how borrowing works.
    let p1 = count_visible(grid);
    let p2 = scenic_score(g2);
    (p1, p2)
}

fn count_visible(grid: Vec<Vec<u32>>) -> u32 {
    let dim = grid.len();

    let mut mask = Vec::new();
    for _r in 0..dim {
        mask.push(vec![false; dim]);
    }
    // Mark the Outside Visible
    for y in 0..dim {
        for x in 0..dim {
            if x == 0 || x == dim - 1 || y == 0 || y == dim - 1 {
                mask[y][x] = true;
            }
        }
    }

    for y in 1..dim - 1 {
        for x in 1..dim - 1 {
            // Left
            let mut visible = true;
            for lookx in 0..x {
                if grid[y][x] <= grid[y][lookx] {
                    visible = false
                }
            }
            mask[y][x] |= visible;

            // Right
            visible = true;
            for lookx in x + 1..dim {
                if grid[y][x] <= grid[y][lookx] {
                    visible = false
                }
            }
            mask[y][x] |= visible;

            // Top
            visible = true;
            for looky in 0..y {
                if grid[y][x] <= grid[looky][x] {
                    visible = false
                }
            }
            mask[y][x] |= visible;

            // Bottom
            visible = true;
            for looky in y + 1..dim {
                if grid[y][x] <= grid[looky][x] {
                    visible = false
                }
            }
            mask[y][x] |= visible;
        }
    }

    // print_grid(&mask);
    let mut cnt = 0;
    for y in 0..dim {
        for x in 0..dim {
            if mask[y][x] {
                cnt += 1
            }
        }
    }
    println!("Count: {}", cnt);
    cnt
}

fn scenic_score(grid: Vec<Vec<u32>>) -> u32 {
    let mut scores: Vec<u32> = Vec::new();
    let dim = grid.len();

    for y in 1..dim - 1 {
        for x in 1..dim - 1 {
            let mut scenic_score = (0, 0, 0, 0);
            // Look Left
            for lookx in (0..x).rev() {
                scenic_score.1 += 1;
                if grid[y][x] <= grid[y][lookx] {
                    break;
                }
            }

            // Look Right
            for lookx in x + 1..dim {
                scenic_score.2 += 1;
                if grid[y][x] <= grid[y][lookx] {
                    break;
                }
            }

            // Look Up
            for looky in (0..y).rev() {
                scenic_score.0 += 1;
                if grid[y][x] <= grid[looky][x] {
                    break;
                }
            }

            // Look Down
            for looky in y + 1..dim {
                scenic_score.3 += 1;
                if grid[y][x] <= grid[looky][x] {
                    break;
                }
            }
            scores.push(scenic_score.0 * scenic_score.1 * scenic_score.2 * scenic_score.3);
        }
    }
    *scores.iter().max().unwrap()
}

fn print_grid<T: Debug>(grid: &Vec<Vec<T>>) {
    for row in grid {
        println!("{:?}", row);
    }
}
