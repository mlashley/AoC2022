struct Board {
    data: Vec<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum RocksEnum {
    HorizLine,
    Cross,
    BackwardsL,
    VertLine,
    Square,
}

static ROCKS: [RocksEnum; 5] = [
    RocksEnum::HorizLine,
    RocksEnum::Cross,
    RocksEnum::BackwardsL,
    RocksEnum::VertLine,
    RocksEnum::Square,
];

impl Board {
    #[allow(dead_code)]
    fn get_height(&self) -> usize {
        self.data.len()
    }
    fn init() -> Self {
        Self {
            data: vec![0x7fu8, 0x00u8],
        }
    }
    fn extend(&mut self, height: usize) -> usize {
        // "... bottom edge is three units above the highest rock in the room"
        // so board needs to potentially extend by 3+current-max-rocks

        let mut top_rock = 0;
        for (y, d) in self.data.iter().enumerate().rev() {
            if *d != 0u8 {
                top_rock = y + 1;
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
        top_rock + 3
    }
    #[allow(dead_code)]
    fn display(&self) {
        for row in self.data.iter().rev() {
            println!("{:07b}", row);
        }
        println!("========");
    }
    #[allow(dead_code)]
    fn display_over(&self, r: &Rock) {
        let mut temp = self.data.clone();

        for y in 0..r.height {
            temp[r.ypos + y] |= Rock::get_outline(r.rock_type)[y] >> (r.offset + 1);
            // +1 because we put our stuff in the MSB and we are only 7bits wide column.
        }

        for row in temp.iter().rev() {
            println!("{:07b}", row);
        }
        println!("========");
    }
    fn move_rock(&mut self, mut r: Rock, c: char) -> Rock {
        if r.dead {
            panic!("You moved a dead rock, it explodes, killing everyone")
        };

        let offset_delta: i8 = match c {
            '<' => {
                if r.offset < 1 {
                    0
                } else {
                    -1
                } // Hit left side
            }

            '>' => {
                if r.offset + r.width > 6 {
                    0
                } else {
                    1
                } // Hit right side
            }
            _ => {
                panic!("Fuuu - unsupported move: {}", c);
            }
        };

        let mut is_ok = true;
        for y in 0..r.height {
            // r.offset +1 by default because we put our stuff in the MSB and we are only 7bits wide column.
            if self.data[r.ypos + y]
                & Rock::get_outline(r.rock_type)[y] >> (r.offset + (1 + offset_delta) as u8)
                != 0
            {
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
            if self.data[r.ypos + y - 1] & Rock::get_outline(r.rock_type)[y] >> (r.offset + 1) != 0
            {
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
                self.data[r.ypos + y] |= Rock::get_outline(r.rock_type)[y] >> (r.offset + 1);
            }
        }
        r
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
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
            // These are intentionally drawn 'upside down' as increaing-y is 'up'
            // It only affects the 'L'
            RocksEnum::HorizLine => vec![0b11110000u8],
            RocksEnum::Cross => vec![0b01000000u8, 0b11100000u8, 0b01000000u8],
            RocksEnum::BackwardsL => vec![0b11100000u8, 0b00100000u8, 0b00100000u8],
            RocksEnum::VertLine => vec![0b10000000u8, 0b10000000u8, 0b10000000u8, 0b10000000u8],
            RocksEnum::Square => vec![0b11000000u8, 0b11000000u8],
        }
    }
    fn height_from_enum(r: RocksEnum) -> usize {
        Rock::get_outline(r).len()
    }
    fn width_from_enum(r: RocksEnum) -> u8 {
        match r {
            RocksEnum::HorizLine => 4,
            RocksEnum::Cross => 3,
            RocksEnum::BackwardsL => 3,
            RocksEnum::VertLine => 1,
            RocksEnum::Square => 2,
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

    #[allow(unused_mut)]
    let mut r = Rock::init_from_enum(RocksEnum::VertLine, 3);
    debug_assert!(
        r == Rock {
            height: 4,
            offset: 2,
            rock_type: RocksEnum::VertLine,
            width: 1,
            ypos: 3,
            dead: false,
        }
    );

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

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            2022
        ) == 3068
    );

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            4000
        ) == 1514285714288
    );
}

fn part1(data: &str, iterations: u64) -> usize {
    let mut b = Board::init();
    let mut rock_types = ROCKS.iter().cycle();
    let mut gas = data.strip_suffix('\n').unwrap().chars().cycle();

    let mut row_period = 0;
    let mut next_period_row = 0;
    let mut rock_period = 0;
    let mut next_period_rock_count = 0;

    const P2_BLOCK_NUMBER: u64 = 1000000000000;

    for i in 1..=iterations {
        let mut r = Rock::init_from_enum(*rock_types.next().unwrap(), 0);
        r.ypos = b.extend(r.height);

        while !r.dead {
            r = b.move_rock(r, gas.next().unwrap());
        }
        // b.display();
        if i == 2022 {
            // Part 2 - try to find period.

            // Ignore 30 rows as probably incomplete, take the preceding 30 (so n-60 .. n-30 ) to match.
            let win_sz = 30;
            let end = b.data.len() - win_sz;
            let slice_to_match = &b.data[end - win_sz..end];
            for rownum in (0..end - win_sz).rev() {
                if &b.data[rownum..rownum + slice_to_match.len()] == slice_to_match {
                    row_period = end - win_sz - rownum;
                    next_period_row = b.data.len() + row_period;
                    println!(
                        "iter={i} Found row period {} waiting for row {} [ rownum {} ]",
                        row_period, next_period_row, rownum
                    );
                    break;
                }
            }
        } else if next_period_row > 0 && b.data.len() == next_period_row {
            rock_period = i - 2022;
            let remaining = ((P2_BLOCK_NUMBER - i as u64) % rock_period as u64) as usize;
            next_period_rock_count = i as usize + remaining;
            println!("iter={i} Found rock period: {rock_period} remaining {remaining} next rock count {next_period_rock_count}");
        } else if next_period_rock_count > 0 && i == next_period_rock_count.try_into().unwrap() {
            let todo_rocks = P2_BLOCK_NUMBER - i;
            let todo_height = (todo_rocks / rock_period) as usize * row_period;

            // Finally exclude the blank space...
            let mut top_rock = 0;
            for (y, d) in b.data.iter().enumerate().rev() {
                if *d != 0u8 {
                    top_rock = y;
                    break;
                }
            }

            let blanks = b.data.len() - top_rock;
            let p2 = todo_height + b.data.len() - blanks;

            // Now that I have it - I think some of this simplifies down, but it is stupid-am...

            println!("P2: {}", p2);
            return p2;
        }
    }

    let answer = b
        .data
        .iter()
        .enumerate()
        .filter(|d| *d.1 == 0u8)
        .collect::<Vec<(usize, &u8)>>();
    answer.first().unwrap().0 - 1
}

fn main() {
    test();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), 2022);
    println!("Part1: {}", p1);
    assert!(p1 == 3106);
    let p2 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        40000,
    );
    println!("Part2: {}", p2);
    assert!(p2 == 1537175792495);
}