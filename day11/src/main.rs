use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Div;
use std::path::Path;
use regex::Regex;
use std:: collections::VecDeque;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Test
    debug_assert!(part1("./input_sample.txt".into()).unwrap() == 10605);

    // Part 1
    let part1 = part1("./input.txt".into()).unwrap();
    println!("Part1: {}", part1);
    debug_assert!(part1 == 55458); // Keep part 1 working.

    // Part2 sits so nicely inside part 1 - I didn't refactor.
}
#[derive(Debug, Default, PartialEq)]
enum OpType {
    #[default]
    Add,
    Multiply,
}
#[derive(Debug, Default)]
struct Monkey<T> {
    operation_rhs: Option<T>, // None means 'self' (or 'old')
    operation: OpType, 
    test_divisor: T,
    items: VecDeque<T>,
    destination_monkey: (u8,u8),
    inspections: u32,
}

impl<T: std::str::FromStr + std::ops::AddAssign + std::ops::MulAssign + From<u32> + Ord + Div<Output=T> + std::ops::Rem<Output=T> + Copy > Monkey<T>
{

    fn from_string(s: &str) -> Result<Self, String>
    where <T as std::str::FromStr>::Err: Debug
    {

        // let hdr_re = Regex::new(r"^Monkey (\d+):$").unwrap();
        let items_re = Regex::new(r"^Starting items: (.*)$").unwrap();
        let op_re = Regex::new(r"^Operation: new = old (.) (.*)$").unwrap();
        let test_re = Regex::new(r"^Test: divisible by (\d+)$").unwrap();
        let truedest_re = Regex::new(r"^If true: throw to monkey (\d+)$").unwrap();
        let falsedest_re = Regex::new(r"^If false: throw to monkey (\d+)$").unwrap();

        let mut operation = OpType::default();
        let mut operation_rhs = Option::None;
        let mut test_divisor: T = 0.try_into().unwrap();
        let mut destination_monkey = (0,0);
        let mut items = VecDeque::new();

        for line in s.split('\n') {
            // if let Some(caps) = hdr_re.captures(&line) {};
            if let Some(caps) = items_re.captures(line) {
                let itemstr = caps.get(1).unwrap().as_str();
                items = itemstr.split(", ").map(|x| x.parse::<T>().unwrap()).collect();
            };
            if let Some(caps) = op_re.captures(line) {
                let opstr = caps.get(1).unwrap().as_str();
                if opstr.eq("*") { operation = OpType::Multiply }
                else if opstr.eq("+") { operation = OpType::Add }
                else { panic!("Unsupported operation {}", opstr)}

                let t = caps.get(2).unwrap().as_str();
                operation_rhs = if t.eq("old") {
                    None
                } else {
                    Some(t.parse::<T>().unwrap())
                };
            };
            
            if let Some(caps) = test_re.captures(line) {
                test_divisor = caps.get(1).unwrap().as_str().parse::<T>().unwrap();
            };
            if let Some(caps) = truedest_re.captures(line) {
                destination_monkey.0 = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
            }
            if let Some(caps) = falsedest_re.captures(line) {
                destination_monkey.1 = caps.get(1).unwrap().as_str().parse::<u8>().unwrap();
            }
        }

        Ok(Self {
            operation_rhs,
            operation,
            test_divisor,
            items,
            destination_monkey,
            inspections: 0
        })
    }

    // Inspect the items we hold
    // return an Vec of tuples indicating items and their destination monkey.
    fn inspect(&mut self) -> Vec<(u8,T)> {

        let mut tosses = Vec::new();
        while let Some(mut item) = self.items.pop_front() {
     
            // print!("  Monkey inspects {}",item);
            self.inspections += 1;
            let rhs = match self.operation_rhs {
                None => item,
                Some(x) => x,
            };
            if self.operation == OpType::Add { item += rhs; }
            else if self.operation == OpType::Multiply {item *= rhs;}
            // print!(" new worry level {}",item);
            let three = T::try_from(3).ok().unwrap();

            item = item / three;
            // println!(" bored => level {}",item);
            let zero = T::try_from(0).ok().unwrap();
            if item % self.test_divisor == zero {
                // println!("  Is divisble by {}, throw to {}",self.test_divisor,self.destination_monkey.0);
                tosses.push((self.destination_monkey.0,item));
            } else {
                // println!("  Is not divisble by {}, throw to {}",self.test_divisor,self.destination_monkey.1);
                tosses.push((self.destination_monkey.1,item));
            }
        }
        tosses
    }

}

fn part1(input_filename: String) -> Result<u32, String> {
    let mut monkeys = Vec::new();

    let mut monkeylines = String::new();
    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            if line.is_empty() {
                let monkey: Monkey<u32> = Monkey::from_string(&monkeylines)?;
                monkeys.push(monkey);

            } else {
                monkeylines.push_str(line.trim());
                monkeylines.push('\n');
            }
        }
    }

    for _round in 1..=20 {
        // println!("== Round {} ==",_round);
        for i in 0..monkeys.len() {
            // println!("Monkey {}",i);
            let tosses = monkeys[i].inspect();
            for (throw_to,item) in tosses.iter() {
                monkeys[*throw_to as usize].items.push_back(*item);
            }
        }
    }
    for (i,monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} ends with: {:?} and inspected {} items",i,monkey.items,monkey.inspections);
    }

    // Return the product of the 2 most active primates...
    let mut bizness:Vec<u32> = monkeys.iter().map(|x| x.inspections).collect();
    bizness.sort();
    bizness.reverse();

    Ok(bizness[0]*bizness[1])
}
