use std::collections::{HashMap, HashSet};

fn main() {
    let input: Vec<Vec<u8>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string().parse::<u8>().expect("Not an ascii digit"))
                .collect()
        })
        .collect();
    assert!(!input.is_empty(), "Empty input");
    assert!(!input[0].is_empty(), "Empty line");
    assert!(
        input.iter().all(|row| row.len() == input[0].len()),
        "Uneven input"
    );
    let trail_heads: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_j, height)| **height == 0)
                .map(move |(j, _height)| (i, j))
        })
        .collect();
    let graph: HashMap<(usize, usize), Vec<(usize, usize)>> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, height)| ((i, j), *height))
        })
        .map(|((i, j), height)| {
            let top_node = i.checked_sub(1).map(|i| ((i, j), input[i][j]));
            let right_node = j
                .checked_add(1)
                .filter(|j| *j < input[0].len())
                .map(|j| ((i, j), input[i][j]));
            let bottom_node = i
                .checked_add(1)
                .filter(|i| *i < input.len())
                .map(|i| ((i, j), input[i][j]));
            let left_node = j.checked_sub(1).map(|j| ((i, j), input[i][j]));
            let valid_neighbouring_nodes = [top_node, right_node, bottom_node, left_node]
                .into_iter()
                .flatten()
                // height + 1 can't overflow since height is between 0 and 9.
                .filter(|(_n_pos, n_height)| *n_height == height + 1)
                .map(|(n_pos, _n_height)| n_pos)
                .collect::<Vec<(usize, usize)>>();
            ((i, j), valid_neighbouring_nodes)
        })
        .collect();
    let output = trail_heads
        .into_iter()
        .map(|head| {
            let mut next_nodes = vec![head];
            for _ in 0..9 {
                next_nodes = next_nodes
                    .into_iter()
                    .flat_map(|node| graph.get(&node))
                    .flatten()
                    .copied()
                    .collect();
            }
            next_nodes
                .into_iter()
                .collect::<HashSet<(usize, usize)>>()
                .len()
        })
        .sum::<usize>();
    println!("{}", output);
}
