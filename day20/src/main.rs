use std::time::Instant;

fn test() {
    // Treatise on remainder vs modulo for -ve numbers.
    // let mo = 7;
    // for a in [-1i32,-2, -7,-8,-22] {
    //     println!("{} % {} = {}",a,mo, a % mo );
    //     println!("{} rem_euclid {} = {}",a,mo, a.rem_euclid(mo) );
    // }

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            1,
            1,
        ) == 3
    );
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
            811589153,
            10
        ) == 1623178306
    );
}

fn calc_result(v: Vec<NumberWithPosition>) -> i64 {
    let zero_at = v.iter().position(|x| x.number == 0).unwrap();
    let mut answer = Vec::new();

    for i in 1..=3 {
        answer.push((zero_at + (i * 1000)) % v.len());
    }
    answer.iter().map(|x| v[*x].number).sum()
}

// So there are duplicate numbers in the non-sample data - and all previous entries didn't work
// because we assumed there weren't - make a struct to keep them separate.
#[derive(Debug)]
struct NumberWithPosition {
    number: i64,
    orig_pos: usize,
}

fn part1(data: &str, scale: i64, iterations: usize) -> i64 {
    let mut i = 0;
    let mut origlist: Vec<NumberWithPosition> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| {
            let r = NumberWithPosition {
                number: x.parse::<i64>().unwrap() * scale,
                orig_pos: i,
            };
            i += 1;
            r
        })
        .collect();

    let message_size = origlist.len() - 1;
    for _mixes in 0..iterations {
        for orig_pos in 0..=message_size {
            let cur_pos = origlist
                .iter()
                .position(|x| x.orig_pos == orig_pos)
                .unwrap();
            let value = origlist[cur_pos].number;

            let new_pos = (cur_pos as i64 + value).rem_euclid(message_size as i64);

            let elem = origlist.remove(cur_pos);
            origlist.insert(new_pos as usize, elem);
        }
    }
    calc_result(origlist)
}

fn main() {
    const DECRYPTION_KEY: i64 = 811589153;
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str(), 1, 1);
    println!("Part1: {}", p1);
    assert!(p1 == 23321);
    let p2 = part1(
        std::fs::read_to_string("input.txt").unwrap().as_str(),
        DECRYPTION_KEY,
        10,
    );
    println!("Part2: {}", p2);
    assert!(p2 == 1428396909280);
    println!("Completed in {} us", now.elapsed().as_micros());
}
