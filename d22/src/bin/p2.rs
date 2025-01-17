use itertools::Itertools;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
    sync::mpsc,
};

fn main() {
    let input: Vec<i64> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.parse::<u64>()
                .expect("Not an unsigned integer that fits in 64 bits") as i64
        })
        .collect();
    let (sender, receiver) = mpsc::channel();
    input.into_par_iter().for_each(|initial_n| {
        (0..2000)
            .scan(initial_n, |n, _idx| {
                *n = ((*n * 64) ^ *n) % 16777216;
                *n = ((*n / 32) ^ *n) % 16777216;
                *n = ((*n * 2048) ^ *n) % 16777216;
                Some(*n)
            })
            .map(|n| n % 10)
            .scan(initial_n % 10, |prev_price, curr_price| {
                let change = *prev_price - curr_price;
                *prev_price = curr_price;
                Some((change, curr_price))
            })
            .tuple_windows()
            .map(|((c1, _), (c2, _), (c3, _), (c4, price))| ([c1, c2, c3, c4], price))
            .scan(HashSet::new(), |seen_windows, (window, price)| {
                let was_not_seen_before = seen_windows.insert(window);
                if was_not_seen_before {
                    Some(Some((window, price)))
                } else {
                    Some(None)
                }
            })
            .flatten()
            .for_each(|(window, price)| {
                sender.send((window, price)).unwrap();
            });
    });
    drop(sender);
    let output = receiver
        .iter()
        .fold(HashMap::new(), |mut acc, (window, price)| {
            acc.entry(window).or_insert(0).add_assign(price);
            acc
        })
        .into_values()
        .max()
        .expect("Empty input");
    println!("{}", output);
}
