// use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::ops::Sub;

// use std::cmp;
// use std::collections::HashMap;

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
    let mut cubes: Vec<Cube> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<Cube>().unwrap())
        .collect();

    // using a bitmask of each side.
    let mut sidesTouchedMap: HashMap<Cube, u8> = cubes.iter().map(|x| (*x, 0)).collect();

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
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {                        
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Front as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    // project:  along=z in reverse direction
    for x in xmin - 1..=xmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            // Loop along z in negative direction until we touch
            'nextcube: for z in (zmin - 2..=zmax + 2).rev() {
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Back as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    // project( along=x from xmin -2 .. xmax+2 or collisiaion , zmin:zmax,ymin:ymax )
    for z in zmin - 1..=zmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            'nextcube: for x in xmin - 2..=xmax + 2 {
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {  
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Left as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    // project:  along=x in reverse direction
    for z in zmin - 1..=zmax + 1 {
        for y in ymin - 1..=ymax + 1 {
            'nextcube: for x in (xmin - 2..=xmax + 2).rev() {
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) { 
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Right as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    // project( along=y from ymin -2 .. ymax+2 or collisiaion , zmin:zmax,xmin:xmax )
    for z in zmin - 1..=zmax + 1 {
        for x in xmin - 1..=xmax + 1 {
            'nextcube: for y in ymin - 2..=ymax + 2 {
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        println!("tgt {} test {} r {}", other, testcube, r);
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Bottom as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    // project:  along=x in reverse direction
    for z in zmin - 1..=zmax + 1 {
        for x in xmin - 1..=xmax + 1 {

            'nextcube: for y in (ymin - 2..=ymax + 2).rev() {
                let testcube = Cube { x, y, z };
                for other in cubes.iter() {
                    if let Some(r) = other.touches_sides(&testcube) {
                        println!("tgt {} test {} r {}", other, testcube, r);
                        sidesTouchedMap.entry(*other).and_modify(|mask| *mask |= r);
                        if r == Faces::Top as u8 {
                            break 'nextcube;
                        }
                    };
                }
            }
        }
    }

    println!("{:?}", sidesTouchedMap);
    let outsides: u32 = sidesTouchedMap.iter().map(|(c, v)| v.count_ones()).sum();
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

    // 1882 is too low.

    // assert!(p2 == 2520);
}
