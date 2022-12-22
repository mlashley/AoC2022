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
        ) == 3
    );
}

fn calc_result(v: Vec<NumberWithPosition>) -> i32 {
    let zero_at = v.iter().position(|x| x.number == 0).unwrap();
    let mut answer = Vec::new();

    for i in 1..=3 {
        answer.push((zero_at + (i * 1000)) % v.len());
    }
    answer.iter().map(|x| v[*x].number).sum()
}

// So there are duplicate numbers in the non-sample data - and all previous entries didn't work
// because we assumed there weren't - make a struct to keep them separate.

struct NumberWithPosition {
    number: i32,
    orig_pos: usize,
}

fn part1(data: &str) -> i32 {
    let mut i = 0;
    let mut origlist: Vec<NumberWithPosition> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| {
            let r = NumberWithPosition {
                number: x.parse::<i32>().unwrap(),
                orig_pos: i,
            };
            i += 1;
            r
        })
        .collect();

    let message_size = origlist.len() - 1;
    for orig_pos in 0..=message_size {
        let cur_pos = origlist
            .iter()
            .position(|x| x.orig_pos == orig_pos)
            .unwrap();
        let value = origlist[cur_pos].number;

        let new_pos = (cur_pos as i32 + value).rem_euclid(message_size as i32);

        let elem = origlist.remove(cur_pos);
        origlist.insert(new_pos as usize, elem);
    }
    calc_result(origlist)
}

fn main() {
    test();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 != -12134); // First wrong answer
    assert!(p1 != 18535); // Another wrong answer.
    assert!(p1 != -13438);
    assert!(p1 != -22100);
    assert!(p1 == 23321); // Finally :)

    // assert!(p1 == ??);
    // let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    // println!("Part2: {}", p2);
    // assert!(p2 == ??);
}
