use std::collections::HashSet;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

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
    let mut visits: HashSet<(usize, usize)> = HashSet::new();
    let mut position = input
        .iter()
        .enumerate()
        .filter(|(_i, row)| row.contains(&'^'))
        .map(|(i, line)| (i, line.iter().position(|ch| *ch == '^').unwrap()))
        .next()
        .expect("Guard not found");
    let mut direction = Direction::Up;
    loop {
        let (next_position, next_direction) = find_next(&input, position, direction);
        mark_visits(&mut visits, position, next_position);
        match next_direction {
            Some(next_direction) => {
                position = next_position;
                direction = next_direction;
            }
            None => {
                break;
            }
        }
    }
    let output = visits.len();
    println!("{}", output);
}

fn mark_visits(
    visits: &mut HashSet<(usize, usize)>,
    position: (usize, usize),
    next_position: (usize, usize),
) {
    let (i, j) = position;
    let (ni, nj) = next_position;
    if i == ni {
        for j in j.min(nj)..=j.max(nj) {
            visits.insert((i, j));
        }
    } else if j == nj {
        for i in i.min(ni)..=i.max(ni) {
            visits.insert((i, j));
        }
    } else {
        panic!("Diagonal movement not allowed");
    }
}

fn find_next(
    input: &[Vec<char>],
    position: (usize, usize),
    direction: Direction,
) -> ((usize, usize), Option<Direction>) {
    match direction {
        Direction::Up => {
            let (mut i, j) = position;
            while let Some(new_i) = i.checked_sub(1) {
                if input[new_i][j] == '#' {
                    return ((i, j), Some(Direction::Right));
                }
                i = new_i;
            }
            ((i, j), None)
        }
        Direction::Right => {
            let (i, mut j) = position;
            while let Some(new_j) = j.checked_add(1).filter(|j| *j < input[i].len()) {
                if input[i][new_j] == '#' {
                    return ((i, j), Some(Direction::Down));
                }
                j = new_j;
            }
            ((i, j), None)
        }
        Direction::Down => {
            let (mut i, j) = position;
            while let Some(new_i) = i.checked_add(1).filter(|i| *i < input.len()) {
                if input[new_i][j] == '#' {
                    return ((i, j), Some(Direction::Left));
                }
                i = new_i;
            }
            ((i, j), None)
        }
        Direction::Left => {
            let (i, mut j) = position;
            while let Some(new_j) = j.checked_sub(1) {
                if input[i][new_j] == '#' {
                    return ((i, j), Some(Direction::Up));
                }
                j = new_j;
            }
            ((i, j), None)
        }
    }
}
