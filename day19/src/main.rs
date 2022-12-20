use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::fmt::Error;
use std::time::Instant;

use divrem::DivCeil;
use rayon::prelude::*;

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[display(r"Blueprint {id}: Each ore robot costs {ore_robot_cost_in_ore} ore. Each clay robot costs {clay_robot_cost_in_ore} ore. Each obsidian robot costs {obsidian_robot_cost_in_ore} ore and {obsidian_robot_cost_in_clay} clay. Each geode robot costs {geode_robot_cost_in_ore} ore and {geode_robot_cost_in_obsidian} obsidian.")]
struct Blueprint {
    id: u32,
    ore_robot_cost_in_ore: u32,
    clay_robot_cost_in_ore: u32,
    obsidian_robot_cost_in_ore: u32,
    obsidian_robot_cost_in_clay: u32,
    geode_robot_cost_in_ore: u32,
    geode_robot_cost_in_obsidian: u32,
    #[from_str(default)]
    max_ore_cost: u32,
}

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

#[derive(Debug, Display, PartialEq, Eq, Clone, Default, Hash)] // Display
#[display(
    r"trem={time_remaining} Ore,Clay,Obs,Geo: {ore},{clay},{obsidian},{geode} bots: {bots:?}"
)]
struct State {
    time_remaining: u32,
    bots: [u32; 4], // Indexed by above const's, we could get fancy with Vec etc, but something easily hashable is prefereble.
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32, // This is technically unneeded as we count them for t-remaining once we generate the bot, but easier for debugging.
}

impl Blueprint {
    fn enhance(&mut self) {
        self.max_ore_cost = self
            .ore_robot_cost_in_ore
            .max(self.clay_robot_cost_in_ore)
            .max(self.obsidian_robot_cost_in_ore)
            .max(self.geode_robot_cost_in_ore);
    }
}

impl State {
    fn new(time_remaining: u32) -> State {
        State {
            bots: [1, 0, 0, 0],
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            time_remaining,
        }
    }

    // Calculate robot production for some time
    fn run_production(self, minutes: u32) -> State {
        State {
            ore: self.ore + (self.bots[ORE] * minutes),
            clay: self.clay + (self.bots[CLAY] * minutes),
            obsidian: self.obsidian + (self.bots[OBSIDIAN] * minutes),
            geode: self.geode + (self.bots[GEODE] * minutes),
            time_remaining: self.time_remaining - minutes,
            ..self
        }
    }

    fn build_ore_bot(self, blueprint: &Blueprint) -> State {
        State {
            ore: self.ore - blueprint.ore_robot_cost_in_ore,
            bots: [
                self.bots[ORE] + 1,
                self.bots[CLAY],
                self.bots[OBSIDIAN],
                self.bots[GEODE],
            ],
            ..self
        }
    }

    fn build_clay_bot(self, blueprint: &Blueprint) -> State {
        State {
            ore: self.ore - blueprint.clay_robot_cost_in_ore,
            bots: [
                self.bots[ORE],
                self.bots[CLAY] + 1,
                self.bots[OBSIDIAN],
                self.bots[GEODE],
            ],
            ..self
        }
    }

    fn build_obsidian_bot(self, blueprint: &Blueprint) -> State {
        State {
            ore: self.ore - blueprint.obsidian_robot_cost_in_ore,
            clay: self.clay - blueprint.obsidian_robot_cost_in_clay,
            bots: [
                self.bots[ORE],
                self.bots[CLAY],
                self.bots[OBSIDIAN] + 1,
                self.bots[GEODE],
            ],
            ..self
        }
    }

    fn build_geode_bot(self, blueprint: &Blueprint) -> State {
        State {
            ore: self.ore - blueprint.geode_robot_cost_in_ore,
            obsidian: self.obsidian - blueprint.geode_robot_cost_in_obsidian,
            bots: [
                self.bots[ORE],
                self.bots[CLAY],
                self.bots[OBSIDIAN],
                self.bots[GEODE] + 1,
            ],
            ..self
        }
    }

    // Calculate times to completeion of a given bot. Either 1 if we have the resources already, or rounded up based on forecast generation of that resource.

    #[allow(unstable_name_collisions)]
    fn time_to_ore_bot(&self, blueprint: &Blueprint) -> u32 {
        if self.ore >= blueprint.ore_robot_cost_in_ore {
            1
        } else {
            (blueprint.ore_robot_cost_in_ore - self.ore).div_ceil(self.bots[ORE]) + 1
        }
    }

    #[allow(unstable_name_collisions)]
    fn time_to_clay_bot(&self, blueprint: &Blueprint) -> u32 {
        if self.ore >= blueprint.clay_robot_cost_in_ore {
            1
        } else {
            (blueprint.clay_robot_cost_in_ore - self.ore).div_ceil(self.bots[ORE]) + 1
        }
    }

    #[allow(unstable_name_collisions)]
    fn time_to_obsidian_bot(&self, blueprint: &Blueprint) -> Option<u32> {
        if self.bots[CLAY] == 0 {
            None
        } else {
            let ore_days = if self.ore >= blueprint.obsidian_robot_cost_in_ore {
                1
            } else {
                (blueprint.obsidian_robot_cost_in_ore - self.ore).div_ceil(self.bots[ORE]) + 1
            };
            let clay_days = if self.clay >= blueprint.obsidian_robot_cost_in_clay {
                1
            } else {
                (blueprint.obsidian_robot_cost_in_clay - self.clay).div_ceil(self.bots[CLAY]) + 1
            };

            Some(ore_days.max(clay_days))
        }
    }

    #[allow(unstable_name_collisions)]
    fn time_to_geode_bot(&self, blueprint: &Blueprint) -> Option<u32> {
        if self.bots[OBSIDIAN] == 0 {
            None
        } else {
            let ore_days = if self.ore >= blueprint.geode_robot_cost_in_ore {
                1
            } else {
                (blueprint.geode_robot_cost_in_ore - self.ore).div_ceil(self.bots[ORE]) + 1
            };
            let obsidian_days = if self.obsidian >= blueprint.geode_robot_cost_in_obsidian {
                1
            } else {
                (blueprint.geode_robot_cost_in_obsidian - self.obsidian)
                    .div_ceil(self.bots[OBSIDIAN])
                    + 1
            };

            Some(ore_days.max(obsidian_days))
        }
    }
}

pub fn main() -> Result<(), Error> {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 1413);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 21080);
    println!("Completed in {} us", now.elapsed().as_micros());
    Ok(())
}
fn test() {
    let _bp = std::fs::read_to_string("input_sample.txt")
        .unwrap()
        .as_str()
        .split('\n')
        .take(1)
        .collect::<String>()
        .parse::<Blueprint>()
        .unwrap();

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 33
    );

    println!("This should be 56 and 62, respectively");
    assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 56 * 62
    );
    println!("======== END TESTS ===========");
}

fn part1(data: &str) -> u64 {
    let blueprints: Vec<Blueprint> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        // .inspect((|x| println!("{}",x)))
        .map(|x| x.parse::<Blueprint>().unwrap())
        .map(|mut bp| {
            bp.enhance();
            bp
        })
        .collect();

    blueprints
        .par_iter()
        .enumerate()
        .map(|(idx, d)| {
            let state: State = State::new(24);
            let mut cache: HashMap<State, u32> = Default::default();
            let mut best: u32 = 0;

            let score = get_best_score(state, d, 0, &mut best, &mut cache) as u64;
            println!("Best for BP {} is {}", idx + 1, score);
            score * (idx + 1) as u64
        })
        .sum()
}

fn part2(data: &str) -> u64 {
    let blueprints: Vec<Blueprint> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        // .inspect((|x| println!("{}",x)))
        .map(|x| x.parse::<Blueprint>().unwrap())
        .map(|mut bp| {
            bp.enhance();
            bp
        })
        .collect();

    blueprints
        .par_iter()
        .take(3)
        .enumerate()
        .map(|(idx, d)| {
            let state: State = State::new(32);
            let mut cache: HashMap<State, u32> = Default::default();
            let mut best: u32 = 0;
            let score = get_best_score(state, d, 0, &mut best, &mut cache) as u64;
            println!("Best for BP {} is {}", idx + 1, score);
            score
        })
        .product()
}

fn get_best_score(
    state: State,
    blueprint: &Blueprint,
    current: u32,
    best: &mut u32,
    cache: &mut HashMap<State, u32>,
) -> u32 {
    if state.time_remaining <= 1 {
        return 0;
    }

    // Memoization
    if let Some(memory) = cache.get(&state) {
        return *memory;
    }

    // Trying to prune if we assumed all new Geode bots from here on, can that beat the current best
    // Works - but not convinced by it - some speedup, but we are already 25ms...

    // I take it back - really helps with part2, convinced myself it is legit.
    let best_remaining_score = (state.time_remaining) * (state.time_remaining + 1) / 2;
    if current + best_remaining_score <= *best {
        return 0;
    }

    // println!("State: {}", state);

    let mut my_best = 0u32;

    // Can we build a Geode Bot?
    if let Some(minutes) = state.time_to_geode_bot(blueprint) {
        if minutes < state.time_remaining {
            let next_state = state
                .clone()
                .run_production(minutes)
                .build_geode_bot(blueprint);
            let new_added_score = next_state.time_remaining;
            let next_score = get_best_score(
                next_state,
                blueprint,
                current + new_added_score,
                best,
                cache,
            ) + new_added_score;
            my_best = next_score.max(my_best)
        }
    }

    // Do we wait to build an Obsididan Bot? Would it actually help (don't branch at end...)
    if state.bots[OBSIDIAN] < blueprint.geode_robot_cost_in_obsidian && state.time_remaining >= 3 {
        if let Some(minutes) = state.time_to_obsidian_bot(blueprint) {
            if minutes < state.time_remaining {
                let next_state = state
                    .clone()
                    .run_production(minutes)
                    .build_obsidian_bot(blueprint);
                let next_score = get_best_score(next_state, blueprint, current, best, cache);
                my_best = next_score.max(my_best)
            }
        }
    }

    // Do we wait to build a Clay Bot? Should we...
    if state.bots[CLAY] < blueprint.obsidian_robot_cost_in_clay && state.time_remaining >= 4 {
        let minutes = state.time_to_clay_bot(blueprint);
        if minutes < state.time_remaining {
            let next_state = state
                .clone()
                .run_production(minutes)
                .build_clay_bot(blueprint);
            let next_score = get_best_score(next_state, blueprint, current, best, cache);
            my_best = next_score.max(my_best)
        }
    }

    // Do we wait to build an Ore Bot? Should we (no point if we have enough bots to generate the max-ore-cost every cycle)
    if state.bots[ORE] < blueprint.max_ore_cost {
        let minutes = state.time_to_ore_bot(blueprint);
        if minutes < state.time_remaining {
            let next_state = state
                .clone()
                .run_production(minutes)
                .build_ore_bot(blueprint);
            let next_score = get_best_score(next_state, blueprint, current, best, cache);
            my_best = next_score.max(my_best)
        }
    }

    cache.insert(state, my_best);
    let curr_best = my_best + current;
    if curr_best > *best {
        *best = curr_best
    };

    my_best
}
