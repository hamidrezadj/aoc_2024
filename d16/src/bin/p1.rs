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

    let mut shortest_path_tree: HashMap<Node, u64> = HashMap::new();
    let mut neighbours_of_tree: BinaryHeap<Reverse<(u64, Node)>> =
        starting_neighbours(start_position);
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

fn starting_neighbours(start_position: (usize, usize)) -> BinaryHeap<Reverse<(u64, Node)>> {
    BinaryHeap::from([
        Reverse((
            0,
            Node {
                position: start_position,
                direction: Direction::East,
            },
        )),
        Reverse((
            2000,
            Node {
                position: start_position,
                direction: Direction::West,
            },
        )),
    ])
}

fn neighbours(input: &[Vec<char>], node: Node, length: u64) -> BinaryHeap<Reverse<(u64, Node)>> {
    let next_node_in_same_direction = Some(node)
        .into_iter()
        .map(Node::forward)
        .filter(|node| input[node.position.0][node.position.1] != '#')
        .map(|node| (length + 1, node));
    let rotated_nodes = Some(node)
        .into_iter()
        .flat_map(Node::sides)
        .filter(|node| input[node.position.0][node.position.1] != '#')
        .map(|side_node| Node {
            position: node.position,
            direction: side_node.direction,
        })
        .map(|node| (length + 1000, node));
    next_node_in_same_direction
        .chain(rotated_nodes)
        .map(Reverse)
        .collect()
}

impl Node {
    fn forward(self) -> Node {
        let (i, j) = self.position;
        let position = match self.direction {
            Direction::East => (i, j + 1),
            Direction::South => (i + 1, j),
            Direction::West => (i, j - 1),
            Direction::North => (i - 1, j),
        };
        Node {
            position,
            direction: self.direction,
        }
    }
    fn sides(self) -> [Node; 2] {
        let (i, j) = self.position;
        match self.direction {
            Direction::East | Direction::West => [
                Node {
                    position: (i + 1, j),
                    direction: Direction::South,
                },
                Node {
                    position: (i - 1, j),
                    direction: Direction::North,
                },
            ],
            Direction::South | Direction::North => [
                Node {
                    position: (i, j + 1),
                    direction: Direction::East,
                },
                Node {
                    position: (i, j - 1),
                    direction: Direction::West,
                },
            ],
        }
    }
}
