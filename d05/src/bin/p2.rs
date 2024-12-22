use std::{
    collections::{HashMap, HashSet},
    ops::ControlFlow,
};

fn main() {
    let ordering_rules: HashMap<u32, HashSet<u32>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut splits = line.split('|').map(|word| {
                word.parse::<u32>()
                    .expect("Not an unsigned integer that fits in 32 bits")
            });
            let lhs_number = splits.next().expect("Less than one number in rule");
            let rhs_number = splits.next().expect("Less than two numbers in rule");
            if splits.next().is_some() {
                panic!("More than two numbers in rule")
            }
            (lhs_number, rhs_number)
        })
        .fold(HashMap::new(), |mut acc, (lhs, rhs)| {
            acc.entry(lhs).or_default().insert(rhs);
            acc
        });
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split(',')
                .map(|word| {
                    word.parse::<u32>()
                        .expect("Not an unsigned integer that fits in 32 bits")
                })
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            line.iter()
                .try_fold(Vec::new(), |mut acc, num| {
                    let related_rules = ordering_rules.get(num);
                    if related_rules.is_none() {
                        acc.push(num);
                        return ControlFlow::Continue(acc);
                    }
                    let related_rules = related_rules.unwrap();
                    if acc.iter().any(|prev| related_rules.contains(prev)) {
                        return ControlFlow::Break(());
                    }
                    acc.push(num);
                    ControlFlow::Continue(acc)
                })
                .is_break()
        })
        .map(|line| {
            line.iter().fold(Vec::new(), |mut acc, num| {
                let related_rules = ordering_rules.get(num);
                if related_rules.is_none() {
                    acc.push(num.to_owned());
                    return acc;
                }
                let related_rules = related_rules.unwrap();
                let ordering_result = acc.iter().enumerate().try_for_each(|(idx, prev)| {
                    if related_rules.contains(prev) {
                        ControlFlow::Break(idx)
                    } else {
                        ControlFlow::Continue(())
                    }
                });
                match ordering_result {
                    ControlFlow::Continue(()) => {
                        acc.push(num.to_owned());
                    }
                    ControlFlow::Break(idx) => {
                        acc.insert(idx, num.to_owned());
                    }
                }
                acc
            })
        })
        .filter(|line| {
            line.iter()
                .try_fold(Vec::new(), |mut acc, num| {
                    let related_rules = ordering_rules.get(num);
                    if related_rules.is_none() {
                        acc.push(num);
                        return ControlFlow::Continue(acc);
                    }
                    let related_rules = related_rules.unwrap();
                    if acc.iter().any(|prev| related_rules.contains(prev)) {
                        return ControlFlow::Break(());
                    }
                    acc.push(num);
                    ControlFlow::Continue(acc)
                })
                .is_continue()
        })
        .inspect(|line| {
            if line.is_empty() {
                panic!("Empty line")
            }
        })
        .inspect(|line| {
            if line.len() % 2 == 0 {
                panic!("Even number count")
            }
        })
        .map(|line| line[line.len() / 2])
        .sum::<u32>();
    println!("{}", output);
}
