use std::collections::HashMap;

fn main() {
    // A map from n to count of n.
    let mut input: HashMap<u64, u64> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<u64>()
                        .expect("Not an unsigned integer that fits in 64 bits")
                })
                .collect::<Vec<u64>>()
        })
        .fold(HashMap::new(), |mut acc, n| {
            let entry = acc.entry(n).or_insert(0);
            *entry += 1;
            acc
        });
    for _ in 0..75 {
        input = input
            .into_iter()
            .flat_map(|(n, count)| match n {
                0 => vec![(1, count)],
                n if n.to_string().len() % 2 == 0 => {
                    let n = n.to_string();
                    let (n1, n2) = n.split_at(n.len() / 2);
                    let n1 = n1.parse::<u64>().unwrap();
                    let n2 = n2.parse::<u64>().unwrap();
                    vec![(n1, count), (n2, count)]
                }
                n => vec![(n.checked_mul(2024).expect("Overflow"), count)],
            })
            .fold(HashMap::new(), |mut acc, (n, count)| {
                let entry = acc.entry(n).or_insert(0);
                *entry += count;
                acc
            });
    }
    let output = input.into_values().sum::<u64>();
    println!("{}", output);
}
