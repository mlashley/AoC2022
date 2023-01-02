use std::collections::HashMap;
use std::time::Instant;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            10,
            false,
        ) == 110
    );

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            1000,
            true,
        ) == 20
    );
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Elf {
    current_pos: (i64, i64),
    proposed_pos: (i64, i64),
}

impl Elf {
    fn propose(&mut self, map: &HashMap<(i64, i64), Elf>, cycle: usize) -> Option<(i64, i64)> {
        let order = vec!['N', 'S', 'W', 'E', 'N', 'S', 'W'];
        debug_assert!(cycle < 4); // or we fall off.

        // Is anyone adjacent to us _at all_ ?
        for try_x in self.current_pos.0 - 1..=self.current_pos.0 + 1 {
            for try_y in self.current_pos.1 - 1..=self.current_pos.1 + 1 {
                if !(try_x == self.current_pos.0 && try_y == self.current_pos.1)
                    && map.contains_key(&(try_x, try_y))
                {
                    // Someone (else) is nearby - look in the directions in order
                    for dir in order.iter().skip(cycle).take(4) {
                        // println!("Elf at {:?} is looking {} [{}]",self.current_pos,order[i],i);
                        match dir {
                            'N' => {
                                if map.contains_key(&(self.current_pos.0, self.current_pos.1 - 1))
                                    || map.contains_key(&(
                                        self.current_pos.0 - 1,
                                        self.current_pos.1 - 1,
                                    ))
                                    || map.contains_key(&(
                                        self.current_pos.0 + 1,
                                        self.current_pos.1 - 1,
                                    ))
                                {
                                    continue;
                                } else {
                                    self.proposed_pos.0 = self.current_pos.0;
                                    self.proposed_pos.1 = self.current_pos.1 - 1;
                                    return Some(self.proposed_pos);
                                }
                            }
                            'S' => {
                                if map.contains_key(&(self.current_pos.0, self.current_pos.1 + 1))
                                    || map.contains_key(&(
                                        self.current_pos.0 - 1,
                                        self.current_pos.1 + 1,
                                    ))
                                    || map.contains_key(&(
                                        self.current_pos.0 + 1,
                                        self.current_pos.1 + 1,
                                    ))
                                {
                                    continue;
                                } else {
                                    self.proposed_pos.0 = self.current_pos.0;
                                    self.proposed_pos.1 = self.current_pos.1 + 1;
                                    return Some(self.proposed_pos);
                                }
                            }
                            'W' => {
                                if map
                                    .contains_key(&(self.current_pos.0 - 1, self.current_pos.1 + 1))
                                    || map
                                        .contains_key(&(self.current_pos.0 - 1, self.current_pos.1))
                                    || map.contains_key(&(
                                        self.current_pos.0 - 1,
                                        self.current_pos.1 - 1,
                                    ))
                                {
                                    continue;
                                } else {
                                    self.proposed_pos.0 = self.current_pos.0 - 1;
                                    self.proposed_pos.1 = self.current_pos.1;
                                    return Some(self.proposed_pos);
                                }
                            }
                            'E' => {
                                if map
                                    .contains_key(&(self.current_pos.0 + 1, self.current_pos.1 + 1))
                                    || map
                                        .contains_key(&(self.current_pos.0 + 1, self.current_pos.1))
                                    || map.contains_key(&(
                                        self.current_pos.0 + 1,
                                        self.current_pos.1 - 1,
                                    ))
                                {
                                    continue;
                                } else {
                                    self.proposed_pos.0 = self.current_pos.0 + 1;
                                    self.proposed_pos.1 = self.current_pos.1;
                                    return Some(self.proposed_pos);
                                }
                            }
                            _ => {
                                panic!("The compass is b0rk3d :(")
                            }
                        }
                    }
                    // No Moves - Fall thru and set proposed = current
                }
            }
        }
        self.proposed_pos = self.current_pos;
        None
    }
    fn new(current_pos: (i64, i64)) -> Self {
        Self {
            current_pos,
            proposed_pos: (0, 0),
        }
    }
}
fn print_map(map: &HashMap<(i64, i64), Elf>) -> u64 {
    let xmax = *map.keys().map(|(x, _)| x).max().unwrap();
    let xmin = *map.keys().map(|(x, _)| x).min().unwrap();
    let ymax = *map.keys().map(|(_, y)| y).max().unwrap();
    let ymin = *map.keys().map(|(_, y)| y).min().unwrap();

    let mut empty_count = 0;
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if map.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
                empty_count += 1;
            }
        }
        println!();
    }
    empty_count
}

fn part1(data: &str, rounds: usize, part2: bool) -> u64 {
    let mut elf_map: HashMap<(i64, i64), Elf> = HashMap::new();
    let mut new_elf_map: HashMap<(i64, i64), Elf> = HashMap::new();
    let mut proposals: HashMap<(i64, i64), u64> = HashMap::new();

    data.split('\n')
        .enumerate()
        .map(|(y, s)| {
            for (x, c) in s.chars().enumerate() {
                if c == '#' {
                    elf_map.insert((x as i64, y as i64), Elf::new((x as i64, y as i64)));
                    println!("New Elf at {}, {}", x, y);
                }
            }
        })
        .for_each(drop);

    for round in 0..rounds {
        let loc_map = elf_map.clone();
        new_elf_map.clear();
        // println!("{} Elves (check {})",elf_map.len(),loc_map.len());
        proposals.clear();
        for elf_loc in loc_map.keys() {
            if let Some(prop) = elf_map
                .get_mut(elf_loc)
                .unwrap()
                .propose(&loc_map, round % 4)
            {
                // println!("Elf at {:?} proposes {:?}", elf_loc,prop);
                proposals.entry(prop).and_modify(|e| *e += 1).or_insert(1);
            } else {
                // println!("Elf at {:?} is idle", elf_loc);
            }
        }
        // println!("{} Distinct Proposals: {:?}",proposals.len(),proposals);
        if part2 && proposals.is_empty() {
            return (round + 1) as u64;
        }

        for elf_loc in loc_map.keys() {
            let elf = elf_map.get_mut(elf_loc).unwrap();
            if elf.proposed_pos == elf.current_pos {
                // he didn't want to propose - we should have used an Option<(i64,i64)>
                // println!("Elf at {:?} is just staying put",elf.current_pos);
                new_elf_map.insert(elf.current_pos, Elf::new(elf.current_pos));
                continue;
            }

            if *proposals.get(&elf.proposed_pos).unwrap() == 1u64 {
                // Move
                // println!("Elf at {:?} moving to {:?}",elf.current_pos,elf.proposed_pos);
                new_elf_map.insert(elf.proposed_pos, Elf::new(elf.proposed_pos));
            } else {
                // println!("Elf at {:?} collides on {:?}, staying put",elf.current_pos,elf.proposed_pos);
                new_elf_map.insert(elf.current_pos, Elf::new(elf.current_pos));
            }
        }
        println!("Round {}", round);
        print_map(&new_elf_map);
        elf_map = new_elf_map.clone();
    }
    print_map(&new_elf_map)
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        10,
        false,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 4247);
    let p2 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        10000,
        true,
    );
    println!("Part2: {}", p2);
    assert!(p2 == 1049);
    println!("Completed in {} us", now.elapsed().as_micros());
}
