use std::collections::{HashMap, HashSet, VecDeque};
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
    // Part 1
    debug_assert!(part1("./input_sample.txt".into()) == 31);
    let part1_result = part1("./input.txt".into());
    println!("Part1: {}", part1_result);
    debug_assert!(part1_result == 456); // Keep part 1 working.

    // Part2
    debug_assert!(part2("./input_sample.txt".into()) == 29);
    let part2_result = part2("./input.txt".into());
    println!("Part2: {}", part2_result);
    debug_assert!(part2_result == 454); // Keep part 2 working.
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn part1(input_filename: String) -> usize {
    let mut mymap = Vec::new();
    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            mymap.push(line.as_str().chars().collect::<Vec<char>>());
        }
    }
    // println!("{:?}",mymap);
    let start = match find_start(&mymap) {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };
    let end = match find_end(&mymap) {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };
    mymap[end.y][end.x] = 'z';
    mymap[start.y][start.x] = 'a';

    solve(&mymap, start, end)
}

fn part2(input_filename: String) -> usize {
    let mut mymap = Vec::new();
    if let Ok(lines) = read_lines(input_filename) {
        for line in lines.flatten() {
            mymap.push(line.as_str().chars().collect::<Vec<char>>());
        }
    }
    let start = match find_start(&mymap) {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };
    let end = match find_end(&mymap) {
        Ok(x) => x,
        Err(y) => panic!("{}", y),
    };
    mymap[end.y][end.x] = 'z';
    mymap[start.y][start.x] = 'a';

    let starts = find_all(&mymap, 'a');
    println!("Need to solve for {} starting positions...", starts.len());
    // There is probably a smarter way to do this - but I'm a day behind and anyway:
    //
    // $ time  cargo rr
    // Finished release [optimized] target(s) in 0.00s
    // Running `target/release/day12`
    // Part1: 456
    // Need to solve for 1838 starting positions...
    // Part2: 454

    // real    0m0.330s
    // user    0m0.318s
    // sys     0m0.012s

    let mut lengths = Vec::new();
    for s in starts {
        lengths.push(solve(&mymap, s, end));
    }
    lengths.sort();
    // println!("lengths: {:?}",lengths);
    match lengths.first() {
        Some(x) => *x,
        None => usize::MAX,
    }
}

fn solve(mymap: &Vec<Vec<char>>, start: Coord, end: Coord) -> usize {
    // println!("Start: {:?} End: {:?}", start, end);

    let mut visited: HashSet<Coord> = HashSet::new();
    let mut q: VecDeque<Coord> = std::iter::once(start).collect();
    let mut d: HashMap<Coord, usize> = std::iter::once((start, 0)).collect();

    while let Some(c) = q.pop_front() {
        // println!("Trying {:?}",c);
        if !visited.insert(c) {
            // println!("Already visited {:?}",c);
            continue;
        }
        if c == end {
            break;
        }

        let cheight: u32 = mymap[c.y][c.x].into();
        for possibility in possible(mymap, c) {
            // println!("  Possible: {:?} ",possibility);
            let nheight: u32 = mymap[possibility.y][possibility.x].into();
            if nheight <= cheight + 1 {
                // println!("    {:?} is a possibility ({},{})",possibility,nheight,cheight);
                let cdist = d[&c];
                let ndist = d.get(&possibility).unwrap_or(&usize::MAX);
                if cdist < *ndist {
                    // println!("      Pushing poss: {:?} dist: {}",possibility,cdist+1);
                    d.insert(possibility, cdist + 1);
                    q.push_back(possibility);
                } else {
                    // println!("      Too Far {} >= {}",cdist,*ndist);
                }
            } else {
                // println!("    {:?} is NOT a possibility ({},{})",possibility,nheight,cheight);
            }
        }
    }
    if d.contains_key(&end) {
        d[&end]
    } else {
        usize::MAX
    }
}

fn find_all(m: &[Vec<char>], c: char) -> Vec<Coord> {
    let mut v = Vec::new();
    for (y, row) in m.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == c {
                v.push(Coord::new(x, y));
            }
        }
    }
    v
}

fn find_thing(m: &[Vec<char>], c: char) -> Result<Coord, String> {
    for (y, row) in m.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == c {
                return Ok(Coord::new(x, y));
            }
        }
    }
    Err("Fuck".into())
}

fn find_start(m: &[Vec<char>]) -> Result<Coord, String> {
    find_thing(m, 'S')
}

fn find_end(m: &[Vec<char>]) -> Result<Coord, String> {
    find_thing(m, 'E')
}

fn possible(m: &Vec<Vec<char>>, c: Coord) -> Vec<Coord> {
    let mut v = Vec::with_capacity(4);

    let height = m.len();
    let width = m[0].len();

    if c.x > 0 {
        v.push(Coord { x: c.x - 1, y: c.y })
    }
    if c.y > 0 {
        v.push(Coord { x: c.x, y: c.y - 1 })
    }
    if c.x < width - 1 {
        v.push(Coord { x: c.x + 1, y: c.y })
    }
    if c.y < height - 1 {
        v.push(Coord { x: c.x, y: c.y + 1 })
    }
    // println!("Adjacent to {:?} is {:?}",c,v);
    v
}
