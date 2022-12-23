use std::collections::HashMap;
use std::time::Instant;

fn test() {

    let p = PlayerOne { x:7, y:5, facing: 90, map: vec![], instructions: String::from("1L2R3"), history: HashMap::new() };
    debug_assert!(p.password() == 6032);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 6032
    );
    // debug_assert!(part2(std::fs::read_to_string("input.txt").unwrap().as_str(),) == 301);
}

#[derive(Debug, Clone)]
struct PlayerOne {
    x: i64,
    y: i64,
    facing: i16, // 0, 90, 180, 270
    map: Vec<Vec<char>>,
    instructions: String,
    history: HashMap<(i64,i64),i16>,
}

#[derive(Debug, Clone)]
enum Instruction {
    Forward(u32),
    Rotate(i16),
}

impl Instruction {
    fn next(s: &mut String) -> Option<Instruction> {
        if s.is_empty() { return None };
        let c1 = s.chars().next().unwrap();
        if c1 == 'R' {
            *s = s[1..].to_string();
            Some(Instruction::Rotate(90))
        } else if c1 == 'L' {
            *s = s[1..].to_string();
            Some(Instruction::Rotate(-90))
        } else if c1.is_ascii_digit() {
            let v1 = c1.to_digit(10).unwrap();
            if s.len() > 1 {
                let c2 = s.chars().nth(1).unwrap();
                if c2 == 'R' || c2 == 'L' {
                    *s = s[1..].to_string();
                    Some(Instruction::Forward(v1))
                } else {
                    let v2 = c2.to_digit(10).unwrap();
                    *s = s[2..].to_string();
                    Some(Instruction::Forward((v1*10)+v2))
                }
            } else {
                *s = s[1..].to_string();
                Some(Instruction::Forward(v1))
            }
        } else {
            panic!("I'm sorry Dave, I cannot do that.")
        }
    }
}

impl PlayerOne {
    fn from_str(data: &str) -> Self {

        let mapwidth = data.split('\n').filter(|x| x.contains('.')).map(|x| x.len()).max().unwrap();
        let map: Vec<Vec<char>> = data.split('\n').filter(|x| x.contains('.')).map(
            |x|
            {
                let to_add = mapwidth - x.len(); // probably there is a better padding function ;-)
                let mut s = String::from(x);
                for _ in 0..to_add {
                    s += " "
                }
                s.chars().collect()
            }
            
        )
        .collect();
        let action_string = data.split('\n').filter(|x| x.contains('R')).last().unwrap();
        let x = map[0].iter().enumerate().find(|(_, &v)| v == '.').unwrap().0 as i64;
        Self {
            x,
            y: 0,
            facing: 90,
            map,
            instructions: action_string.into(),
            history: HashMap::new(),
        }
    }
    fn rot(&mut self, deg: i16) {
        self.facing = (((self.facing + deg) %360) + 360) % 360 ; // modulo
        self.history.insert((self.x,self.y),self.facing);
    }

    fn mov(&mut self, distance: u32) {

        let (xdelta,ydelta) = 
        match self.facing {
            0 => (0,-1),
            90 => (1,0),
            180 => (0,1),
            270 => (-1,0),
            _ => { panic!("We are the hellarwi!")},
        };
        let mut count = 0;
        while count < distance {
            let mut nextx = self.x + xdelta;
            let mut nexty = self.y + ydelta;
            if nexty < 0 { nexty = self.map.len() as i64 -1} // wrap top around to bottom
            if nextx < 0 { nextx = self.map[0].len() as i64 -1} // wrap
            if nexty == self.map.len() as i64  { nexty = 0} // wrap
            if nextx == self.map[0].len() as i64  { nextx = 0} // wrap
            
            while match self.map[nexty as usize][nextx as usize] {
                '.' => { self.x = nextx; self.y = nexty; false },
                '#' => false,
                ' ' => {
                    nextx += xdelta;
                    nexty += ydelta;
                    if nexty < 0 { nexty = self.map.len() as i64 -1} // wrap top around to bottom
                    if nextx < 0 { nextx = self.map[0].len() as i64 -1} // wrap
                    if nexty == self.map.len() as i64  { nexty = 0} // wrap
                    if nextx == self.map[0].len() as i64  { nextx = 0} // wrap
                    true
                },
                _ => { panic!["Map is corrupt :("]}

            } {} ;
            self.history.insert((self.x,self.y),self.facing);
            count += 1;
        }


    }
    fn password(&self) -> i64 {
        (self.y+1) * 1000
        + (self.x+1) * 4
        + match self.facing {
            90 => 0,
            180 => 1,
            270 => 2,
            0 => 3,
            _ => { panic!("Stand in the place where you live, now face north, think about direction, wonder why you haven't before.")}
        }
    }
    fn print_map(&self)  {
        for y in 0..self.map.len() {
            for x in 0.. self.map[y].len() {
                let xy = (x as i64 ,y as i64);
                if self.history.contains_key(&xy) {
                    let ch = match self.history.get(&xy).unwrap() {
                        0 => '^',
                        90 => '>',
                        180 => 'v',
                        270 => '<',
                        _ => { panic!("Faaaaaaack")}
                    };
                    print!("{}",ch);
                } else {
                    print!("{}",self.map[y][x]);
                }

            }
            println!();
        }
    }
}

fn part1(data: &str) -> i64 {
    
    // let player = PlayerOne::init(data.split('\n').next().unwrap().chars());
    let mut player = PlayerOne::from_str(data);

    println!("PLAYER: {:?}",player);

    
    while let Some(ins) = Instruction::next(&mut player.instructions) {
        println!("INS: {:?}",ins);
        match ins {
            Instruction::Rotate(r) => player.rot(r),
            Instruction::Forward(d) => player.mov(d)
        }
        
    }
    player.print_map();
    player.password()
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 76332);
    // let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    // println!("Part2: {}", p2);
    // assert!(p2 as i64 == ?);
    println!("Completed in {} us", now.elapsed().as_micros());
}
