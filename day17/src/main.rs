
struct Board {
    data: Vec<u8>
}

#[derive(Debug,Copy,Clone,PartialEq)]
enum RocksEnum {
    HorizLine,
    Cross,
    BackwardsL,
    VertLine,
    Square,
}

static ROCKS: [RocksEnum; 5]  = [RocksEnum::HorizLine,RocksEnum::Cross,RocksEnum::BackwardsL,RocksEnum::VertLine,RocksEnum::Square];

impl Board {
    fn get_height(&self) -> usize {
        self.data.len()
    }
    fn init() -> Self { Self { data: vec![0x7fu8,0x00u8]}}
    fn extend(&mut self, height: usize) -> usize {
        // "... bottom edge is three units above the highest rock in the room"
        // so board needs to potentially extend by 3+current-max-rocks

        let mut top_rock = 0;
        for (y,d) in self.data.iter().enumerate().rev() {
            if *d != 0u8 {
                top_rock = y+1;
                break;
            }
        }
        // Need to account for the case we extended to fit something tall, then we have something short.
        // This can underflow then...

        let needed_height = top_rock + 3 + height;
        if self.data.len() < needed_height {
            // println!("Cur: {} Extending: {} from toprock {} to accommodate height {}",self.data.len(),needed_height,top_rock,height);
            for _ in 0..needed_height - self.data.len() {
                self.data.push(0x0u8);
            }
        }
        top_rock+3
    }
    fn display(&self) {
        for row in self.data.iter().rev() {
            println!("{:07b}",row);
        }
        println!("========");
    }
    fn display_over(&self, r: &Rock) {
        let mut temp = self.data.clone();

        for y in 0..r.height {
            temp[r.ypos+y] |= Rock::get_outline(r.rock_type)[y] >> (r.offset+1); 
            // +1 because we put our stuff in the MSB and we are only 7bits wide column.
        }
        
        for row in temp.iter().rev() {
            println!("{:07b}",row);
        }
        println!("========");
    }
    fn move_rock(&mut self, mut r: Rock, c: char) -> Rock {
        if r.dead { panic!("You moved a dead rock, it explodes, killing everyone")};

        let offset_delta: i8 = match c {
            '<' => { 
                if r.offset < 1 { 0 } else { -1 } // Hit left side
            },

            '>' =>  {
                if r.offset+r.width > 6 { 0 } else { 1 } // Hit right side
            },
            _ => { panic!("Fuuu - unsupported move: {}",c); }
        };

        
        let mut is_ok = true;
        for y in 0..r.height {
            // r.offset +1 by default because we put our stuff in the MSB and we are only 7bits wide column.
            if self.data[r.ypos+y] & Rock::get_outline(r.rock_type)[y] >> (r.offset + (1+offset_delta) as u8) != 0 {
                is_ok = false;
                break;
            };
        }
        if is_ok {      
           r.offset = (r.offset as i8 + offset_delta) as u8;
        }

        // Try down...
        is_ok = true;
        for y in 0..r.height {
            if self.data[r.ypos+y-1] & Rock::get_outline(r.rock_type)[y] >> (r.offset + 1) != 0 {
                is_ok = false;
                break;
            };
        }
        if is_ok {      
            r.ypos -= 1;
        } else {
            // println!("Dead");
            r.dead = true;
            // Apply the change to the board
            for y in 0..r.height {
                self.data[r.ypos+y] |= Rock::get_outline(r.rock_type)[y] >> (r.offset + 1);
            }

        }
        r
    }
}

#[derive(Debug,PartialEq,Copy,Clone)]
struct Rock {
    rock_type: RocksEnum,
    offset: u8,
    width: u8,
    height: usize,
    ypos: usize,
    dead: bool,
}

impl Rock {
    fn get_outline(r: RocksEnum) -> Vec<u8> {
        match r {
            // These are intentionall drawn 'upside down' as increaing-y is 'up'
            // It only affects the 'L'
            RocksEnum::HorizLine =>  vec![0b11110000u8],
            RocksEnum::Cross =>      vec![0b01000000u8,
                                        0b11100000u8,
                                        0b01000000u8],
            RocksEnum::BackwardsL => vec![0b11100000u8,
                                        0b00100000u8,
                                        0b00100000u8],
            RocksEnum::VertLine =>   vec![0b10000000u8,
                                        0b10000000u8,
                                        0b10000000u8,
                                        0b10000000u8],
            RocksEnum::Square =>     vec![0b11000000u8,
                                        0b11000000u8,],
        }
    }
    fn height_from_enum(r: RocksEnum) -> usize {
        Rock::get_outline(r).len()
    }
    fn width_from_enum(r: RocksEnum) -> u8 {
        match r {
            RocksEnum::HorizLine  => 4,
            RocksEnum::Cross      => 3,
            RocksEnum::BackwardsL => 3,
            RocksEnum::VertLine   => 1,
            RocksEnum::Square     => 2,
        }
    }
    fn init_from_enum(rock_type: RocksEnum, ypos: usize) -> Self {
        Self {
            rock_type,
            offset: 2, // We start at 2 from the left wall.
            width: Self::width_from_enum(rock_type),
            height: Self::height_from_enum(rock_type),
            ypos,
            dead: false,
        }
    }
}



fn test() {

    let _a = ROCKS.iter();

    debug_assert!(Rock::height_from_enum(RocksEnum::Square) == 2);
    debug_assert!(Rock::height_from_enum(RocksEnum::VertLine) == 4);

    let mut r = Rock::init_from_enum(RocksEnum::VertLine,3);
    debug_assert!( r == Rock { 
        height: 4,
        offset: 2,
        rock_type: RocksEnum::VertLine,
        width: 1,
        ypos: 3,
        dead: false,
    });


    // let mut b = Board::init();
    // b.extend(r.height);
    // b.display();
    // b.display_over(&r);
    // r=b.move_rock(r, '<');
    // b.display_over(&r);
    // r=b.move_rock(r, '<');
    // b.display_over(&r); // at edge
    // println!("Should not move left");
    // r=b.move_rock(r, '<'); 
    // debug_assert!(r.dead);
    // b.display_over(&r); // at edge
    // // r=b.move_rock(r, '>'); // dead
    
    
    // r = Rock::init_from_enum(E_Rocks::Cross,0);
    // r.ypos = b.extend(r.height);
    // println!("Extended");
    // b.display();
    // b.display_over(&r);
    // r=b.move_rock(r, '>'); b.display_over(&r);
    // r=b.move_rock(r, '>'); b.display_over(&r);
    // r=b.move_rock(r, '>'); b.display_over(&r);
    // r=b.move_rock(r, '>'); b.display_over(&r);
    // r=b.move_rock(r, '<'); b.display_over(&r);
    // r=b.move_rock(r, '<'); b.display_over(&r);
    // r=b.move_rock(r, '<'); b.display_over(&r);
    // r=b.move_rock(r, '>'); b.display_over(&r);
    // debug_assert!(r.dead);
    

    assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            2022
        ) == 3068
    );
                      
    // for i in vec![100000000] {
    //     println!("{} => {}",i,part1(
    //         std::fs::read_to_string("input_sample.txt")
    //             .unwrap()
    //             .as_str(),
    //         i
    //     ));
    // }
    // panic!("Malc");
    
}

fn part1(data: &str, iterations: u64) -> usize {
    let mut b = Board::init();
    let mut rock_types = ROCKS.iter().cycle();
    let mut gas = data.strip_suffix('\n').unwrap().chars().cycle();

    for _ in 1..=iterations {       
        let mut r = Rock::init_from_enum(*rock_types.next().unwrap(),0);
        r.ypos = b.extend(r.height);

        while !r.dead {
            r=b.move_rock(r, gas.next().unwrap()); 
        }
        // b.display();
    }
    
    let answer = b.data.iter().enumerate().filter(|d| *d.1 == 0u8).collect::<Vec<(usize,&u8)>>();
    answer.first().unwrap().0-1
}



fn main() {
    test();
    let p1 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        2022,
    );
    println!("Part1: {}", p1);
    assert!(p1 == 3106);
    // let p2 = part1(
    //     std::fs::read_to_string("input.txt").unwrap().as_str(),
    //     1000000000000,
    // );
    // println!("Part2: {}", p2);
    // assert!(p2 == 2520);
}

