use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

fn main() {
    let results: HashMap<Vec<i64>, u64> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.parse::<u64>()
                .expect("Not an unsigned integer that fits in 64 bits")
        })
        .fold(HashMap::new(), |mut acc, initial_n| {
            let prices: Vec<u64> = (0..2000)
                .scan(initial_n, |n, _idx| {
                    *n = ((*n * 64) ^ *n) % 16777216;
                    *n = ((*n / 32) ^ *n) % 16777216;
                    *n = ((*n * 2048) ^ *n) % 16777216;
                    Some(*n)
                })
                .map(|n| n % 10)
                .collect();
            let changes: Vec<i64> = prices
                .iter()
                .scan(initial_n % 10, |prev_n, curr_n| {
                    let change = *curr_n as i64 - *prev_n as i64;
                    *prev_n = *curr_n;
                    Some(change)
                })
                .collect();
            changes
                .windows(4)
                .zip(prices.into_iter().skip(3))
                .scan(HashSet::new(), |seen_changes, (window, price)| {
                    if seen_changes.contains(&window) {
                        return Some(None);
                    }
                    seen_changes.insert(window);
                    Some(Some((window, price)))
                })
                .flatten()
                .for_each(|(window, price)| {
                    acc.entry(window.to_vec()).or_insert(0).add_assign(price)
                });
            acc
        });
    let output = results.into_values().max().expect("Empty input");
    println!("{}", output);
}
