use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet},
    ops::Not,
};

type Length = u64;
type Position = (u64, u64);

fn main() {
    let input: Vec<(u64, u64)> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_once(',')
                .map(|(left, right)| {
                    (
                        left.parse().expect("Bad left integer"),
                        right.parse().expect("Bad right integer"),
                    )
                })
                .expect("No comma in line")
        })
        .collect();
    let mut args = std::env::args().skip(1);
    let size: u64 = args
        .next()
        .map(|a| a.parse().expect("Bad integer as size argument"))
        .inspect(|size| {
            if *size == 0 {
                panic!("Size set to zero, there will be no bottom right corner.")
            }
        })
        .unwrap_or(71);
    let time: usize = args
        .next()
        .map(|a| a.parse().expect("Bad integer as size argument"))
        .unwrap_or(1024);

    let corrupted: HashSet<Position> = input.into_iter().take(time).collect();
    // Shortest path from the starting position (0, 0) to every other position.
    let mut shortest_path_tree: HashMap<Position, Length> = Default::default();
    // Neighbours to the shortest path tree,
    // sorted by their length from starting position,
    // in descending order.
    let mut sp_tree_neighbours: BinaryHeap<Reverse<(Length, Position)>> =
        BinaryHeap::from([Reverse((0, (0, 0)))]);
    while let Some(Reverse((length, position))) = sp_tree_neighbours.pop() {
        match shortest_path_tree.entry(position) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(length);
            }
        }
        let mut neighbours = {
            // Since we have symmetry the order doesn't matter.
            let (i, j) = position;
            let right = j.checked_add(1).filter(|j| *j < size).map(|j| (i, j));
            let down = i.checked_add(1).filter(|i| *i < size).map(|i| (i, j));
            let left = j.checked_sub(1).map(|j| (i, j));
            let up = i.checked_sub(1).map(|i| (i, j));
            [right, down, left, up]
                .into_iter()
                .flatten()
                .filter(|p| corrupted.contains(p).not())
                .map(|p| (length + 1, p))
                .map(Reverse)
                .collect()
        };
        sp_tree_neighbours.append(&mut neighbours);
    }

    let output = shortest_path_tree
        .get(&(size - 1, size - 1))
        .expect("Couldn't reach the bottom right corner");
    println!("{}", output);
}
