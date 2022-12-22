use eqsolver::single_variable::Secant;
use std::collections::HashMap;
use std::time::Instant;

// Part2 - Didn't make my unwinding function work for the proper data, even though it did for the sample data... - so just solved the linear equation I had already derived,
// Not proud...
// This totally blows up rustfmt also ;-)

fn solve2more() -> f64 {
    // This is the equation/target printed out by a previous run
    let f = |humn: f64| ((886f64+(2f64*(117205375899188f64-(3f64*(((((((((338f64+(((5f64*(995f64+(((((2f64*(((((694f64+((7f64+(((5f64*(((858f64+(((815f64+(((((2f64*(282f64+((528f64+(((4f64*(((((2f64*((((((2f64*(867f64+(((21f64*(452f64+humn))-886f64)/2f64)))-513f64)/5f64)+859f64)/2f64)-153f64))-727f64)+677f64)/2f64)+343f64))-287f64)*4f64))/4f64)))-852f64)/10f64)-542f64)*17f64))/2f64)+922f64))/7f64)-854f64))-175f64)/5f64))*2f64))*9f64)-972f64)/3f64)+51f64))-850f64)/2f64)-388f64)/2f64)))-171f64)*2f64))/2f64)+853f64)/3f64)-789f64)*2f64)-118f64)/3f64)+155f64)))))/4f64)-23622695042414f64;
    let solution = Secant::new(f).solve(99999999.9, 500000.0);
    if let Ok(result) = solution {
        result
    } else {
        panic!("Unable to solve equation")
    }
}

fn test() {
    debug_assert!(
        part1(
            std::fs::read_to_string("input_sample.txt")
                .unwrap()
                .as_str(),
        ) == 152
    );
    debug_assert!(part2(std::fs::read_to_string("input.txt").unwrap().as_str(),) == 301);
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

    // If the 2 children are numbers... smoosh them up.
    fn simplify(&self, others: &HashMap<&str, Self>) -> Option<MonkeyValue> {
        if let MonkeyValue::Value(_) = self {
            // Already completely simplified :)
            None
        } else if let MonkeyValue::Expression(e) = self {
            if e.lhs.contains("humn") || e.rhs.contains("humn") {
                // Cannot simplify humans...
                None
            } else {
                let lchild = others.get(e.lhs.as_str()).unwrap();
                let rchild = others.get(e.rhs.as_str()).unwrap();

                if let MonkeyValue::Value(lval) = lchild {
                    if let MonkeyValue::Value(rval) = rchild {
                        // we have actual values - and something to simplify.
                        match e.operand {
                            '+' => Some(MonkeyValue::Value(lval + rval)),
                            '-' => Some(MonkeyValue::Value(lval - rval)),
                            '*' => Some(MonkeyValue::Value(lval * rval)),
                            '/' => Some(MonkeyValue::Value(lval / rval)),
                            _ => None,
                        }
                    } else {
                        // Do nothing (catch in another pass..)
                        None
                    }
                } else {
                    // Do nothing (catch in another pass..)
                    None
                }
            }
        } else {
            panic!("Our enum is very broken...");
        }
    }

    // fn expand(&self, others: &HashMap<&str, Self>) -> &str {
    //     if let MonkeyValue::Value(val) = self {
    //         val.to_
    //     } else if let MonkeyValue::Expression(e) = self {
    //         let lexpr = others.get(e.lhs.as_str()).unwrap();
    //         let rexpr = others.get(e.rhs.as_str()).unwrap();
    //         match e.operand {
    //             '+' => Self::eval(lexpr, others) + Self::eval(rexpr, others),
    //             '-' => Self::eval(lexpr, others) - Self::eval(rexpr, others),
    //             '*' => Self::eval(lexpr, others) * Self::eval(rexpr, others),
    //             '/' => Self::eval(lexpr, others) / Self::eval(rexpr, others),
    //             _ => std::i64::MAX,
    //         }
    //     } else {
    //         panic!("Our enum is very broken...");
    //     }

    //     "malc"
    // }
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

// Failed attempt to reverse stack - worked on sample - not on real...
// fn print_expr2(root: &str, h: &HashMap<&str, MonkeyValue>, target: i64) -> String {
//     if root.contains("humn") {
//         String::from(root)
//     } else if let MonkeyValue::Value(mv) = h.get(root).unwrap() {
//         mv.to_string()
//     } else if let MonkeyValue::Expression(me) = h.get(root).unwrap() {
//         let left = print_expr(me.lhs.as_str(), h);
//         let right = print_expr(me.rhs.as_str(), h);

//         let mut newtarget = 0;
//         if left.contains("humn") {
//             // Right is a value - apply reverse operation to target
//             newtarget = match me.operand {
//                 '+' => target - right.parse::<i64>().unwrap(),
//                 '-' => target + right.parse::<i64>().unwrap(),
//                 '*' => target / right.parse::<i64>().unwrap(),
//                 '/' => target * right.parse::<i64>().unwrap(),
//                 _ => {
//                     panic!("Foo");
//                 }
//             };
//             println!(
//                 "Right: {}, Operand: {}, NT: {}",
//                 right, me.operand, newtarget
//             );
//             print_expr2(me.lhs.as_str(), h, newtarget);
//         } else if right.contains("humn") {
//             // Left is a value - apply reverse operation to target
//             newtarget = match me.operand {
//                 '+' => target - left.parse::<i64>().unwrap(),
//                 '-' => target + left.parse::<i64>().unwrap(),
//                 '*' => target / left.parse::<i64>().unwrap(),
//                 '/' => {
//                     panic!("Probably an error");
//                     // target * left.parse::<i64>().unwrap()
//                 }
//                 _ => {
//                     panic!("Foo");
//                 }
//             };
//             println!("Left: {}, Operand: {}, NT: {}", left, me.operand, newtarget);
//             print_expr2(me.rhs.as_str(), h, newtarget);
//         }
//         println!("newtarget = {} (should be 3429411069028)", newtarget);
//         assert!(newtarget == 3429411069028); // per online equation solver of my previous output.
//         panic!(
//             "Answer Is: {} should be 3429411069028 (these are wrong 8061312057692, ...)",
//             newtarget
//         );  
//     } else {
//         String::from("FAIL")
//     }
// }

fn print_expr(root: &str, h: &HashMap<&str, MonkeyValue>) -> String {
    if root.contains("humn") {
        String::from(root)
    } else if let MonkeyValue::Value(mv) = h.get(root).unwrap() {
        mv.to_string() + "f64"
    } else if let MonkeyValue::Expression(me) = h.get(root).unwrap() {
        return String::from("(") + print_expr(me.lhs.as_str(),h).as_str() 
         + me.operand.to_string().as_str() // horrible :)
         + print_expr(me.rhs.as_str(),h).as_str()
        + ")";
    } else {
        String::from("FAIL")
    }
}

fn part2(data: &str) -> i64 {
    let mut monkeys: HashMap<_, _> = data
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

    let mut mykeys = Vec::new();
    for k in monkeys.keys() {
        mykeys.push(*k);
    }

    let mut updates;
    while {
        updates = 0;
        for k in &mykeys {
            let r = monkeys.get(k).unwrap().simplify(&monkeys);
            if let Some(v) = r {
                println!("Updated {} with {:?}", k, v);
                monkeys.insert(k, v);
                updates += 1;
            }
        }
        println!("Updated {}", updates);
        updates > 0
    } {}

    if let MonkeyValue::Expression(root) = monkeys.get("root").unwrap() {
        let lexpr = print_expr(root.lhs.as_str(), &monkeys);
        let rexpr = print_expr(root.rhs.as_str(), &monkeys);
        println!("LEFT-RIGHT==0 (feed me to eqn solver): {}-{}", lexpr,rexpr);

        // if lexpr.contains("humn") {
        //     let target = monkeys.get(root.rhs.as_str()).unwrap().eval(&monkeys);
        //     println!("RCALC: {}", target);
        //     println!("LREV: {}", print_expr2(root.lhs.as_str(), &monkeys, target));
        // } else {
        //     let target = monkeys.get(root.lhs.as_str()).unwrap().eval(&monkeys);
        //     println!("LCALC: {}", target);
        //     println!("RREV: {}", print_expr2(root.rhs.as_str(), &monkeys, target));
        // }
    }

    // if let MonkeyValue::Expression(root) = monkeys.get("root").unwrap() {
    //     monkeys.get(root.lhs.as_str()).unwrap().simplify(&monkeys);
    // }
    // println!("{:?}",monkeys);
    // println!("{}",print_expr("root",&monkeys));

    0
}

fn main() {
    
    test();
    let now = Instant::now();
    let p1 = part1(std::fs::read_to_string("input.txt").unwrap().as_str());
    println!("Part1: {}", p1);
    assert!(p1 == 82225382988628);
    // let p2 = part2(std::fs::read_to_string("input.txt").unwrap().as_str());
    let p2 = solve2more();
    println!("Part2: {}", p2);
    assert!(p2 as i64 == 3429411069028);
    println!("Completed in {} us", now.elapsed().as_micros());
}
