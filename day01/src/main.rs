use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    if let Ok(lines) = read_lines("./input.txt") {
        let mut elf_total_calories:Vec<i32> = Vec::new();
        // Consumes the iterator, returns an (Optional) String
        let mut elf_count = 0;
        let mut elf_max = 0;
        let mut current_elf_sum = 0;
        for line in lines {
            if let Ok(cal) = line {
                let i = cal.parse::<i32>().unwrap_or(0) ;
                current_elf_sum += i;
                if i == 0 {
//                        println!("{}", current_elf_sum);
                    
                    elf_count += 1;
                    if current_elf_sum > elf_max  {
                        elf_max = current_elf_sum;
                        println!("Elf {} is new max {}",elf_count,current_elf_sum);
                    }
                    current_elf_sum=0;
                }
            }
        }
        // No blank line at end - check the final elf
        elf_count += 1;
        if current_elf_sum > elf_max  {
            elf_max = current_elf_sum;
            println!("Elf {} is new max {}",elf_count,current_elf_sum);
        }
        println!("Processed {} elves and max is {}",elf_count,elf_max);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

