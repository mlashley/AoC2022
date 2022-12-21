use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Sub};
use std::vec;



fn test() {

    // let v: VecDeque<i32> = VecDeque::from([ 1, 2, -3, 4, 0, 3, -2 ]);
    // debug_assert!(calc_result(v) == 3);

    // Treaties on remainder vs modulo for -ve numbers.    
    let mo = 7;
    for a in [-1i32,-2, -7,-8,-22] {
        println!("{} % {} = {}",a,mo, a % mo );
        println!("{} rem_euclid {} = {}",a,mo, a.rem_euclid(mo) );
    }
    // debug_assert!(
    //     part1(
    //         std::fs::read_to_string("input_malc.txt")
    //             .unwrap()
    //             .as_str(),
    //     ) == 3
    // );
    // let mut t1: VecDeque<i32> = VecDeque::from([ -1, 2, 3, 4, 0, 5, 1 ]);
    // println!("{:?} Moving -1",t1);
    // move_number(&mut t1,-1);
    // println!("{:?}",t1);
    // debug_assert!(t1 == VecDeque::from([ 2, 3, 4, 0, 5, -1, 1 ]));
    // println!("{:?} Moving 5",t1);
    // move_number(&mut t1,5);
    // debug_assert!(t1 == VecDeque::from([ 2, 3, 4, 5, 0, -1, 1 ]));
    // println!("{:?}",t1);

    // let mut t1: VecDeque<i32> = VecDeque::from([ 2, -1, 3, 4, 0, 5, 1 ]);
    // println!("{:?} Moving -1",t1);
    // move_number(&mut t1,-1);
    // println!("{:?} ",t1);

    // debug_assert!(t1 == VecDeque::from([ -1, 2, 3, 4, 0, 5, 1  ]));


    // panic!("singletests");

    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 3
    );

}
fn move_number(v: &mut VecDeque<i32>,number: i32) {
    let mut pos = v.iter().position(|x| *x == number).unwrap();
    if number > 0 {
        let mut i = number;
        while i > 0 {
            let mut nextpos = pos + 1;
            if nextpos == v.len() {  // Rolling off the right
                nextpos=0;
            }
            v.swap(pos,nextpos);
            pos = nextpos;
            i -= 1;
        }
    } else if number < 0 {
        let mut i = number.abs();
        while i > 0 {
            let nextpos: usize;
            if (pos == 0) {
                nextpos = v.len()-1;
            } else {
                nextpos = pos - 1 ;
            }
            v.swap(pos,nextpos);
            pos = nextpos;

            i -= 1;
        }
    } else {
        // Do nothing
    }
}


fn move_number_take2(v: &mut VecDeque<i32>,number: i32) {
    let mut pos = v.iter().position(|x| *x == number).unwrap();
    if number > 0 {
        let mut i = number;
        while i > 0 {
            let mut nextpos = pos + 1;
            if nextpos == v.len() {  // Rolling off the right
                assert!(number == v.pop_back().unwrap());
                v.insert(1,number);
                pos = 1;
            } else {
                v.swap(pos,nextpos);
                pos = nextpos;
            }
            i -= 1;
        }
    } else if number < 0 {
        let mut i = number.abs();
        while i > 0 {
            // if pos == 1 { // Rolling off the left is funky, jump zero pos and land on far-end
            //     v.remove(pos);
            //     v.push_back(number);
            //     pos = v.len()-1;
            // } else if pos == 0 { // Jump the end number (insert at it and shift it right)
            if (pos == 0) {
                assert!(number == v.pop_front().unwrap());
                v.insert(v.len()-1,number);
                pos = v.len()-2;
            } else {
                let nextpos = pos -1 ;
                v.swap(pos,nextpos);
                pos = nextpos
            }
            i -= 1;
        }
    } else {
        // Do nothing
    }
}


fn move_number_wrong(v: Vec<i32>,num: i32) -> Vec<i32> {

    let mut vc = v.clone();

    let pos = v.iter().rposition(|x| x == &num).unwrap();
    
    // let mut newpos = (pos as i32 + num).rem_euclid(v.len() as i32 - 1);


    let ms = v.len() as i32 - 1;
    let mut newpos = (pos as i32 + num);

    newpos = ((newpos % ms) + ms) % ms;


    // newpos = newpos.rem_euclid(v.len()as i32);
    assert!(newpos >= 0);
    assert!((newpos as usize) < v.len());
  
    // println!("Moving {} from {} to {} [{}]",num,pos,newpos, (newpos as usize) <pos);
    println!("Moving from {} to {}",pos,newpos);



    vc.remove(pos);
    vc.insert(newpos as usize,num);

    vc
}

fn calc_result(v: Vec<i32>) -> i32 {
    let zero_at = v.iter().position(|x| *x == 0).unwrap();
    let mut answer = Vec::new();

    for i in 1..=3 {
        answer.push((zero_at + (i*1000)) % v.len());
    }
    answer.iter().map(|x| v[*x]).sum()
}


fn part1(data: &str) -> i32 {
    let origlist: Vec<i32> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut mixlist: Vec<i32> = data
    .split('\n')
    .filter(|y| !y.is_empty())
    .map(|x| x.parse::<i32>().unwrap())
    .collect();

    // let mut mixlist = origlist.clone();

    for origval in origlist {
        mixlist = move_number_wrong( mixlist, origval);
        // println!("{:?} - Moved origval {}",mixlist,origval)
    }
    
    calc_result(mixlist)



}


fn main() {
    test();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 != -12134); // First wrong answer
    assert!(p1 != 18535); // Another wrong answer.
    assert!(p1 !=-13438);
    assert!(p1 !=-22100);
    assert!(p1 == 23321); // Finally :)
    
    
    // assert!(p1 == ??);
    // let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    // println!("Part2: {}", p2);
    // assert!(p2 == ??);
}
