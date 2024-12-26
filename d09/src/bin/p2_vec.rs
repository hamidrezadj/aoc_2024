use std::{collections::HashSet, io::Read};

#[derive(Clone, Copy)]
enum Space {
    File { size: usize, id: usize },
    Free { size: usize },
}

fn main() {
    let input: Vec<Space> = std::io::stdin()
        .bytes()
        .map(|byte_result| byte_result.expect("Stdin error"))
        .filter(|byte| *byte != b'\n')
        .map(|byte| match byte {
            byte @ b'0'..=b'9' => byte - b'0',
            _ => panic!("Not an ascii digit"),
        })
        .map(|num| num as usize)
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
    let output = input
        .iter()
        .enumerate()
        .scan(
            HashSet::new(),
            |moved_ids, (forward_idx, forward_space)| match forward_space {
                Space::File { size, id } if !moved_ids.contains(id) => Some(vec![Space::File {
                    size: *size,
                    id: *id,
                }]),
                Space::File { size, id: _ } => Some(vec![Space::Free { size: *size }]),
                Space::Free { size: free_size } => Some(scan_backwards_and_find_fitting_files(
                    &input,
                    free_size,
                    forward_idx,
                    moved_ids,
                )),
            },
        )
        .flatten()
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

fn scan_backwards_and_find_fitting_files(
    input: &[Space],
    free_size: &usize,
    forward_idx: usize,
    moved_ids: &mut HashSet<usize>,
) -> Vec<Space> {
    let fitted_files: Vec<Space> = input
        .iter()
        .enumerate()
        .filter_map(|(backward_idx, backward_space)| match backward_space {
            Space::File { size, id } => Some((backward_idx, *size, *id)),
            Space::Free { size: _ } => None,
        })
        .rev()
        .scan(
            *free_size,
            |free_size: &mut usize, (bfile_idx, bfile_size, bfile_id)| {
                if forward_idx <= bfile_idx
                    && *free_size >= bfile_size
                    && !moved_ids.contains(&bfile_id)
                {
                    *free_size -= bfile_size;
                    moved_ids.insert(bfile_id);
                    Some(Some(Space::File {
                        size: bfile_size,
                        id: bfile_id,
                    }))
                } else {
                    Some(None)
                }
            },
        )
        .flatten()
        .collect();
    let fitted_files_len: usize = fitted_files
        .iter()
        .map(|file| match file {
            Space::File { size, id: _ } => size,
            Space::Free { size: _ } => unreachable!(),
        })
        .sum();
    let remaining_free_size = free_size - fitted_files_len;
    if remaining_free_size > 0 {
        let mut new_spaces = fitted_files;
        new_spaces.push(Space::Free {
            size: remaining_free_size,
        });
        new_spaces
    } else {
        fitted_files
    }
}
