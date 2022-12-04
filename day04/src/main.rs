use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct ElfPair {
    lmin: u8,
    lmax: u8,
    rmin: u8,
    rmax: u8,
}

impl ElfPair {
    fn from_string(s: String) -> Self {
        let v: Vec<&str> = s.split(',').collect();
        let l: Vec<&str> = v[0].split('-').collect();
        let r: Vec<&str> = v[1].split('-').collect();
        ElfPair {
            lmin: l[0].parse::<u8>().unwrap(),
            lmax: l[1].parse::<u8>().unwrap(),
            rmin: r[0].parse::<u8>().unwrap(),
            rmax: r[1].parse::<u8>().unwrap(),
        }
    }
    fn has_complete_overlap(&self) -> bool {
        if self.lmin <= self.rmin && self.lmax >= self.rmax {
            return true;
        }
        if self.rmin <= self.lmin && self.rmax >= self.lmax {
            return true;
        }
        false
    }
    fn has_partial_overlap(&self) -> bool {
        // This does _not_ include the full-overlap case...
        if self.lmin >= self.rmin && self.lmin <= self.rmax {
            return true;
        }

        if self.lmax >= self.rmin && self.lmax <= self.rmax {
            return true;
        }
        false
    }
}

impl fmt::Display for ElfPair {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}-{},{}-{}", self.lmin, self.lmax, self.rmin, self.rmax)
    }
}

fn main() {
    let mut soln = (0, 0);
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            let elf_pair = ElfPair::from_string(line);
            println!(
                "ElfPair {} full-overlap {} partial-overlap {}",
                elf_pair,
                elf_pair.has_complete_overlap(),
                elf_pair.has_partial_overlap()
            );
            if elf_pair.has_complete_overlap() {
                soln.0 += 1;
                soln.1 += 1;
            } else if elf_pair.has_partial_overlap() {
                soln.1 += 1;
            }
        }
    }
    println!("Solutions for Part1,Part2 are {:?}", soln);
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
