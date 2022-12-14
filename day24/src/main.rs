// use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::time::Instant;

fn test() {
    let mut player = Valley::from_str(
        std::fs::read_to_string("input_sample2.txt")
            .unwrap()
            .as_str(),
    );

    println!("PLAYER: {:?}", player);

    player.gen_bad_map(10);
    for i in 0..8 {
        player.print_map(i);
        // player.move_blizzards();
    }

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            false,
        ) == 18
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            true
        ) == 54
    );
}

#[derive(Debug, Clone, Default)]
struct Blizzard {
    x: usize,
    y: usize,
    facing: char,
}

#[derive(Debug, Clone, Default)]
struct Valley {
    blizzards: Vec<Blizzard>,
    bad_map: Vec<Vec<Vec<bool>>>,
    map_width: usize,
    map_height: usize,
}

impl Valley {
    fn from_str(data: &str) -> Self {
        let map_width = data
            .split('\n')
            .filter(|x| x.contains('#'))
            .map(|x| x.len())
            .max()
            .unwrap();
        let map_height = data.split('\n').filter(|x| x.contains('#')).count();

        println!("w x h = {} x {}", map_width, map_height);

        let blizzards: Vec<Blizzard> = data
            .split('\n')
            .enumerate()
            .flat_map(|(y, r)| {
                r.chars()
                    .enumerate()
                    .filter(|(_, facing)| "<^v>".contains(*facing))
                    .map(|(x, facing)| Blizzard { x, y, facing })
                    .collect::<Vec<Blizzard>>()
            })
            .collect::<Vec<Blizzard>>();

        let bad_map = Vec::new();

        Valley {
            blizzards,
            bad_map,
            map_width,
            map_height,
        }
    }

    fn gen_bad_map(&mut self, iterations: usize) {
        for i in 0..iterations {
            self.bad_map.push(Vec::new());
            // Walls
            for y in 0..self.map_height {
                let mut row = Vec::new();
                for x in 0..self.map_width {
                    if x == 0
                        || x == self.map_width - 1
                        || (y == 0 && x != 1)
                        || (y == self.map_height - 1 && x != self.map_width - 2)
                    {
                        row.push(true);
                    } else {
                        row.push(false);
                    }
                }
                self.bad_map[i].push(row);
            }
            for bliz in self.blizzards.iter() {
                self.bad_map[i][bliz.y][bliz.x] = true;
            }
            self.move_blizzards();
        }
    }

    fn move_blizzards(&mut self) {
        for bliz in self.blizzards.iter_mut() {
            match bliz.facing {
                '<' => {
                    if bliz.x == 1 {
                        bliz.x = self.map_width - 1
                    };
                    bliz.x -= 1;
                }
                '>' => {
                    if bliz.x == self.map_width - 2 {
                        bliz.x = 0
                    };
                    bliz.x += 1;
                }
                '^' => {
                    if bliz.y == 1 {
                        bliz.y = self.map_height - 1
                    };
                    bliz.y -= 1;
                }
                'v' => {
                    if bliz.y == self.map_height - 2 {
                        bliz.y = 0
                    };
                    bliz.y += 1;
                }
                _ => {
                    panic!("What frozen hell is this?")
                }
            }
        }
    }

    fn print_map(&self, t: usize) {
        println!("Map for t={}", t);
        for y in 0..self.bad_map[0].len() {
            for x in 0..self.bad_map[0][y].len() {
                if self.bad_map[t][y][x] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Copy)]
struct State {
    x: usize,
    y: usize,
    time: usize,
    end_visited: bool,
    start_visited: bool,
}

fn part1(data: &str, is_part2: bool) -> usize {
    let mut player = Valley::from_str(data);

    player.gen_bad_map(1000);

    let state = State {
        x: 1,
        y: 0,
        time: 0,
        end_visited: !is_part2,
        start_visited: !is_part2,
    };

    let mut q = VecDeque::new();
    let mut seen: HashSet<State> = HashSet::new();
    q.push_back(state);
    let mut best = usize::MAX;

    while let Some(curr_state) = q.pop_front() {
        if seen.contains(&curr_state) {
            continue;
        }
        seen.insert(curr_state);

        if best < curr_state.time {
            // prune
            // println!("Prune: {}, {:?}",best,curr_state);
            continue;
        }

        let mut end_visited = curr_state.end_visited;
        let mut start_visited = curr_state.start_visited;

        if curr_state.y == player.map_height - 1
            && curr_state.x == player.map_width - 2
            && !curr_state.end_visited
        {
            end_visited = true;
        }
        if curr_state.y == 0
            && curr_state.x == 1
            && curr_state.end_visited
            && !curr_state.start_visited
        {
            start_visited = true;
        }

        if curr_state.y == player.map_height - 1
            && curr_state.x == player.map_width - 2
            && curr_state.end_visited
            && curr_state.start_visited
        {
            println!("Goal {:?}", curr_state);
            if curr_state.time < best {
                best = curr_state.time;
            }
            continue;
        }

        let x = curr_state.x;
        let y = curr_state.y;
        let t = curr_state.time;

        if player.bad_map[t + 1][y][x] {
            // A storm is coming, cannot stay put
        } else {
            q.push_back(State {
                x,
                y,
                time: t + 1,
                end_visited,
                start_visited,
            });
        }

        // South
        if y < player.map_height - 1 {
            if player.bad_map[t + 1][y + 1][x] {
                // A storm/wall will be there - cannot move
            } else {
                q.push_back(State {
                    x,
                    y: y + 1,
                    time: t + 1,
                    end_visited,
                    start_visited,
                });
            }
        }
        // North
        if y > 0 && player.bad_map[t + 1][y - 1][x] {
            // A storm/wall will be there - cannot move
        } else if y > 0 {
            q.push_back(State {
                x,
                y: y - 1,
                time: t + 1,
                end_visited,
                start_visited,
            });
        }

        // East
        if player.bad_map[t + 1][y][x + 1] {
            // A storm/wall will be there - cannot move
        } else {
            q.push_back(State {
                x: x + 1,
                y,
                time: curr_state.time + 1,
                end_visited,
                start_visited,
            });
        }

        // West
        if x > 0 && player.bad_map[t + 1][y][x - 1] {
            // A storm/wall will be there - cannot move
        } else if x > 0 {
            q.push_back(State {
                x: x - 1,
                y,
                time: curr_state.time + 1,
                end_visited,
                start_visited,
            });
        }
    }

    best
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        false,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 334);
    let p2 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), true);
    println!("Part2: {}", p2);
    assert!(p2 == 934);
    println!("Completed in {} us", now.elapsed().as_micros());
}
