use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Node {
    Leaf(i32),
    Inner(Box<Vec<Node>>),
}

// #[derive(Debug,PartialEq)]
// struct Packet {
//     node: Node,
// }
type Packet = Node;

// If the thing is a number - append it as a leaf to the deepest array
fn append_number(num: Option<i32>, parsetree: &mut Vec<Vec<Node>>) -> Option<i32> {
    let lev = parsetree.len() - 1;
    if let Some(num) = num {
        parsetree[lev].push(Node::Leaf(num));
    }
    None
}

impl Packet {
    fn from_string(s: &str) -> Self {
        let mut num = None;
        let mut parsetree = vec![vec![]];

        // Loop thru chars, each time we see a new list, push a new whole empty vector onto the tree
        // each time we see a number, stash it
        // each time we see a comma - push the stashed number the deepest vector (level)
        // each time we close a bracket, push any stashed number, then, pop the deepest vector, and create new Node::Inner(v) of it to append to the new-deepest vector.

        for c in s.chars() {
            // println!("char {} num is {:?} vecs was {:?}",c,num,parsetree);
            match c {
                '[' => parsetree.push(vec![]), // open a new list
                ']' => {
                    num = append_number(num, &mut parsetree);
                    let v = parsetree.pop().unwrap();
                    let lev = parsetree.len();
                    parsetree[lev - 1].push(Node::Inner(Box::new(v)));
                }
                ' ' => {}
                ',' => num = append_number(num, &mut parsetree),
                // THere are 2 digit ints in the *true* input (but not the test-data, you fuckers...)
                digit => num = Some(num.unwrap_or(0) * 10 + (digit.to_digit(10).unwrap()) as i32),
            };
        }
        Node::Inner(Box::new(parsetree.pop().unwrap()))
    }

    fn compare(&self, other: &Node) -> Ordering {
        match (self, other) {
            (Node::Leaf(left_val), Node::Leaf(right_val)) => (*left_val).cmp(right_val), // Simple int compare
            (Node::Inner(left_list), Node::Inner(right_list)) => {
                let mut i = 0;
                while i < left_list.len() && i < right_list.len() {
                    match left_list[i].compare(&right_list[i]) {
                        Ordering::Equal => {}
                        other => return other, // Less or Greater => Good or Bad.
                    };
                    i += 1;
                }
                left_list.len().cmp(&right_list.len()) // If we get here - one list is exhausted, left-done=good, right-done=bad.
            }
            (l, Node::Leaf(v)) => l.compare(&Node::Inner(Box::new(vec![Node::Leaf(*v)]))), // Left is vec, right is int, wrap int and compare
            (Node::Leaf(v), l) => Node::Inner(Box::new(vec![Node::Leaf(*v)])).compare(&l), // ^^ opposite
        }
    }
}

fn test() {
    let mut p1 = Packet::from_string("[[1],[2,3,4]]");
    println!("{:?}", p1);

    let mut p2 = Packet::from_string("[[1],4]");
    println!("{:?}", p2);
    let r = p1.compare(&p2);
    println!("{:?}", r);
    debug_assert!(r == Ordering::Less);

    p1 = Packet::from_string("[9]");
    p2 = Packet::from_string("[[8,7,6]]");
    debug_assert!(p1.compare(&p2) == Ordering::Greater);

    debug_assert!(
        13 == part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
                .into()
        )
    );
    debug_assert!(
        140 == part2(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str()
                .into()
        )
    );
}
fn part1(data: &str) -> u32 {
    let nodes = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| Packet::from_string(line))
        .collect::<Vec<_>>();

    // println!("{:?}",nodes);

    let mut i = 0;
    let mut result = 0;
    for pair in nodes.chunks(2) {
        i += 1;
        let r = pair[0].compare(&pair[1]);
        // println!("{} {:?}",i,r);
        if r == Ordering::Less {
            result += i;
        }
    }
    println!("Part1: {}", result);
    result
}

fn part2(data: &str) -> usize {
    let mut nodes = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(Packet::from_string)
        .collect::<Vec<_>>();

    nodes.append(&mut vec![
        Packet::from_string("[[6]]"),
        Packet::from_string("[[2]]"),
    ]);
    nodes.sort_by(|a, b| a.compare(&b));

    let looking_for = vec![Packet::from_string("[[6]]"), Packet::from_string("[[2]]")];
    let keys = (0..nodes.len())
        .filter(|&x| looking_for.contains(&nodes[x]))
        .map(|i| i + 1)
        .collect::<Vec<usize>>();

    assert!(keys.len() == 2);

    println!("Part2: {} * {} = {}", keys[0], keys[1], keys[0] * keys[1]);
    keys[0] * keys[1]
}

fn main() {
    test();
    // for file in ["input_sample.txt", "input.txt"] {
    assert!(part1(std::fs::read_to_string("input.txt").unwrap().as_str()) == 6656);
    assert!(part2(std::fs::read_to_string("input.txt").unwrap().as_str()) == 19716);
}
