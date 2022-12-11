use std::collections::VecDeque;
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
    debug_assert!(part1("./input_sample.txt".into()).unwrap() == 13140);

    // Part 1
    let part1 = part1("./input.txt".into()).unwrap();
    println!("Part1: {}", part1);
    debug_assert!(part1 == 15880); // Keep part 1 working.

    // Part2 sits so nicely inside part 1 - I didn't refactor.
}
#[derive(Debug, Default)]
struct Cpu {
    x: i32,
    instruction_cycles_remaining: usize,
    cycle: usize,
    history: Vec<i32>,
}

#[derive(Debug, Default, PartialEq)]
struct Instruction {
    is_add: bool, // else no-op in our ISA :)
    operand: i32,
    cycles: usize,
}

impl Instruction {
    fn new(is_add: bool, operand: i32, cycles: usize) -> Self {
        Self {
            is_add,
            operand,
            cycles,
        }
    }
    fn from_inputstring(s: String) -> Result<Self, String> {
        if s.eq("noop") {
            return Ok(Self::new(false, 0, 1));
        } else {
            let a: Vec<&str> = s.split(' ').collect();
            if let Ok(operand) = a[1].parse::<i32>() {
                return Ok(Self::new(true, operand, 2));
            } else {
                println!("Error parsing operand '{}' in '{}'", a[1], s);
            }
        }
        Err("Unsupported instruction".to_string())
    }
}

impl Cpu {
    fn new(x: i32, instruction_cycles_remaining: usize, cycle: usize, history: Vec<i32>) -> Self {
        Self {
            x,
            cycle,
            instruction_cycles_remaining,
            history,
        }
    }
    fn boot() -> Self {
        Self::new(1, 0, 0, Vec::new())
    }
    fn execute(&mut self, instructions: &mut VecDeque<Instruction>) -> i32 {
        let mut instruction = match instructions.pop_front() {
            Some(x) => x,
            None => {
                panic!("CPU Pipeline undeflow!")
            }
        };
        self.instruction_cycles_remaining = instruction.cycles;

        let mut crt = String::new();
        loop {
            self.history.push(self.x);

            let beampos =  self.cycle as i32 % 40;
            let pixel = if self.x >= beampos-1 && self.x <= beampos+1 {
                '#'
            } else {
                '.'
            };
            crt.push(pixel);

            self.cycle += 1;
            self.instruction_cycles_remaining -= 1;

            // println!("cycle: {} x: {}, c_rem: {}",self.cycle,self.x,self.instruction_cycles_remaining);

            if self.instruction_cycles_remaining == 0 {
                // perform effect
                if instruction.is_add {
                    self.x += instruction.operand;
                }

                // fetch next
                instruction = match instructions.pop_front() {
                    Some(x) => x,
                    None => {
                        break;
                    }
                };
                // println!("Fetched {} ",instruction);
                self.instruction_cycles_remaining = instruction.cycles;
            }
        }

        let mut signal_strength = 0;
        for i in (20..=220).step_by(40) {
            let j = (i as i32) * self.history[i - 1];
            signal_strength += j;
            // println!( "{} * {} = {} ... ss: {}", i, self.history[i - 1], j, signal_strength )
        }
        for (i,c) in crt.chars().enumerate() {
            print!("{}",c);
            if i % 40 == 39 { println!() }
        }
        println!();

        signal_strength
    }
}

fn part1(input_filename: String) -> Result<i32, String> {
    let mut instructions = VecDeque::new();

    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            let insn = Instruction::from_inputstring(line)?;
            instructions.push_back(insn);
        }
    }
    let mut cpu = Cpu::boot();
    Ok(cpu.execute(&mut instructions))
}
