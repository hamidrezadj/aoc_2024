use std::ops::Not;

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    assert!(map.is_empty().not(), "Empty input");
    assert!(map.first().unwrap().is_empty().not(), "Empty line");
    assert!(
        map.iter()
            .all(|row| row.len() == map.first().unwrap().len()),
        "Uneven input: Not a rectangle"
    );
    assert!(
        map.iter().flatten().all(|ch| ".#@O".contains(*ch)),
        "Invalid character in map section of input"
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
    let initial_position = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, ch)| (i, j, *ch)))
        .find(|(_i, _j, ch)| *ch == '@')
        .map(|(i, j, _ch)| (i, j))
        .unwrap();
    let instructions: Vec<char> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    assert!(
        instructions.iter().all(|ch| ">v<^".contains(*ch)),
        "Invalid character in instructions section of input"
    );

    let mut map = map;
    let mut position = initial_position;
    for direction in instructions {
        match direction {
            '>' if move_tile(&mut map, position, |i, j| (i, j + 1)) => position.1 += 1,
            'v' if move_tile(&mut map, position, |i, j| (i + 1, j)) => position.0 += 1,
            '<' if move_tile(&mut map, position, |i, j| (i, j - 1)) => position.1 -= 1,
            '^' if move_tile(&mut map, position, |i, j| (i - 1, j)) => position.0 -= 1,
            _ => (),
        }
    }
    let output = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, ch)| (i, j, *ch)))
        .filter(|(_i, _j, ch)| *ch == 'O')
        .map(|(i, j, _ch)| (i, j))
        .map(|(i, j)| i * 100 + j)
        .sum::<usize>();
    println!("{}", output);
}

fn move_tile(
    map: &mut [Vec<char>],
    position: (usize, usize),
    next_position: impl Fn(usize, usize) -> (usize, usize),
) -> bool {
    let (i, j) = position;
    let tile = map[i][j];
    let (ni, nj) = next_position(i, j);
    let next_tile = map[ni][nj];
    match next_tile {
        '#' => false,
        '.' => {
            map[i][j] = '.';
            map[ni][nj] = tile;
            true
        }
        'O' if move_tile(map, (ni, nj), next_position) => {
            map[i][j] = '.';
            map[ni][nj] = tile;
            true
        }
        _ => false,
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
