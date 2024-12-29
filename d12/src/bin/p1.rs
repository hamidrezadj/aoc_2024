use std::collections::HashMap;

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
        "Uneven input: Not a rectangle"
    );
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;
    let graph: HashMap<(usize, usize), Vec<(usize, usize)>> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, plant)| ((i, j), *plant))
        })
        .map(|((i, j), plant)| {
            let right_plant = j
                .checked_add(1)
                .filter(|j| *j <= max_j)
                .map(|j| ((i, j), input[i][j]));
            let bottom_plant = i
                .checked_add(1)
                .filter(|i| *i <= max_i)
                .map(|i| ((i, j), input[i][j]));
            let left_plant = j.checked_sub(1).map(|j| ((i, j), input[i][j]));
            let top_plant = i.checked_sub(1).map(|i| ((i, j), input[i][j]));
            let regional_neighbours = [right_plant, bottom_plant, left_plant, top_plant]
                .into_iter()
                .flatten()
                .filter(|(_pos, n_plant)| *n_plant == plant)
                .map(|(pos, _plant)| pos)
                .collect::<Vec<(usize, usize)>>();
            ((i, j), regional_neighbours)
        })
        .collect();
    let initial_visited: Vec<Vec<bool>> = vec![vec![false; max_j + 1]; max_i + 1];
    let output = (0..=max_i)
        .flat_map(|i| (0..=max_j).map(move |j| (i, j)))
        .scan(initial_visited, |visited, (i, j)| {
            if visited[i][j] {
                return Some(None);
            }
            visited[i][j] = true;
            let mut region_positions = vec![(i, j)];
            let mut neighbours = graph.get(&(i, j)).unwrap().clone();
            while let Some((n_i, n_j)) = neighbours.pop() {
                if visited[n_i][n_j] {
                    continue;
                }
                visited[n_i][n_j] = true;
                region_positions.push((n_i, n_j));
                neighbours.append(&mut graph.get(&(n_i, n_j)).unwrap().clone());
            }
            Some(Some(region_positions))
        })
        .flatten()
        .map(|region_positions| {
            region_positions
                .into_iter()
                .map(|position| {
                    let area = 1;
                    let perimeter = graph
                        .get(&position)
                        .map(|neighbours| 4 - neighbours.len())
                        .unwrap();
                    (area, perimeter)
                })
                .fold(
                    (0usize, 0usize),
                    |(area_acc, perimeter_acc), (area, perimeter)| {
                        (
                            area_acc.checked_add(area).expect("Overflow"),
                            perimeter_acc.checked_add(perimeter).expect("Overflow"),
                        )
                    },
                )
        })
        .map(|(area, perimeter)| area.checked_mul(perimeter).expect("Overflow"))
        .sum::<usize>();
    println!("{}", output);
}
