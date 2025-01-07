use std::ops::ControlFlow;

fn main() {
    let mut corrupted_positions = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_once(',')
                .map(|(left, right)| {
                    (
                        left.parse::<u64>().expect("Bad left integer"),
                        right.parse::<u64>().expect("Bad right integer"),
                    )
                })
                .expect("No comma in line")
        });
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

    let initial_corrupted_trees: Vec<Vec<(u64, u64)>> = Default::default();
    let output = corrupted_positions
        .try_fold(
            initial_corrupted_trees,
            |mut corrupted_trees, corrupted_position| {
                let (i, j) = corrupted_position;
                let right = j.checked_add(1).filter(|j| *j < size).map(|j| (i, j));
                let down = i.checked_add(1).filter(|i| *i < size).map(|i| (i, j));
                let left = j.checked_sub(1).map(|j| (i, j));
                let up = i.checked_sub(1).map(|i| (i, j));
                let right_down = down.map(|(i, _j)| i).zip(right.map(|(_i, j)| j));
                let left_down = down.map(|(i, _j)| i).zip(left.map(|(_i, j)| j));
                let left_up = up.map(|(i, _j)| i).zip(left.map(|(_i, j)| j));
                let right_up = up.map(|(i, _j)| i).zip(right.map(|(_i, j)| j));
                let mut neighbouring_corrupted_trees_indexes: Vec<usize> = [
                    right, down, left, up, right_down, left_down, left_up, right_up,
                ]
                .into_iter()
                .flatten()
                .filter_map(|neighbouring_position| {
                    corrupted_trees
                        .iter()
                        .enumerate()
                        .find(|(_idx, corrupt_tree)| corrupt_tree.contains(&neighbouring_position))
                })
                .map(|(tree_index, _tree)| tree_index)
                .collect();
                neighbouring_corrupted_trees_indexes.sort();
                neighbouring_corrupted_trees_indexes.dedup();
                neighbouring_corrupted_trees_indexes.reverse();
                let mut new_corrupted_tree: Vec<(u64, u64)> = neighbouring_corrupted_trees_indexes
                    .into_iter()
                    .flat_map(|tree_index| corrupted_trees.remove(tree_index))
                    .collect();
                new_corrupted_tree.push(corrupted_position);
                if new_corrupted_tree
                    .iter()
                    .any(|(i, j)| *i == 0 || *j == size - 1)
                    && new_corrupted_tree
                        .iter()
                        .any(|(i, j)| *i == size - 1 || *j == 0)
                {
                    return ControlFlow::Break(corrupted_position);
                }
                corrupted_trees.push(new_corrupted_tree);
                ControlFlow::Continue(corrupted_trees)
            },
        )
        .break_value()
        .expect("No corrupt tile found that would isolate the start and end positions");
    println!("{},{}", output.0, output.1);
}
