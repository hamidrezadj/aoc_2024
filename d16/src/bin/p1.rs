use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BinaryHeap, HashMap},
    ops::Not,
};

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    position: (usize, usize),
    direction: Direction,
}

fn main() {
    let input: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| line.chars().collect())
        .collect();
    assert!(input.is_empty().not(), "Empty input");
    assert!(input.first().unwrap().is_empty().not(), "Empty line");
    assert!(
        input
            .iter()
            .all(|row| row.len() == input.first().unwrap().len()),
        "Uneven input: Not a rectangle"
    );
    assert!(
        input.iter().flatten().all(|ch| ".#SE".contains(*ch)),
        "Invalid character in input"
    );
    assert_eq!(
        input.iter().flatten().filter(|ch| **ch == 'S').count(),
        1,
        "Less or more than one 'S' character in input"
    );
    assert_eq!(
        input.iter().flatten().filter(|ch| **ch == 'E').count(),
        1,
        "Less or more than one 'E' character in input"
    );
    assert!(
        input.first().unwrap().iter().all(|ch| *ch == '#'),
        "Upper wall not complete"
    );
    assert!(
        input.last().unwrap().iter().all(|ch| *ch == '#'),
        "Lower wall not complete"
    );
    assert!(
        input
            .iter()
            .map(|row| row.first().unwrap())
            .all(|ch| *ch == '#'),
        "Left wall not complete"
    );
    assert!(
        input
            .iter()
            .map(|row| row.last().unwrap())
            .all(|ch| *ch == '#'),
        "Right wall not complete"
    );
    let start_position = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &ch)| ((i, j), ch)))
        .find(|((_i, _j), ch)| *ch == 'S')
        .map(|((i, j), _ch)| (i, j))
        .unwrap();
    let start_node = Node {
        position: start_position,
        direction: Direction::East,
    };
    let mut shortest_path_tree: HashMap<Node, u64> = HashMap::from([(start_node, 0)]);
    let mut neighbours_of_tree: BinaryHeap<Reverse<(u64, Node)>> =
        neighbours(&input, start_node, 0);
    while let Some(Reverse((length, node))) = neighbours_of_tree.pop() {
        let is_end = {
            let (i, j) = node.position;
            input[i][j] == 'E'
        };
        if is_end {
            println!("{}", length);
            return;
        }
        match shortest_path_tree.entry(node) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(length);
            }
        }
        neighbours_of_tree.append(&mut neighbours(&input, node, length));
    }
}

fn neighbours(input: &[Vec<char>], node: Node, length: u64) -> BinaryHeap<Reverse<(u64, Node)>> {
    let next_node_in_same_direction = Some(node.position)
        .into_iter()
        .map(|position| node.direction.forward_position(position))
        .filter(|&(i, j)| input[i][j] != '#')
        .map(|position| Node {
            position,
            direction: node.direction,
        })
        .map(|node| (length + 1, node));
    let rotated_nodes = Some(node.position)
        .into_iter()
        .flat_map(|position| node.direction.side_positions(position))
        .filter(|&(i, j)| input[i][j] != '#')
        .map(|side_position| Direction::find_direction(side_position, node.position))
        .map(|direction| Node {
            position: node.position,
            direction,
        })
        .map(|node| (length + 1000, node));
    next_node_in_same_direction
        .chain(rotated_nodes)
        .map(Reverse)
        .collect()
}

impl Direction {
    fn forward_position(&self, position: (usize, usize)) -> (usize, usize) {
        let (i, j) = position;
        match self {
            Direction::East => (i, j + 1),
            Direction::South => (i + 1, j),
            Direction::West => (i, j - 1),
            Direction::North => (i - 1, j),
        }
    }
    fn side_positions(&self, position: (usize, usize)) -> [(usize, usize); 2] {
        let (i, j) = position;
        match self {
            Direction::East | Direction::West => [(i + 1, j), (i - 1, j)],
            Direction::South | Direction::North => [(i, j + 1), (i, j - 1)],
        }
    }

    fn find_direction(position2: (usize, usize), position1: (usize, usize)) -> Direction {
        let (i2, j2) = position2;
        let (i1, j1) = position1;
        if i2 == i1 && j2 == j1 + 1 {
            Direction::East
        } else if i2 == i1 + 1 && j2 == j1 {
            Direction::South
        } else if i2 == i1 && j2 + 1 == j1 {
            Direction::West
        } else if i2 + 1 == i1 && j2 == j1 {
            Direction::North
        } else {
            panic!("Not a proper use of find_direction")
        }
    }
}
