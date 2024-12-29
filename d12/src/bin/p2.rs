use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct Edge;

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
    let regions: Vec<Vec<(usize, usize)>> = (0..=max_i)
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
        .collect();
    let areas: Vec<usize> = regions
        .iter()
        .map(|region_positions| region_positions.len())
        .collect();
    let sides = regions
        .into_iter()
        .map(|region_positions| {
            region_positions
                .iter()
                .flat_map(|(i, j)| {
                    let right = j
                        .checked_add(1)
                        .filter(|j| *j <= max_j)
                        .map(|nj| input[*i][nj] == input[*i][*j])
                        .or(Some(false))
                        .filter(|is_in_region| !*is_in_region)
                        .map(|_| (Direction::Right, (*j, *i)));
                    let down = i
                        .checked_add(1)
                        .filter(|i| *i <= max_i)
                        .map(|ni| input[ni][*j] == input[*i][*j])
                        .or(Some(false))
                        .filter(|is_in_region| !*is_in_region)
                        .map(|_| (Direction::Down, (*i, *j)));
                    let left = j
                        .checked_sub(1)
                        .map(|nj| input[*i][nj] == input[*i][*j])
                        .or(Some(false))
                        .filter(|is_in_region| !*is_in_region)
                        .map(|_| (Direction::Left, (*j, *i)));
                    let up = i
                        .checked_sub(1)
                        .map(|ni| input[ni][*j] == input[*i][*j])
                        .or(Some(false))
                        .filter(|is_in_region| !*is_in_region)
                        .map(|_| (Direction::Up, (*i, *j)));
                    [right, down, left, up].into_iter().flatten()
                })
                .collect::<BinaryHeap<(Direction, (usize, usize))>>()
        })
        .map(|region_periphery| {
            region_periphery
                .into_sorted_vec()
                .into_iter()
                .scan(None, |prev_peripheral_node, peripheral_node| {
                    let (dir, pos) = peripheral_node;
                    let edge = match prev_peripheral_node {
                        Some((p_dir, _pos)) if *p_dir != dir => Some(Edge),
                        Some((Direction::Right | Direction::Left, (p_j, p_i)))
                            if *p_j == pos.0 && pos.1.checked_sub(1).is_some_and(|i| *p_i == i) =>
                        {
                            None
                        }
                        Some((Direction::Down | Direction::Up, (p_i, p_j)))
                            if *p_i == pos.0 && pos.1.checked_sub(1).is_some_and(|j| *p_j == j) =>
                        {
                            None
                        }
                        _ => Some(Edge),
                    };
                    *prev_peripheral_node = Some((dir, pos));
                    Some(edge)
                })
                .flatten()
                .count()
        });
    let output = areas
        .into_iter()
        .zip(sides)
        .map(|(side, area)| side.checked_mul(area).expect("Overflow"))
        .reduce(|acc, p| acc.checked_add(p).expect("Overflow"))
        .unwrap();
    println!("{}", output);
}
