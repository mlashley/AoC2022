use std::str::FromStr;
use std::time::Instant;

fn test() {
    debug_assert!(3 == SnafuNum::from_str("1=").unwrap().as_int());
    debug_assert!(20 == SnafuNum::from_str("1-0").unwrap().as_int());
    debug_assert!(12345 == SnafuNum::from_str("1-0---0").unwrap().as_int());

    let mut sn = SnafuNum::from_int(12345);
    debug_assert!(sn.s == "1-0---0");

    sn = SnafuNum::from_int(314159265);
    debug_assert!(sn.s == "1121-1110-1=0");

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == "2=-1=0"
    );
}

// Really didn't need a struct etc. here, but forgot there was typically no part2.
// Nevertheless, allowed(forced) me to implement the FromStr trait properly.
struct SnafuNum {
    s: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSnafuError;

impl FromStr for SnafuNum {
    type Err = ParseSnafuError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { s: String::from(s) })
    }
}

impl SnafuNum {
    fn as_int(&self) -> i64 {
        let base: i64 = 5;
        self.s
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| match c {
                '2' => 2 * base.pow(i as u32),
                '1' => base.pow(i as u32),
                '0' => 0,                   // 0 * ...
                '-' => -base.pow(i as u32), // -1 * ...
                '=' => -2 * base.pow(i as u32),
                _ => {
                    panic!("SNAFU :)");
                }
            })
            .sum()
    }
    fn from_int(i: i64) -> Self {
        let base = 5;
        let mut left = i;
        let mut s = String::from("");
        while left > 0 {
            match left % base {
                0 => {
                    s += "0";
                }
                1 => {
                    left -= 1;
                    s += "1";
                }
                2 => {
                    left -= 2;
                    s += "2";
                }
                3 => {
                    left += 2;
                    s += "=";
                }
                4 => {
                    left += 1;
                    s += "-";
                }
                _ => {
                    panic!("Another SNAFU :) (from_int)")
                }
            }
            left /= 5;
        }
        Self {
            s: s.chars().rev().collect::<String>(),
        }

        // Find current %5 / remainder - round it up or down (with carry/borrow) and 'shift right' (== divide by 5)

        // 11 % 5 => Out: 1
        // 10/5 => 2, mod 5 is 2 => Out:2
        // ANS: 21

        // 201 % 5 == 1 Out:1
        // (200/5)%5 = 40 %5 = 0 Out: 0
        // (40/5)%5 = 8%5 => 3 Out: =
        // 5+(8-3) = 10/5 = 2, 2%5 = 2 Out=2
        // ANS 2=0
    }
}

fn part1(data: &str) -> String {
    let sn = SnafuNum::from_int(
        data.split('\n')
            .map(|x| SnafuNum::from_str(x).unwrap())
            .map(|x| x.as_int())
            .sum(),
    );
    sn.s
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == "2=0=02-0----2-=02-10");
    println!("Completed in {} us", now.elapsed().as_micros());
}
