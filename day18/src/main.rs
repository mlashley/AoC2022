use parse_display::{Display, FromStr};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};
use std::vec;

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(r"{x},{y},{z}")]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}
impl Sub for Cube {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Cube {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

enum Faces {
    Right = 1,
    Left = 2,
    Top = 4,
    Bottom = 8,
    Front = 16,
    Back = 32,
}

impl Cube {
    fn touching(&self, other: &Self) -> bool {
        matches!(
            *self - *other,
            Cube { x: 1, y: 0, z: 0 }
                | Cube { x: -1, y: 0, z: 0 }
                | Cube { x: 0, y: 1, z: 0 }
                | Cube { x: 0, y: -1, z: 0 }
                | Cube { x: 0, y: 0, z: 1 }
                | Cube { x: 0, y: 0, z: -1 }
        )
    }
    fn touches_sides(&self, other: &Self) -> Option<u8> {
        match *other - *self {
            Cube { x: 1, y: 0, z: 0 } => Some(Faces::Right as u8),
            Cube { x: -1, y: 0, z: 0 } => Some(Faces::Left as u8),
            Cube { x: 0, y: 1, z: 0 } => Some(Faces::Top as u8),
            Cube { x: 0, y: -1, z: 0 } => Some(Faces::Bottom as u8),
            Cube { x: 0, y: 0, z: 1 } => Some(Faces::Back as u8),
            Cube { x: 0, y: 0, z: -1 } => Some(Faces::Front as u8),
            _ => None,
        }
    }
    fn in_bounds(&self, lower: &Self, upper: &Self) -> bool {
        self.x < lower.x
            || self.y < lower.y
            || self.z < lower.z
            || self.x > upper.x
            || self.y > upper.y
            || self.z > upper.z
    }
}

fn test() {
    let c1 = "1,1,1".parse::<Cube>().unwrap();
    let c2 = "1,2,1".parse::<Cube>().unwrap();
    let c3 = "0,1,0".parse::<Cube>().unwrap();
    println!("c2 {} - c1 {} = {:?}", c2, c1, c2 - c1,);
    debug_assert!(c1 != c2);
    debug_assert!(c2 - c1 == c3);
    debug_assert!(c2.touching(&c1));
    debug_assert!(c1.touching(&c2));
    debug_assert!(!c1.touching(&c3));
    debug_assert!(!c3.touching(&c1));

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 64
    );
    debug_assert!(
        part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 58
    );
}

fn part1(data: &str) -> usize {
    let mut cubes: Vec<Cube> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Cube>().unwrap())
        .collect();
    cubes.sort();

    let all_sides = cubes.len() * 6;

    let mut touching_sides = 0;
    cubes.iter().for_each(|i| {
        cubes.iter().for_each(|j| {
            if i.touching(j) {
                touching_sides += 1
            };
        })
    });

    println!(
        "Total Sides {} - Touching Sides {} == {}",
        all_sides,
        touching_sides,
        all_sides - touching_sides
    );
    all_sides - touching_sides
}

fn part2(data: &str) -> u32 {
    let cubes: Vec<Cube> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Cube>().unwrap())
        .collect();

    // We're going to bound a volume (xmin-1,ymin-1,zmin-1)-(xmax+1,ymax+1,zmax+1)
    // Starting a single cube in the corner iterate thru all of them seeing where we can move..
    // Touching edges until we fill the space

    let wiggle = 2;

    let xmin = cubes.iter().map(|c| c.x).min().unwrap() - wiggle;
    let xmax = cubes.iter().map(|c| c.x).max().unwrap() + wiggle;
    let ymin = cubes.iter().map(|c| c.y).min().unwrap() - wiggle;
    let ymax = cubes.iter().map(|c| c.y).max().unwrap() + wiggle;
    let zmin = cubes.iter().map(|c| c.z).min().unwrap() - wiggle;
    let zmax = cubes.iter().map(|c| c.z).max().unwrap() + wiggle;

    let lowerbound = Cube {
        x: xmin,
        y: ymin,
        z: zmin,
    };
    let upperbound = Cube {
        x: xmax,
        y: ymax,
        z: zmax,
    };

    // using a bitmask of each side.
    let mut sides_touched: HashMap<Cube, u8> = cubes.iter().map(|x| (*x, 0)).collect();
    assert!(sides_touched.keys().len() == cubes.len());
    let mut visited: HashSet<Cube> = HashSet::new();

    let start = Cube {
        x: xmin,
        y: ymin,
        z: zmin,
    };
    let mut q: VecDeque<Cube> = std::iter::once(start).collect();

    let mut guard = (1 + xmax - xmin) * (1 + ymax - ymin) * (1 + zmax - zmin);

    // This may look a bunch like some code in Day12...
    while let Some(c) = q.pop_front() {
        if !visited.insert(c) {
            // Already visited
            continue;
        }
        if c.in_bounds(&lowerbound, &upperbound) {
            continue;
        }
        guard -= 1;
        assert!(guard > 0);

        // Test if this position touches any cube in the object
        for other in cubes.iter() {
            if let Some(r) = other.touches_sides(&c) {
                sides_touched.entry(*other).and_modify(|mask| *mask |= r);
            }
        }

        // Figure out next positions (very verbosely...)

        let left = c + Cube { x: -1, y: 0, z: 0 };
        let right = c + Cube { x: 1, y: 0, z: 0 };
        let up = c + Cube { x: 0, y: 1, z: 0 };
        let down = c + Cube { x: 0, y: -1, z: 0 };
        let back = c + Cube { x: 0, y: 0, z: 1 };
        let front = c + Cube { x: 0, y: 0, z: -1 };

        let possible = vec![left, right, up, down, back, front];

        for p in possible {
            if !cubes.iter().any(|x| *x == p) {
                q.push_back(p);
            }
        }
    }

    let outsides: u32 = sides_touched.iter().map(|(_, v)| v.count_ones()).sum();

    outsides
}

// This was naive, (at least after I rendered the shape in blender) but it may be useful at some point.
#[allow(dead_code)]
fn part2_failed_attempt(data: &str) -> u32 {
    let mut cubes: Vec<Cube> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Cube>().unwrap())
        .collect();

    // using a bitmask of each side.
    let mut sides_touched: HashMap<Cube, u8> = cubes.iter().map(|x| (*x, 0)).collect();

    cubes.sort();

    let xmin = cubes.iter().map(|c| c.x).min().unwrap();
    let xmax = cubes.iter().map(|c| c.x).max().unwrap();
    let ymin = cubes.iter().map(|c| c.y).min().unwrap();
    let ymax = cubes.iter().map(|c| c.y).max().unwrap();
    let zmin = cubes.iter().map(|c| c.z).min().unwrap();
    let zmax = cubes.iter().map(|c| c.z).max().unwrap();

    println!(
        "min {},{},{} max {},{},{}",
        xmin, ymin, zmin, xmax, ymax, zmax
    );

    // We're going to project a rectangle of cubes along each axis, and find the external sides.

    // project( along=z from zmin -2 .. zmax+2 or collisiaion , xmin:max,ymin:max )
    for x in xmin - 1..=xmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            // Loop along z in positive direction until we touch
            'nextcube: for z in zmin - 2..=zmax + 2 {
                let mut should_break = false;
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Front as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    // project:  along=z in reverse direction
    for x in xmin - 1..=xmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            // Loop along z in negative direction until we touch
            'nextcube: for z in (zmin - 2..=zmax + 2).rev() {
                let mut should_break = false;
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Back as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    // project( along=x from xmin -2 .. xmax+2 or collisiaion , zmin:zmax,ymin:ymax )
    for z in zmin - 1..=zmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            'nextcube: for x in xmin - 2..=xmax + 2 {
                let mut should_break = false;
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Left as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    // project:  along=x in reverse direction
    for z in zmin - 1..=zmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            'nextcube: for x in (xmin - 2..=xmax + 2).rev() {
                let testcube = Cube { x, y, z };
                let mut should_break = false;
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Right as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    // project( along=y from ymin -2 .. ymax+2 or collisiaion , zmin:zmax,xmin:xmax )
    for z in zmin - 1..=zmax + 1 {
        for x in xmin - 1..=xmax + 1 {
            'nextcube: for y in ymin - 2..=ymax + 2 {
                let mut should_break = false;
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        println!("tgt {} test {} r {}", other, testcube, r);
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Bottom as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    // project:  along=x in reverse direction
    for z in zmin - 1..=zmax + 1 {
        for x in xmin - 1..=xmax + 1 {
            'nextcube: for y in (ymin - 2..=ymax + 2).rev() {
                let mut should_break = false;
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        println!("tgt {} test {} r {}", other, testcube, r);
                        sides_touched.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Top as u8 {
                            should_break = true;
                        }
                    };
                }
                if should_break {
                    break 'nextcube;
                }
            }
        }
    }

    println!("{:?}", sides_touched);
    let outsides: u32 = sides_touched.iter().map(|(_, v)| v.count_ones()).sum();
    println!("Outsides: {}", outsides);

    outsides
}

fn main() {
    test();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 3576);
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 == 2066); // 1882 is too low.     // 2031 is too low.
}
