use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use ascii::AsciiChar;
use ascii::AsciiStr;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {

        let mut priority_total = 0;
        for backpack in lines.flatten() {
            let l = backpack.len()/2;
            let mut compartment1 = backpack;
            let compartment2 = compartment1.split_off(l);
            // println!("{} {}",compartment1,compartment2);

            let mask1 = strToMask(compartment1);
            println!("Mask1 {:#b}",mask1);
            let mask2 = strToMask(compartment2);
            println!("Mask2 {:#b}",mask2);

            println!("AND: {:#b}",mask1&mask2);

            let p = get_priority(mask1&mask2);
            priority_total += p.unwrap();
            println!("Prio: {} Total: {} ",p.unwrap(),priority_total)
            
        }
    }
}

fn strToMask(s: String) -> u64 {
    let ass = AsciiStr::from_ascii(&s).unwrap();
    println!("AS: {:?} ",ass.as_bytes());

    let base: u64 = 2;
    let mut mask:u64 = 0;
    
    for c in ass.chars() {
        if c >= AsciiChar::A && c <= AsciiChar::Z{
            let d = 26+c.as_byte()-AsciiChar::A.as_byte();
            mask |= base.pow(d as u32);
        } else if c >= AsciiChar::a && c <= AsciiChar::z {
            let d = c.as_byte()-AsciiChar::a.as_byte();
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