use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| line.chars().collect())
        .collect();
    assert!(!input.is_empty(), "Empty input");
    assert!(!input[0].is_empty(), "Empty line");
    assert!(
        input.iter().all(|row| row.len() == input[0].len()),
        "Uneven input"
    );
    let max_i = input.len() as i64 - 1;
    let max_j = input[0].len() as i64 - 1;
    let is_in_bound = |(i, j)| i >= 0 && i <= max_i && j >= 0 && j <= max_j;
    let antennas: HashMap<char, Vec<(i64, i64)>> = input
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(j, ch)| (ch, (i as i64, j as i64)))
        })
        .filter(|(ch, _pos)| ch.is_alphanumeric())
        .fold(HashMap::new(), |mut acc, (ch, pos)| {
            acc.entry(ch).or_default().push(pos);
            acc
        });
    let anti_nodes: HashSet<(i64, i64)> = antennas
        .into_values()
        .flat_map(combinations_of_two)
        .flat_map(|((a1i, a1j), (a2i, a2j))| {
            let diff_i = a1i - a2i;
            let diff_j = a1j - a2j;
            let left_anti_nodes = (0..)
                .map(move |multiplier| (a1i + multiplier * diff_i, a1j + multiplier * diff_j))
                .take_while(|anti_node| is_in_bound(*anti_node));
            let right_anti_nodes = (1..)
                .map(|m| -m)
                .map(move |m| (a1i + m * diff_i, a1j + m * diff_j))
                .take_while(|anti_node| is_in_bound(*anti_node));
            left_anti_nodes.chain(right_anti_nodes)
        })
        .collect();
    let output = anti_nodes.len();
    println!("{}", output);
}

fn combinations_of_two(positions: Vec<(i64, i64)>) -> Vec<((i64, i64), (i64, i64))> {
    let mut combinations = Vec::new();
    for i in 0..positions.len() {
        for j in (i + 1)..positions.len() {
            combinations.push((positions[i], positions[j]));
        }
    }
    combinations
}
