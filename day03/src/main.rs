use ascii::AsciiChar;
use ascii::AsciiStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_total = 0;
        for backpack in lines.flatten() {
            let l = backpack.len() / 2;
            let mut compartment1 = backpack;
            let compartment2 = compartment1.split_off(l);
            // println!("{} {}",compartment1,compartment2);

            let mask1 = str_to_mask(compartment1);
            println!("Mask1 {:#b}", mask1);
            let mask2 = str_to_mask(compartment2);
            println!("Mask2 {:#b}", mask2);

            println!("AND: {:#b}", mask1 & mask2);

            let p = get_priority(mask1 & mask2);
            priority_total += p.unwrap();
            println!("Prio: {} Total: {} ", p.unwrap(), priority_total);
        }
    }

    // Part 2
    let mut mask_array: [u64; 3] = [0; 3];
    if let Ok(lines) = read_lines("./input.txt") {
        let mut priority_total = 0;
        for (i,backpack) in lines.enumerate() {

            mask_array[i%3] = str_to_mask(backpack.unwrap());
            
            if i%3 == 2 { // We have our group
                let uniq = mask_array[0] & mask_array[1] & mask_array[2];
                let p = get_priority(uniq);
                priority_total += p.unwrap();
                println!("Prio: {} Total: {} ", p.unwrap(), priority_total);
            }
        }
    }

}

fn str_to_mask(s: String) -> u64 {
    let ass = AsciiStr::from_ascii(&s).unwrap();
    // println!("AS: {:?} ", ass.as_bytes());
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
    // println!("i = {}", mask);
    mask
}

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
