// use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::ops::Sub;

// use std::cmp;
// use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Cube {
    fn touching(&self, other: &Self) -> bool {
        match *self-*other {
            Cube { x: 1, y: 0, z: 0} => true,
            Cube { x: -1, y: 0, z: 0} => true,
            Cube { x: 0, y: 1, z: 0} => true,
            Cube { x: 0, y: -1, z: 0} => true,
            Cube { x: 0, y: 0, z: 1} => true,
            Cube { x: 0, y: 0, z: -1} => true,
            _ => false,
        }
    }
}

fn test() {

    let c1 = "1,1,1".parse::<Cube>().unwrap();
    let c2 = "1,2,1".parse::<Cube>().unwrap();
    let c3 = "0,1,0".parse::<Cube>().unwrap();
    println!("c2 {} - c1 {} = {:?}",c2,c1,c2-c1,);
    debug_assert!(c1 != c2);
    debug_assert!(c2-c1 == c3);
    debug_assert!(c2.touching(&c1));
    debug_assert!(c1.touching(&c2));
    debug_assert!(!c1.touching(&c3));
    debug_assert!(!c3.touching(&c1));


    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            false
        ) == 64
    );
    // debug_assert!(
    //     part1(
    //         std::fs::read_to_string("input_sample.txt")
    //             .unwrap()
    //             .as_str(),
    //         true
    //     ) == 1707
    // );

}

fn part1(data: &str, is_part2: bool) -> usize {

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
            if i.touching(&j) { touching_sides += 1 };
        })
    });
    
    println!("Total Sides {} - Touching Sides {} == {}",all_sides,touching_sides, all_sides-touching_sides);
    all_sides-touching_sides
}

fn main() {
    test();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        false,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 3576);
    // let p2 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), true);
    // println!("Part2: {}", p2);
    // assert!(p2 == 2520);
}
