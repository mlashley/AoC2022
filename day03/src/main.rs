use ascii::AsciiChar;
use ascii::AsciiStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut soln = (0, 0);
    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_total = 0;
        for backpack in lines.flatten() {
            let l = backpack.len() / 2;
            let mut compartment1 = backpack;
            let compartment2 = compartment1.split_off(l);

            // Find unique letters by and'ing the two masks.
            let p = get_priority(str_to_mask(compartment1) & str_to_mask(compartment2));
            priority_total += p.unwrap();
        }
        soln.0 = priority_total;
    }

    // Part 2
    let mut mask_array: [u64; 3] = [0; 3];
    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_total = 0;
        for (i, backpack) in lines.enumerate() {
            mask_array[i % 3] = str_to_mask(backpack.unwrap());

            if i % 3 == 2 {
                priority_total +=
                    get_priority(mask_array[0] & mask_array[1] & mask_array[2]).unwrap();
            }
        }
        soln.1 = priority_total;
    }
    println!("==> Solutions for Part1,Part2 {:?} <==", soln);
}

// Convert a string to an int where bits 0 thru 51 represent the presence or absence of [a-z,A-Z]
fn str_to_mask(s: String) -> u64 {
    let ass = AsciiStr::from_ascii(&s).unwrap();
    let base: u64 = 2;
    let mut mask: u64 = 0;
    for ch in ass.chars() {
        if ch >= AsciiChar::A && ch <= AsciiChar::Z {
            let d = 26 + ch.as_byte() - AsciiChar::A.as_byte();
            mask |= base.pow(d as u32);
        } else if ch >= AsciiChar::a && ch <= AsciiChar::z {
            let d = ch.as_byte() - AsciiChar::a.as_byte();
            mask |= base.pow(d as u32);
        }
    }
    mask
}

// Find the set bitbumber (I'm sure there should be a built-in/crate for this...)
fn get_priority(i: u64) -> Option<u64> {
    let base: u64 = 2;
    for c in 0..52 {
        if i == base.pow(c) {
            return Some(c as u64 + 1);
        }
    }
    None
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
