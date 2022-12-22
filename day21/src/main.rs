use std::collections::HashMap;
use std::time::Instant;

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 152
    );
}

#[derive(Debug, Clone)]
struct Expression {
    lhs: String,
    rhs: String,
    operand: char,
}

impl Expression {
    fn from_str(s: &str) -> Self {
        let ss: Vec<&str> = s.trim().split(' ').collect();
        assert!(ss.len() == 3);
        assert!(ss[1].len() == 1);
        Self {
            lhs: ss[0].to_string(),
            operand: ss[1].chars().next().unwrap(), // first...
            rhs: ss[2].to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum MonkeyValue {
    Expression(Expression),
    Value(i64),
}

impl MonkeyValue {
    fn eval(&self, others: &HashMap<&str, Self>) -> i64 {
        if let MonkeyValue::Value(val) = self {
            *val
        } else if let MonkeyValue::Expression(e) = self {
            let lexpr = others.get(e.lhs.as_str()).unwrap();
            let rexpr = others.get(e.rhs.as_str()).unwrap();
            match e.operand {
                '+' => Self::eval(lexpr, others) + Self::eval(rexpr, others),
                '-' => Self::eval(lexpr, others) - Self::eval(rexpr, others),
                '*' => Self::eval(lexpr, others) * Self::eval(rexpr, others),
                '/' => Self::eval(lexpr, others) / Self::eval(rexpr, others),
                _ => std::i64::MAX,
            }
        } else {
            panic!("Our enum is very broken...");
        }
    }
}

fn part1(data: &str) -> i64 {
    let monkeys: HashMap<_, _> = data
        .split('\n')
        .filter(|y| !y.is_empty())
        .map(|x| {
            let s: Vec<&str> = x.split(':').collect();
            match s[1].trim().parse::<i64>() {
                Ok(x) => (s[0], MonkeyValue::Value(x)),
                Err(_) => (s[0], MonkeyValue::Expression(Expression::from_str(s[1]))),
            }
        })
        .collect();

    monkeys.get("root").unwrap().eval(&monkeys)
}

fn main() {
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    // assert!(p1 == ?);
    // let p2 = part1(
    //     std::fs::read_to_string("input.txt").unwrap().as_str(),
    // );
    // println!("Part2: {}", p2);
    // assert!(p2 == ?);
    println!("Completed in {} us", now.elapsed().as_micros());
}
