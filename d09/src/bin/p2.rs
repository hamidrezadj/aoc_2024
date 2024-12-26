use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Clone, Copy)]
enum Space {
    File { size: usize, id: usize },
    Free { size: usize },
}

fn main() {
    let input: Vec<Space> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| line.chars().collect::<Vec<char>>())
        .map(|ch| ch.to_string().parse::<usize>().expect("Not an ascii digit"))
        .enumerate()
        .map(|(idx, num)| {
            if idx % 2 == 0 {
                let id = idx / 2;
                let size = num;
                Space::File { size, id }
            } else {
                let size = num;
                Space::Free { size }
            }
        })
        .collect();
    let mut unmoved_id_map: HashMap<usize, usize> = input
        .iter()
        .filter_map(|space| match space {
            Space::File { size, id } => Some((*id, *size)),
            Space::Free { size: _ } => None,
        })
        .collect();
    let mut cumulative_size_map: BTreeMap<usize, BTreeSet<usize>> =
        input
            .iter()
            .fold(BTreeMap::new(), |mut map, space| match space {
                Space::Free { size: _ } => map,
                Space::File { size, id } => {
                    for size in *size..=9 {
                        map.entry(size).or_default().insert(*id);
                    }
                    map
                }
            });
    let output = input
        .iter()
        .flat_map(|space| match space {
            Space::File { size, id } if unmoved_id_map.contains_key(id) => {
                unmoved_id_map.remove(id);
                cumulative_size_map.values_mut().for_each(|ids| {
                    ids.remove(id);
                });
                vec![Space::File {
                    size: *size,
                    id: *id,
                }]
            }
            Space::File { size, id: _ } => vec![Space::Free { size: *size }],
            Space::Free {
                size: mut free_size,
            } => {
                let mut new_spaces = Vec::new();
                while let Some(file_id) = cumulative_size_map
                    .get(&free_size)
                    .and_then(|ids| ids.last().copied())
                {
                    let file_size = unmoved_id_map.get(&file_id).copied().unwrap();
                    unmoved_id_map.remove(&file_id);
                    cumulative_size_map.values_mut().for_each(|ids| {
                        ids.remove(&file_id);
                    });
                    free_size -= file_size;
                    new_spaces.push(Space::File {
                        size: file_size,
                        id: file_id,
                    })
                }
                if free_size > 0 {
                    new_spaces.push(Space::Free { size: free_size });
                }
                new_spaces
            }
        })
        .flat_map(|space| match space {
            Space::File { size, id } => vec![Space::File { size: 1, id }; size],
            Space::Free { size } => vec![Space::Free { size: 1 }; size],
        })
        .enumerate()
        .filter_map(|(idx, space)| match space {
            Space::File { size: _, id } => Some((idx, id)),
            Space::Free { size: _ } => None,
        })
        .map(|(idx, id)| idx.checked_mul(id).expect("Overflow"))
        .reduce(|acc, e| acc.checked_add(e).expect("Overflow"))
        .expect("Empty input");
    println!("{}", output);
}
