use std::{
    cmp::Reverse,
    collections::{hash_map::Entry, BTreeMap, BinaryHeap, HashMap, HashSet},
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
        forward_neighbours(&input, start_node, 0);
    while let Some(Reverse((length, node))) = neighbours_of_tree.pop() {
        match shortest_path_tree.entry(node) {
            Entry::Occupied(_) => continue,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(length);
            }
        }
        neighbours_of_tree.append(&mut forward_neighbours(&input, node, length));
    }

    let end_position = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &ch)| ((i, j), ch)))
        .find(|((_i, _j), ch)| *ch == 'E')
        .map(|((i, j), _ch)| (i, j))
        .unwrap();
    let end_nodes_and_their_length = shortest_path_tree
        .iter()
        .filter(|(node, _length)| node.position == end_position)
        .fold(
            BTreeMap::new(),
            |mut acc: BTreeMap<u64, Vec<Node>>, (node, length)| {
                acc.entry(*length).or_default().push(*node);
                acc
            },
        )
        .pop_first()
        .map(|(end_length, end_nodes)| {
            end_nodes
                .into_iter()
                .map(|end_node| (end_node, end_length))
                .collect::<Vec<_>>()
        })
        .unwrap();
    let output = (0..)
        .scan(end_nodes_and_their_length, |nodes_and_their_lengths, _| {
            if nodes_and_their_lengths.is_empty() {
                return None;
            }
            let (node, length) = nodes_and_their_lengths.pop().unwrap();
            nodes_and_their_lengths.append(&mut backward_neighbours(
                &shortest_path_tree,
                node,
                length,
            ));
            Some(node)
        })
        .map(|node| node.position)
        .collect::<HashSet<(usize, usize)>>()
        .len();
    println!("{}", output);
}

fn backward_neighbours(
    shortest_path_tree: &HashMap<Node, u64>,
    node: Node,
    length: u64,
) -> Vec<(Node, u64)> {
    let sp_tree = shortest_path_tree;
    let backward_node_in_same_direction = Some(node)
        .into_iter()
        .map(Node::backward)
        .filter(|b_node| sp_tree.get(b_node).is_some_and(|b_len| b_len + 1 == length))
        .map(|b_node| (b_node, length - 1));
    let backward_rotated_nodes = Some(node)
        .into_iter()
        .flat_map(Node::rotated)
        .filter(|b_node| {
            sp_tree
                .get(b_node)
                .is_some_and(|b_len| b_len + 1000 == length)
        })
        .map(|node| (node, length - 1000));
    backward_node_in_same_direction
        .chain(backward_rotated_nodes)
        .collect()
}

fn forward_neighbours(
    input: &[Vec<char>],
    node: Node,
    length: u64,
) -> BinaryHeap<Reverse<(u64, Node)>> {
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
    fn backward(self) -> Node {
        let (i, j) = self.position;
        let position = match self.direction {
            Direction::East => (i, j - 1),
            Direction::South => (i - 1, j),
            Direction::West => (i, j + 1),
            Direction::North => (i + 1, j),
        };
        Node {
            position,
            direction: self.direction,
        }
    }
    fn rotated(self) -> Vec<Node> {
        match self.direction {
            Direction::East | Direction::West => [Direction::South, Direction::North],
            Direction::South | Direction::North => [Direction::East, Direction::West],
        }
        .into_iter()
        .map(|direction| Node {
            position: self.position,
            direction,
        })
        .collect()
    }
}
