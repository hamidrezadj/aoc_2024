use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    ops::Not,
};

fn main() {
    let track: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| line.chars().collect())
        .collect();
    assert!(track.is_empty().not(), "Empty track");
    assert!(track.first().unwrap().is_empty().not(), "Empty row");
    assert!(
        track
            .iter()
            .all(|row| row.len() == track.first().unwrap().len()),
        "Uneven track: Not a rectangle"
    );
    assert!(
        track.iter().flatten().all(|ch| ".#SE".contains(*ch)),
        "Invalid character in track"
    );
    assert_eq!(
        track.iter().flatten().filter(|ch| **ch == 'S').count(),
        1,
        "Less or more than one 'S' character in track"
    );
    assert_eq!(
        track.iter().flatten().filter(|ch| **ch == 'E').count(),
        1,
        "Less or more than one 'E' character in track"
    );
    assert!(
        track.first().unwrap().iter().all(|ch| *ch == '#'),
        "Upper wall not complete"
    );
    assert!(
        track.last().unwrap().iter().all(|ch| *ch == '#'),
        "Lower wall not complete"
    );
    assert!(
        track
            .iter()
            .map(|row| row.first().unwrap())
            .all(|ch| *ch == '#'),
        "Left wall not complete"
    );
    assert!(
        track
            .iter()
            .map(|row| row.last().unwrap())
            .all(|ch| *ch == '#'),
        "Right wall not complete"
    );
    let starting_position = (0..track.len())
        .flat_map(|i| (0..track[0].len()).map(move |j| (i, j)))
        .find(|(i, j)| track[*i][*j] == 'S')
        .unwrap();
    let mut shortest_path_tree: HashMap<(usize, usize), u64> = Default::default();
    let mut sp_tree_neighbours: BinaryHeap<Reverse<(u64, (usize, usize))>> =
        BinaryHeap::from([Reverse((0, starting_position))]);
    while let Some(Reverse((length, (i, j)))) = sp_tree_neighbours.pop() {
        match shortest_path_tree.entry((i, j)) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(length);
            }
        }
        let mut neighbours = {
            let right = j
                .checked_add(1)
                .filter(|j| *j < track.first().unwrap().len())
                .map(|j| (i, j));
            let down = i
                .checked_add(1)
                .filter(|i| *i < track.len())
                .map(|i| (i, j));
            let left = j.checked_sub(1).map(|j| (i, j));
            let up = i.checked_sub(1).map(|i| (i, j));
            [right, down, left, up]
                .into_iter()
                .flatten()
                .filter(|(i, j)| track[*i][*j] != '#')
                .map(|pos| (length + 1, pos))
                .map(Reverse)
                .collect::<BinaryHeap<_>>()
        };
        sp_tree_neighbours.append(&mut neighbours);
    }
    let output = shortest_path_tree
        .keys()
        .copied()
        .flat_map(|(i, j)| {
            let right = j
                .checked_add(2)
                .filter(|j| *j < track.first().unwrap().len())
                .map(|j| (i, j));
            let down = i
                .checked_add(2)
                .filter(|i| *i < track.len())
                .map(|i| (i, j));
            let left = j.checked_sub(2).map(|j| (i, j));
            let up = i.checked_sub(2).map(|i| (i, j));
            let down_right = i.checked_add(1).filter(|i| *i < track.len()).zip(
                j.checked_add(1)
                    .filter(|j| *j < track.first().unwrap().len()),
            );
            let down_left = i
                .checked_add(1)
                .filter(|i| *i < track.len())
                .zip(j.checked_sub(1));
            let up_left = i.checked_sub(1).zip(j.checked_sub(1));
            let up_right = i.checked_sub(1).zip(
                j.checked_add(1)
                    .filter(|j| *j < track.first().unwrap().len()),
            );
            let start_len = shortest_path_tree.get(&(i, j)).copied().unwrap();
            [
                right, down, left, up, down_right, down_left, up_left, up_right,
            ]
            .into_iter()
            .flatten()
            .filter(|(i, j)| track[*i][*j] != '#')
            .flat_map(|pos| shortest_path_tree.get(&pos))
            .filter(|end_len| **end_len > start_len + 2)
            .map(|end_len| end_len - start_len - 2)
            .collect::<Vec<_>>()
        })
        .filter(|skip_len| *skip_len >= 100)
        .count();
    println!("{}", output);
}
