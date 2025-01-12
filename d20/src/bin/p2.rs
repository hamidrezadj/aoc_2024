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
        .flat_map(|center| {
            possible_skip_lengths_starting_from_this_point(&track, &shortest_path_tree, center)
        })
        .filter(|skip_len| *skip_len >= 100)
        .count();
    println!("{}", output);
}

fn possible_skip_lengths_starting_from_this_point(
    track: &[Vec<char>],
    shortest_path_tree: &HashMap<(usize, usize), u64>,
    center: (usize, usize),
) -> Vec<u64> {
    let sup_i = track.len();
    let sup_j = track.first().unwrap().len();
    let sizes = (sup_i, sup_j);
    let start_len = shortest_path_tree.get(&center).copied().unwrap();
    (2..=20)
        .flat_map(|radius| {
            diamond(center, radius, sizes)
                .into_iter()
                .map(|end_point| (radius, end_point))
                .collect::<Vec<_>>()
        })
        .filter(|(_radius, (ei, ej))| track[*ei][*ej] != '#')
        .flat_map(|(radius, end_pos)| {
            shortest_path_tree
                .get(&end_pos)
                .map(|end_len| (radius, end_len))
        })
        .filter(|(radius, end_len)| **end_len > start_len + radius)
        .map(|(radius, end_len)| end_len - start_len - radius)
        .collect::<Vec<_>>()
}

fn diamond(center: (usize, usize), radius: u64, sizes: (usize, usize)) -> Vec<(usize, usize)> {
    assert!(radius <= 20);
    let radius = radius as usize;
    let (i, j) = center;
    let (sup_i, sup_j) = sizes;
    let mut diamond: Vec<(usize, usize)> = (1..=radius - 1)
        .flat_map(|delta_i| {
            let delta_j = radius - delta_i;
            let right = j.checked_add(delta_j).filter(|j| *j < sup_j);
            let down = i.checked_add(delta_i).filter(|i| *i < sup_i);
            let left = j.checked_sub(delta_j);
            let up = i.checked_sub(delta_i);
            let down_right = down.zip(right);
            let down_left = down.zip(left);
            let up_left = up.zip(left);
            let up_right = up.zip(right);
            [down_right, down_left, up_left, up_right]
        })
        .flatten()
        .collect();
    let right = j.checked_add(radius).filter(|j| *j < sup_j).map(|j| (i, j));
    let down = i.checked_add(radius).filter(|i| *i < sup_i).map(|i| (i, j));
    let left = j.checked_sub(radius).map(|j| (i, j));
    let up = i.checked_sub(radius).map(|i| (i, j));
    let mut corners = vec![right, down, left, up].into_iter().flatten().collect();
    diamond.append(&mut corners);
    diamond
}
