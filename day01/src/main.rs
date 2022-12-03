use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {
        let mut elf_max = 0;
        let mut current_elf_sum = 0;
        let mut elf_carrying_vector = Vec::with_capacity(260); // we know this is enough from part1

        // Consumes the iterator, returns an (Optional) String
        for cal in lines.flatten() {
            let i = cal.parse::<i32>().unwrap_or(0);
            current_elf_sum += i;
            if i == 0 {
                elf_carrying_vector.push(current_elf_sum);
                if current_elf_sum > elf_max {
                    elf_max = current_elf_sum;
                    println!(
                        "Debug: Elf {} is new max {}",
                        elf_carrying_vector.len(),
                        current_elf_sum
                    );
                }
                current_elf_sum = 0;
            }
        }
        // No blank line at end - check the final elf
        elf_carrying_vector.push(current_elf_sum);
        if current_elf_sum > elf_max {
            elf_max = current_elf_sum;
            println!(
                "Elf {} is new max {}",
                elf_carrying_vector.len(),
                current_elf_sum
            );
        }

        let elf_count = elf_carrying_vector.len();
        println!(
            "Part1: Processed {} elves and max is carrying {} calories",
            elf_count, elf_max
        );
        elf_carrying_vector.sort();
        let top_three = elf_carrying_vector.split_off(elf_count - 3);
        println!("Part2: Top 3 elves {:?}", top_three);
        println!(
            "Part2: Top 3 total {} calories",
            top_three.into_iter().sum::<i32>()
        );
    }
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
