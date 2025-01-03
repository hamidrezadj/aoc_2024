use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    ops::Not,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .flat_map(|ch| match ch {
                    '#' => ['#', '#'],
                    'O' => ['[', ']'],
                    '.' => ['.', '.'],
                    '@' => ['@', '.'],
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();
    assert!(map.is_empty().not(), "Empty input");
    assert!(map.first().unwrap().is_empty().not(), "Empty line");
    assert!(
        map.iter()
            .all(|row| row.len() == map.first().unwrap().len()),
        "Uneven input: Not a rectangle"
    );
    assert_eq!(
        map.iter().flatten().filter(|ch| **ch == '@').count(),
        1,
        "Less or more than one '@' character in map section of input"
    );
    assert!(
        map.first().unwrap().iter().all(|ch| *ch == '#'),
        "Upper wall not complete"
    );
    assert!(
        map.last().unwrap().iter().all(|ch| *ch == '#'),
        "Lower wall not complete"
    );
    assert!(
        map.iter()
            .map(|row| row.first().unwrap())
            .all(|ch| *ch == '#'),
        "Left wall not complete"
    );
    assert!(
        map.iter()
            .map(|row| row.last().unwrap())
            .all(|ch| *ch == '#'),
        "Right wall not complete"
    );
    let instructions: Vec<Direction> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    _ => panic!("Invalid instruction character"),
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let initial_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, ch)| (i, j, *ch)))
        .find(|(_i, _j, ch)| *ch == '@')
        .map(|(i, j, _ch)| (i, j))
        .unwrap();
    let mut map = map;
    let mut position = initial_position;
    for direction in instructions {
        let tree = dependency_tree(&map, position, direction);
        if is_tree_movable(&map, &tree, direction) {
            move_tree(&mut map, tree, direction);
            match direction {
                Direction::Right => position.1 += 1,
                Direction::Down => position.0 += 1,
                Direction::Left => position.1 -= 1,
                Direction::Up => position.0 -= 1,
            }
        }
    }
    let output = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, ch)| (i, j, *ch)))
        .filter(|(_i, _j, ch)| *ch == '[')
        .map(|(i, j, _ch)| (i, j))
        .map(|(i, j)| i * 100 + j)
        .sum::<usize>();
    println!("{}", output);
}

fn move_tree(map: &mut [Vec<char>], mut tree: Vec<(usize, usize)>, direction: Direction) {
    match direction {
        Direction::Right => {
            tree.sort();
            tree.into_iter().rev().for_each(|(i, j)| {
                map[i][j + 1] = map[i][j];
                map[i][j] = '.';
            })
        }
        Direction::Left => {
            tree.sort();
            tree.into_iter().for_each(|(i, j)| {
                map[i][j - 1] = map[i][j];
                map[i][j] = '.';
            })
        }
        Direction::Down => tree
            .iter()
            .fold(
                BTreeMap::new(),
                |mut acc: BTreeMap<usize, BTreeSet<usize>>, &(i, j)| {
                    acc.entry(j).or_default().insert(i);
                    acc
                },
            )
            .into_iter()
            .flat_map(|(j, is)| is.iter().rev().map(|&i| (i, j)).collect::<Vec<_>>())
            .for_each(|(i, j)| {
                map[i + 1][j] = map[i][j];
                map[i][j] = '.';
            }),
        Direction::Up => tree
            .iter()
            .fold(
                BTreeMap::new(),
                |mut acc: BTreeMap<usize, BTreeSet<usize>>, &(i, j)| {
                    acc.entry(j).or_default().insert(i);
                    acc
                },
            )
            .into_iter()
            .flat_map(|(j, is)| is.iter().map(|&i| (i, j)).collect::<Vec<_>>())
            .for_each(|(i, j)| {
                map[i - 1][j] = map[i][j];
                map[i][j] = '.';
            }),
    }
}

fn is_tree_movable(map: &[Vec<char>], tree: &[(usize, usize)], direction: Direction) -> bool {
    match direction {
        Direction::Right => {
            let (i, j) = tree.iter().max_by_key(|(_i, j)| j).copied().unwrap();
            match map[i][j + 1] {
                '.' => true,
                '#' => false,
                _ => unreachable!(),
            }
        }
        Direction::Left => {
            let (i, j) = tree.iter().min_by_key(|(_i, j)| j).copied().unwrap();
            match map[i][j - 1] {
                '.' => true,
                '#' => false,
                _ => unreachable!(),
            }
        }
        Direction::Down => tree
            .iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<usize, Vec<usize>>, &(i, j)| {
                    acc.entry(j).or_default().push(i);
                    acc
                },
            )
            .into_iter()
            .map(|(j, is)| {
                let max_i = is.iter().max().copied().unwrap();
                (max_i, j)
            })
            .all(|(i, j)| match map[i + 1][j] {
                '.' => true,
                '#' => false,
                _ => unreachable!(),
            }),
        Direction::Up => tree
            .iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<usize, Vec<usize>>, &(i, j)| {
                    acc.entry(j).or_default().push(i);
                    acc
                },
            )
            .into_iter()
            .map(|(j, is)| {
                let min_i = is.iter().min().copied().unwrap();
                (min_i, j)
            })
            .all(|(i, j)| match map[i - 1][j] {
                '.' => true,
                '#' => false,
                _ => unreachable!(),
            }),
    }
}

fn dependency_tree(
    map: &[Vec<char>],
    position: (usize, usize),
    direction: Direction,
) -> Vec<(usize, usize)> {
    match direction {
        Direction::Right => (0..)
            .map(|level| {
                let (i, j) = position;
                (i, j + level)
            })
            .take_while(|&(i, j)| match map[i][j] {
                '@' | '[' | ']' => true,
                '.' | '#' => false,
                _ => unreachable!(),
            })
            .collect(),
        Direction::Left => (0..)
            .map(|level| {
                let (i, j) = position;
                (i, j - level)
            })
            .take_while(|&(i, j)| match map[i][j] {
                '@' | '[' | ']' => true,
                '.' | '#' => false,
                _ => unreachable!(),
            })
            .collect(),
        Direction::Down => (0..)
            .map(|level| position.0 + level)
            .scan(HashSet::from([position.1]), |js, i| {
                *js = js
                    .iter()
                    .flat_map(|&j| match map[i][j] {
                        '@' => vec![j].into_iter(),
                        '[' => vec![j, j + 1].into_iter(),
                        ']' => vec![j, j - 1].into_iter(),
                        '.' | '#' => vec![].into_iter(),
                        _ => unreachable!(),
                    })
                    .collect();
                match js.is_empty() {
                    true => None,
                    false => Some(js.iter().map(|&j| (i, j)).collect::<Vec<_>>()),
                }
            })
            .flatten()
            .collect(),
        Direction::Up => (0..)
            .map(|level| position.0 - level)
            .scan(HashSet::from([position.1]), |js, i| {
                *js = js
                    .iter()
                    .flat_map(|&j| match map[i][j] {
                        '@' => vec![j].into_iter(),
                        '[' => vec![j, j + 1].into_iter(),
                        ']' => vec![j, j - 1].into_iter(),
                        '.' | '#' => vec![].into_iter(),
                        _ => unreachable!(),
                    })
                    .collect();
                match js.is_empty() {
                    true => None,
                    false => Some(js.iter().map(|&j| (i, j)).collect::<Vec<_>>()),
                }
            })
            .flatten()
            .collect(),
    }
}

fn _draw(map: &[Vec<char>]) {
    println!();
    for row in map {
        for ch in row {
            print!("{}", ch);
        }
        println!();
    }
    println!();
}
