use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::cmp;
use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Debug, PartialEq)]
#[display(r"Valve {name} has flow rate={rate}; tunnels lead to valves {valve_list}")]
struct Valve {
    name: String,
    rate: u32,
    valve_list: String,
    #[from_str(default)]
    bitmask: u64,
}

impl Valve {
    fn adjacent_list(&self) -> std::str::Split<'_, &str> {
        self.valve_list.split(", ")
    }
}

fn test() {
    debug_assert!(
        "Valve UL has flow rate=0; tunnels lead to valves EC, AA"
            .parse::<Valve>()
            .unwrap()
            == Valve {
                name: "UL".into(),
                rate: 0,
                valve_list: "EC, AA".into(),
                bitmask: 0,
            }
    );
    let a = "Valve UL has flow rate=0; tunnels lead to valves EC, AA"
        .parse::<Valve>()
        .unwrap();
    debug_assert!(a.adjacent_list().collect::<Vec<&str>>() == vec!["EC", "AA"]);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            false
        ) == 1651
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            true
        ) == 1707
    );
}

fn part1(data: &str, is_part2: bool) -> i64 {
    let mut i = 0;
    let valves: HashMap<String, Valve> = data
        .replace("valve ", "valves ") // makes our parser easier if we don't care about singular/plural
        .replace("tunnel ", "tunnels ")
        .replace("leads ", "lead ")
        .split('\n')
        .filter(|y| !y.is_empty())
        // .inspect(|x| println!("{}", x))
        .map(|x| x.parse::<Valve>().unwrap())
        .map(|mut x| {
            x.bitmask = u64::pow(2, i);
            i += 1;
            x
        })
        .map(|x| (x.name.clone(), x))
        .collect();

    // println!("{:?}", valves);

    let mut dist: HashMap<(String, String), i64> = HashMap::new();

    // Floyd Warshall
    valves.keys().for_each(|a| {
        valves.keys().for_each(|b| {
            if valves
                .get(a)
                .unwrap()
                .adjacent_list()
                .any(|x| x == b.as_str())
            // .collect::<Vec<&str>>()
            // .contains(&b.as_str())
            {
                dist.entry((a.clone(), b.clone())).or_insert(1);
            } else if a == b {
                dist.entry((a.clone(), b.clone())).or_insert(0);
            } else {
                dist.entry((a.clone(), b.clone())).or_insert(i64::MAX);
            }
        });
    });

    valves.keys().for_each(|k| {
        valves.keys().for_each(|i| {
            valves.keys().for_each(|j| {
                let ij = *dist.get(&(i.clone(), j.clone())).unwrap();
                let ik = *dist.get(&(i.clone(), k.clone())).unwrap();
                let kj = *dist.get(&(k.clone(), j.clone())).unwrap();
                // the conditional is because we overflow if we compute ixxMAX+ixxMAX...
                dist.insert(
                    (i.clone(), j.clone()),
                    cmp::min(
                        ij,
                        if ik == i64::MAX || kj == i64::MAX {
                            i64::MAX
                        } else {
                            ik + kj
                        },
                    ),
                );
            })
        })
    });

    if !is_part2 && valves.len() < 15 {
        // Print distance map
        // Header
        print!("    ");
        valves.keys().sorted().for_each(|x| print!("{:>2} ", x));
        println!();
        valves.keys().sorted().for_each(|i| {
            print!("{}: ", i);
            valves.keys().sorted().for_each(|j| {
                let val = *dist.get(&(i.clone(), j.clone())).unwrap();
                if val == i64::MAX {
                    print!("{:>2}", 'x');
                } else {
                    print!("{:>2} ", val);
                }
            });
            println!();
        });
    }

    let state: u64 = 0;
    let mut answer: HashMap<u64, i64> = HashMap::new();
    let final_answer = visit(
        String::from("AA"),
        if is_part2 { 26 } else { 30 },
        state,
        &valves,
        &dist,
        0,
        &mut answer,
    );

    // The best possible outcome for us and Nelly T. Elephant combined is
    // the 2 non-overlapping solutions with the highest combined scores.
    if is_part2 {
        let mut total = 0;
        for (key1, val1) in final_answer.iter() {
            for (key2, val2) in final_answer.iter() {
                if key1 & key2 == 0 {
                    // No overlap in these solutions }
                    if val1 + val2 > total {
                        total = val1 + val2;
                    }
                }
            }
        }
        total
    } else {
        let total = *final_answer.values().max().unwrap();
        total
    }
}

fn visit<'a>(
    valve: String,
    time_budget: i64,
    state: u64,
    valves: &HashMap<String, Valve>,
    distmap: &HashMap<(String, String), i64>,
    flow: i64,
    answer: &'a mut HashMap<u64, i64>,
) -> &'a mut HashMap<u64, i64> {
    // println!("visit: {valve} flow: {flow} state: {state} budget: {budget} answer: {answer:?}");

    let n: i64 = if !answer.contains_key(&state) {
        0
    } else {
        *answer.get(&state).unwrap()
    };
    answer.insert(state, cmp::max(n, flow));
    for candidate_valve in valves
        .iter()
        .filter(|(_, cv)| cv.rate > 0)
        .map(|(ck, _)| ck)
    {
        let dist = *distmap
            .get(&(valve.clone(), candidate_valve.clone()))
            .unwrap();
        let remaining_time = time_budget as i64 - dist - 1;
        let mask = valves.get(candidate_valve).unwrap().bitmask;
        if (state & mask) != 0 || remaining_time < 0 {
            // No point in revisiting a valve we already turned on, or one that exceeds the given time.
            continue;
        } else {
            let flow_adjacent = valves.get(candidate_valve).unwrap().rate as i64;
            let _ = visit(
                candidate_valve.clone(),
                remaining_time,
                state | mask,
                valves,
                distmap,
                flow + (remaining_time * flow_adjacent),
                answer,
            );
        }
    }
    answer
}

fn main() {
    test();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        false,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 1880);
    let p2 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), true);
    println!("Part2: {}", p2);
    assert!(p2 == 2520);
}

// See also.
// https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm
