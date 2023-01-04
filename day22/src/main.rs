use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::BitAnd;
use std::time::Instant;

fn test() {
    let p = PlayerOne {
        x: 7,
        y: 5,
        facing: 90,
        map: vec![],
        instructions: String::from("1L2R3"),
        history: HashMap::new(),
        edges: HashMap::new(),
        foldme: Vec::new(),
        cubesize: 5,
    };
    debug_assert!(p.password() == 6032);

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 6032
    );
    debug_assert!(part2(std::fs::read_to_string("input_sample.txt").unwrap().as_str(),) == 5031);
}

#[derive(Debug, Clone, Default)]
struct PlayerOne {
    x: i64,
    y: i64,
    facing: i16, // 0, 90, 180, 270
    map: Vec<Vec<char>>,
    instructions: String,
    history: HashMap<(i64, i64), i16>,
    edges: HashMap<(char, u8), (char, u8)>,
    foldme: Vec<Vec<char>>,
    cubesize: usize,
}

#[derive(Debug, Clone)]
enum Instruction {
    Forward(u32),
    Rotate(i16),
}
#[derive(Debug, Clone, PartialEq)]
enum FlatMapOrientation {
    // Which axis is the 'long' (4) side on?
    Horizontal,
    Vertical,
}

impl Instruction {
    fn next(s: &mut String) -> Option<Instruction> {
        if s.is_empty() {
            return None;
        };
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
                    Some(Instruction::Forward((v1 * 10) + v2))
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
        let mapwidth = data
            .split('\n')
            .filter(|x| x.contains('.'))
            .map(|x| x.len())
            .max()
            .unwrap();
        let map: Vec<Vec<char>> = data
            .split('\n')
            .filter(|x| x.contains('.'))
            .map(|x| {
                let to_add = mapwidth - x.len(); // probably there is a better padding function ;-)
                let mut s = String::from(x);
                for _ in 0..to_add {
                    s += " "
                }
                s.chars().collect()
            })
            .collect();
        let action_string = data.split('\n').filter(|x| x.contains('R')).last().unwrap();
        let x = map[0]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == '.')
            .unwrap()
            .0 as i64;
        Self {
            x,
            y: 0,
            facing: 90,
            map,
            instructions: action_string.into(),
            history: HashMap::new(),
            edges: HashMap::new(),
            foldme: Vec::new(),
            cubesize: 0, // unused in part 1.
        }
    }
    fn from_str2(data: &str) -> Self {
        let mapwidth = data
            .split('\n')
            .filter(|x| x.contains('.'))
            .map(|x| x.len())
            .max()
            .unwrap();
        let mapheight = data
            .split('\n')
            .filter(|x| x.contains('.'))
            .collect::<Vec<&str>>()
            .len();
        // is either 4x3 or 3x4...
        let orientation = if mapwidth > mapheight {
            FlatMapOrientation::Horizontal
        } else {
            FlatMapOrientation::Vertical
        };
        let cubesize = if orientation == FlatMapOrientation::Horizontal {
            mapwidth / 4
        } else {
            mapwidth / 3
        };
        println!("w x h = {} x {} => cube {}", mapwidth, mapheight, cubesize);

        let flatmap: Vec<Vec<char>> = data
            .split('\n')
            .filter(|x| x.contains('.'))
            .map(|x| {
                let to_add = mapwidth - x.len(); // probably there is a better padding function ;-)
                let mut s = String::from(x);
                for _ in 0..to_add {
                    s += " "
                }
                s.chars().collect()
            })
            .collect();

        let mut foldme = Vec::new();
        let mut i: u8 = 0x40;
        let mut Axy: (usize, usize) = (9, 9);
        for y in (0..mapheight / cubesize) {
            let mut row: Vec<char> = Vec::new();
            for x in (0..mapwidth / cubesize) {
                row.push(if flatmap[y * cubesize][x * cubesize] == ' ' {
                    ' '
                } else {
                    i += 1;
                    if i == 0x41 {
                        Axy = (x, y)
                    }
                    i as char
                });
            }
            foldme.push(row);
        }

        // Given side A with edges 1,2,3,4.
        // +---+
        // | 1 |
        // |4A2|
        // | 3 |
        // +---+

        let mut edges: HashMap<(char, u8), (char, u8)> = HashMap::new();
        println!("Foldme:");
        for row in &foldme {
            println!("{:?}", row);
        }
        println!("A is at {:?}", Axy);
        // A is always front...

        // Map-X-Axis Joins
        for y in 0..foldme.len() {
            let mut last = if orientation == FlatMapOrientation::Horizontal {
                foldme[y].iter().last().unwrap().clone() // this joins the left side to the right side - we have this in neither our sample or actual data.
            } else {
                ' '
            };

            for curr in foldme[y].iter() {
                if *curr == ' ' {
                    last = ' ';
                } else if last != ' ' && *curr != ' ' {
                    // last and curr are joined on L2 = C4 and C4=L2
                    edges.insert((last, 2), (*curr, 4));
                    edges.insert((*curr, 4), (last, 2)); // reverse is true
                }
                last = *curr;
            }
        }
        println!("+ Xaxis Edges: {}", edges.len());
        for e in edges.keys().sorted() {
            println!("{:?} => {:?}", e, edges.get(e).unwrap());
        }

        // Map-Y-Axis Joins
        for x in 0..foldme[0].len() {
            let mut last = if orientation == FlatMapOrientation::Vertical {
                println!("{}|{}", x, foldme.len() - 1);
                foldme[foldme.len() - 1][x] // this joins the top edge, to the bottom edge, in a column
            } else {
                ' '
            };

            for y in 0..foldme.len() {
                let curr = foldme[y][x];
                if curr == ' ' {
                    last = ' ';
                } else if last != ' ' && curr != ' ' {
                    // last and curr are joined on L3=C1 and C1=L3 (top and bottom edges)
                    edges.insert((last, 3), (curr, 1));
                    edges.insert((curr, 1), (last, 3)); // reverse is true
                }
                last = curr;
            }
        }

        println!("+ Yaxis Edges: {}", edges.len());
        for e in edges.keys().sorted() {
            println!("{:?} => {:?}", e, edges.get(e).unwrap());
        }

        // Given side X,T,Y with edges 1,2,3,4. - Whenever B1, B2 have adjacents A3,C3 => C1,A2 is an edge
        // +---+
        // | 1 |
        // |4X2|
        // | 3 |
        // +---+
        // +---++---+
        // | 1 || 1 |
        // |4T2||4Y2|
        // | 3 || 3 |
        // +---++---+
        // If T1,T2 joins X3,Y4 then X2,Y1

        // Third and 4th obvious cases from extending above...
        // TX   and YT
        // Y         X
        // If T2,T3 joins X4,Y1 then X3,Y2
        // If T3,T4 joins X1,Y2 then X4,Y3

        //      +---+  When T1, T4 pair have X2,Y3 respectively => X1,Y4 / Y4,X1 is an edge.
        //      | 1 |
        //      |4Y2|
        //      | 3 |
        //      +---+
        // +---++---+
        // | 1 || 1 |
        // |4X2||4T2|
        // | 3 || 3 |
        // +---++---+
        // If T4,T1 joins X2,Y3 then X1,Y4

        // If T1,T2 joins X3,Y4 then X2,Y1
        // If T2,T3 joins X4,Y1 then X3,Y2
        // If T3,T4 joins X1,Y2 then X4,Y3
        // If T4,T1 joins X2,Y3 then X1,Y4

        // This generalizes as below:

        while edges.len() != 24 {
            // We *could* check if we add new edges and terminate when we don't... but
            // I'm sure there is a better pattern to enrich a hash, but we can't insert because the lookups form an immutable borrow...
            let mut todo: Vec<((char, u8), (char, u8))> = Vec::new();
            for target_side in 'A'..='F' {
                // First case above.
                // If T1,T2 joins X3,Y4 then X2,Y1

                for target_sidenum1 in 1..=4 {
                    let mut target_sidenum2 = target_sidenum1 + 1;
                    if target_sidenum2 > 4 {
                        target_sidenum2 -= 4
                    };
                    if let Some((x, xnum)) = edges.get(&(target_side, target_sidenum1)) {
                        if let Some((y, ynum)) = edges.get(&(target_side, target_sidenum2)) {
                            // xnum -1, ynum +1
                            let mut newxnum = xnum - 1;
                            if newxnum == 0 {
                                newxnum = 4
                            }
                            let mut newynum = ynum + 1;
                            if newynum > 4 {
                                newynum -= 4
                            }
                            println!(
                                "{}{}=>{}{} {}{}=>{}{} implies {}{}={}{}  [P]",
                                target_side,
                                target_sidenum1,
                                x,
                                xnum,
                                target_side,
                                target_sidenum2,
                                y,
                                ynum,
                                x,
                                newxnum,
                                y,
                                newynum
                            );
                            todo.push(((*x, newxnum), (*y, newynum)));
                        }
                    }
                }
            }
            while let Some(p) = todo.pop() {
                edges.insert(p.0, p.1);
                edges.insert(p.1, p.0);
            }
        }

        println!("Edges: {}", edges.len());
        for e in edges.keys().sorted() {
            println!("{:?} => {:?}", e, edges.get(e).unwrap());
        }

        // Odd/Odd(!=) and Even/Even(!=) edges all stay flat (1-3, 2-4 etc)
        // Even/Even(equal) edge-pairs (2-2, 4-4) are flipped so the y0 => ymax and ymax => y0 as they cross:  y=>ymax-y 180deg rotate from flatmap perspective.
        // Odd/Odd(equal) - dunno?
        // Odd/Even => x=>y (also rotate from 'flatmap' perspective)
        // Even/Odd => y=>x (also rotate from 'flatmap' perspective)

        let action_string = data.split('\n').filter(|x| x.contains('R')).last().unwrap();
        let x = flatmap[0]
            .iter()
            .enumerate()
            .find(|(_, &v)| v == '.')
            .unwrap()
            .0 as i64;


        Self {
            x,
            y: 0,
            facing: 90,
            map: flatmap,
            instructions: action_string.into(),
            history: HashMap::new(),
            edges,
            foldme,
            cubesize,
        }

    }

    fn get_side(&self) -> (char,u8) {
        let small_y = self.y as usize / self.cubesize;
        let small_x = self.x as usize / self.cubesize;
        let side = self.foldme[small_y][small_x];
        
        let mut my_edges: u8 = 0;
        if self.x as usize % self.cubesize == 0 {
            my_edges |= 1 << 3;
        }
        if (self.x as usize + 1) % self.cubesize == 0 {
            my_edges |= 1 << 1;
        }
        if self.y as usize % self.cubesize == 0 {
            my_edges |= 1 << 0;
        }
        if (self.y as usize + 1) % self.cubesize == 0 {
            my_edges |= 1 << 2;
        }
        println!("Side of {},{} => [{}.{}] is {}, Edges: {}",self.x,self.y,small_x,small_y,side,my_edges);

        (side,my_edges)
    }

    fn flat_to_face(&self,x: usize ,y: usize) -> (usize,usize) {
        let small_y = y  / self.cubesize;
        let small_x = x  / self.cubesize;
        let side = self.foldme[small_y][small_x];
        let face_x = x - (small_x*self.cubesize);
        let face_y = y - (small_y*self.cubesize);
        println!("Global {},{} => side {} face_coord {},{}",x,y,side,face_x,face_y);
        (face_x,face_y)
    }

    fn face_to_flat(&self,face: char,face_x: usize ,face_y: usize) -> (i64,i64 ) {
        let mut small_x = 0;
        let mut small_y = 0;
        while self.foldme[small_y][small_x] != face {
            
            small_x += 1;
            if small_x == self.foldme[0].len() {
                small_x = 0;
                small_y += 1;
            }
        }
        println!("face_to_flat: Found {} at {} {}",face,small_x,small_y);
        let flat_x = (small_x*self.cubesize)+face_x;
        let flat_y = (small_y*self.cubesize)+face_y;
        (flat_x as i64,flat_y as i64)
    }

    fn mov2(&mut self, distance: u32) {

// 0123456789012345
//           111111 

//         0123       0       A
//         1#..       1     BCD
//         2...       2       EF
//         3...       3
// 012301230123       4
// 1...1...1...       5
// 2.#.2..#2...       6
// 3...3...3.#.       7
//         01230123   8
//         1...1#..   9
//         2#..2...  10
//         3...3.#.  11

// 0123456789012345
//           111111 
        

        let mut count = 0;
        while count < distance {

            let (xdelta, ydelta) = match self.facing {
                0 => (0, -1),
                90 => (1, 0),
                180 => (0, 1),
                270 => (-1, 0),
                _ => {
                    panic!("We are the hellarwi!")
                }
            };

            // Are we about to move off the side?
            let mut curr_face = self.get_side();
            let mut next_face: (char,u8);

            let (fx,fy) = self.flat_to_face(self.x as usize, self.y as usize);

            let mut nextx = self.x + xdelta;
            let mut nexty = self.y + ydelta;
            let mut next_rot = self.facing;

            if curr_face.1.bitand(1)==1 && self.facing == 0 {
                next_face = *self.edges.get(&(curr_face.0,1)).unwrap();
                curr_face.1 = 1;
            } else if curr_face.1.bitand(2)==2 && self.facing == 90  {
                next_face = *self.edges.get(&(curr_face.0,2)).unwrap();
                curr_face.1 = 2;
            } else if curr_face.1.bitand(4)==4 && self.facing == 180 {
                next_face = *self.edges.get(&(curr_face.0,3)).unwrap();
                curr_face.1 = 3;
            } else if curr_face.1.bitand(8)==8 && self.facing == 270 {
                next_face = *self.edges.get(&(curr_face.0,4)).unwrap();
                curr_face.1 = 4;
            } else { 
                next_face = curr_face;

                //normal - already set above.
                // todo!("Normal move")
            }
            if curr_face.0 != next_face.0 {
                if curr_face.1 % 2 == 0 {   
                    // curr face is even
                    if next_face.1 % 2 == 0 {
                        // next face is even
                        if curr_face.1 == next_face.1 {
                            // E/E(==) flip y and rotate 180
                            next_rot = (((self.facing + 180) % 360) + 360) % 360; // modulo
                            match next_face.1 {
                                2 => (nextx,nexty) = self.face_to_flat(next_face.0, self.cubesize-1, (self.cubesize-1)-fy),
                                4 => (nextx,nexty) = self.face_to_flat(next_face.0, 0, (self.cubesize-1)-fy ),
                                _ => { panic!("Next side is not 2 or 4.. got {}",next_face.1)},
                            };

                            // todo!("E/E(==) flip y and rotate 180 {:?} {:?}",curr_face,next_face);

                        } else {
                            // E/E(!=) works as flat
                        }
                    } else { 
                        // next face is odd
                        // E/O(!=) y=> x and rotate -90 
                        let (rot,nfx,nfy) = 
                        match (curr_face.1,next_face.1) {
                            (4,1) => (-90,fy,                  0),
                            (2,3) => (-90,fy,                  self.cubesize-1),
                            (4,3) => ( 90,(self.cubesize-1)-fy,self.cubesize-1),
                            (2,1) => ( 90,(self.cubesize-1)-fy,0),
                            _ => panic!("Funny old E/O != pair? {:?}, {:?}", curr_face,next_face)
                        };
                        next_rot = (((self.facing+rot) % 360) + 360) % 360; // modulo
                        println!("nfx,y {},{} [fx, fy {},{}]",nfx,nfy,fx,fy);
                        (nextx,nexty) = self.face_to_flat(next_face.0, nfx, nfy);
                        println!("E/O(!=) y=>x and rotate {:?} {:?}",curr_face,next_face);
                    }

                } else {
                    // curr face is odd
                    if next_face.1 % 2 == 0 {
                        // next face is even 
                        // O/E(!=) x=>y and rotate +90
                        next_rot = (((self.facing + 90) % 360) + 360) % 360; // modulo
                        match next_face.1 {
                            2 => (nextx,nexty) = self.face_to_flat(next_face.0, self.cubesize-1, fx),
                            4 => (nextx,nexty) = self.face_to_flat(next_face.0, 0, fx),
                            _ => { panic!("Next side is not 2 or 4.. got {}",next_face.1)},
                        };
                        // todo!("O/E(!=) x=>y and rotate {:?} {:?}",curr_face,next_face);
                        
                    } else { 
                        // next face is odd
                        if curr_face.1 == next_face.1 {
                            // O/O(==) flip y and rotate 180
                            next_rot = (((self.facing + 180) % 360) + 360) % 360; // modulo
                            match next_face.1 {
                                1 => (nextx,nexty) = self.face_to_flat(next_face.0, (self.cubesize-1)-fx, 0),
                                3 => (nextx,nexty) = self.face_to_flat(next_face.0, (self.cubesize-1)-fx, self.cubesize-1),
                                _ => { panic!("Next side is not 1 or 3.. got {}",next_face.1)},
                            };

                            // todo!("Odd to Odd equal")
                        } else {
                            // O/O(!=) works as flat - except when on opposite sides...
                            match next_face.1 {
                                1 => (nextx,nexty) = self.face_to_flat(next_face.0, fx, 0),
                                3 => (nextx,nexty) = self.face_to_flat(next_face.0, fx, self.cubesize-1),
                                _ => { panic!("Next side is not 1 or 3.. got {}",next_face.1)},
                            };
                        }
                        
                    }

                }
                
            }
        // Odd/Odd(!=) and Even/Even(!=) edges all stay flat (1-3, 2-4 etc)
        // Even/Even(equal) edge-pairs (2-2, 4-4) are flipped so the y0 => ymax and ymax => y0 as they cross:  y=>ymax-y 180deg rotate from flatmap perspective.
        // Odd/Odd(equal) - dunno?
        // Odd/Even => x=>y (also rotate from 'flatmap' perspective)
        // Even/Odd => y=>x (also rotate from 'flatmap' perspective)


            // if nexty < 0 {
            //     nexty = self.map.len() as i64 - 1
            // } // wrap top around to bottom
            // if nextx < 0 {
            //     nextx = self.map[0].len() as i64 - 1
            // } // wrap
            // if nexty == self.map.len() as i64 {
            //     nexty = 0
            // } // wrap
            // if nextx == self.map[0].len() as i64 {
            //     nextx = 0
            // } // wrap

            println!("NextX,Y is {},{} looking for space",nextx,nexty);


            assert!(nextx >= 0);
            assert!(nexty >= 0);

            while match self.map[nexty as usize][nextx as usize] {
                '.' => {
                    self.x = nextx;
                    self.y = nexty;
                    self.facing = next_rot;
                    false
                }
                '#' => false,
                ' ' => {
                    panic!("Attempt to read the void map space at nextx,y = {},{}",nextx,nexty);
                }
                _ => {
                    panic!["Map is corrupt :("]
                }
            } {}
            self.history.insert((self.x, self.y), self.facing);
            count += 1;
        }
    }

    fn rot(&mut self, deg: i16) {
        self.facing = (((self.facing + deg) % 360) + 360) % 360; // modulo
        self.history.insert((self.x, self.y), self.facing);
    }

    fn mov(&mut self, distance: u32) {
        let (xdelta, ydelta) = match self.facing {
            0 => (0, -1),
            90 => (1, 0),
            180 => (0, 1),
            270 => (-1, 0),
            _ => {
                panic!("We are the hellarwi!")
            }
        };
        let mut count = 0;
        while count < distance {
            let mut nextx = self.x + xdelta;
            let mut nexty = self.y + ydelta;
            if nexty < 0 {
                nexty = self.map.len() as i64 - 1
            } // wrap top around to bottom
            if nextx < 0 {
                nextx = self.map[0].len() as i64 - 1
            } // wrap
            if nexty == self.map.len() as i64 {
                nexty = 0
            } // wrap
            if nextx == self.map[0].len() as i64 {
                nextx = 0
            } // wrap

            while match self.map[nexty as usize][nextx as usize] {
                '.' => {
                    self.x = nextx;
                    self.y = nexty;
                    false
                }
                '#' => false,
                ' ' => {
                    nextx += xdelta;
                    nexty += ydelta;
                    if nexty < 0 {
                        nexty = self.map.len() as i64 - 1
                    } // wrap top around to bottom
                    if nextx < 0 {
                        nextx = self.map[0].len() as i64 - 1
                    } // wrap
                    if nexty == self.map.len() as i64 {
                        nexty = 0
                    } // wrap
                    if nextx == self.map[0].len() as i64 {
                        nextx = 0
                    } // wrap
                    true
                }
                _ => {
                    panic!["Map is corrupt :("]
                }
            } {}
            self.history.insert((self.x, self.y), self.facing);
            count += 1;
        }
    }
    fn password(&self) -> i64 {
        (self.y + 1) * 1000
            + (self.x + 1) * 4
            + match self.facing {
                90 => 0,
                180 => 1,
                270 => 2,
                0 => 3,
                _ => {
                    panic!("Stand in the place where you live, now face north, think about direction, wonder why you haven't before.")
                }
            }
    }
    fn print_map(&self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let xy = (x as i64, y as i64);
                if self.history.contains_key(&xy) {
                    let ch = match self.history.get(&xy).unwrap() {
                        0 => '^',
                        90 => '>',
                        180 => 'v',
                        270 => '<',
                        _ => {
                            panic!("Faaaaaaack")
                        }
                    };
                    print!("{}", ch);
                } else {
                    print!("{}", self.map[y][x]);
                }
            }
            println!();
        }
    }
}

fn part1(data: &str) -> i64 {
    let mut player = PlayerOne::from_str(data);

    println!("PLAYER: {:?}", player);

    while let Some(ins) = Instruction::next(&mut player.instructions) {
        println!("INS: {:?}", ins);
        match ins {
            Instruction::Rotate(r) => player.rot(r),
            Instruction::Forward(d) => player.mov(d),
        }
    }
    player.print_map();
    player.password()
}

fn part2(data: &str) -> i64 {
    let mut player = PlayerOne::from_str2(data);

    println!("PLAYER: {:?}", player);

    player.print_map();
    assert!(player.get_side() == ('A',9));

    while let Some(ins) = Instruction::next(&mut player.instructions) {
        println!("INS: {:?}", ins);
        match ins {
            Instruction::Rotate(r) => player.rot(r),
            Instruction::Forward(d) => player.mov2(d),
        }
        player.print_map();

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
    let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part2: {}", p2);
    assert!(p2 as i64 == 144012);
    println!("Completed in {} us", now.elapsed().as_micros());
}
