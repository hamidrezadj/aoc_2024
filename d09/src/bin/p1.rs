use std::io::Read;

#[derive(Clone, Copy)]
enum SpaceUnit {
    File { id: usize },
    Free,
}

fn main() {
    let input: Vec<SpaceUnit> = std::io::stdin()
        .bytes()
        .map(|byte_result| byte_result.expect("Stdin error"))
        .filter(|byte| *byte != b'\n')
        .map(|byte| match byte {
            byte @ b'0'..=b'9' => byte - b'0',
            _ => panic!("Not an ascii digit"),
        })
        .map(|num| num as usize)
        .enumerate()
        .flat_map(|(idx, num)| {
            if idx % 2 == 0 {
                let file_id = idx / 2;
                let file_size = num;
                [SpaceUnit::File { id: file_id }].repeat(file_size)
            } else {
                let free_size = num;
                [SpaceUnit::Free].repeat(free_size)
            }
        })
        .collect();
    let mut backwards_iterator = input
        .iter()
        .enumerate()
        .rev()
        .filter_map(|(idx, unit)| match unit {
            SpaceUnit::File { id } => Some((idx, id)),
            SpaceUnit::Free => None,
        })
        .peekable();
    let output = input
        .iter()
        .map(|forward_unit| match forward_unit {
            SpaceUnit::File { id } => {
                let (backward_idx, _) = backwards_iterator.peek().unwrap();
                (*id, *backward_idx)
            }
            SpaceUnit::Free => {
                let (backward_idx, id) = backwards_iterator.next().unwrap();
                (*id, backward_idx)
            }
        })
        .enumerate()
        .map_while(
            |(f_idx, (id, b_idx))| {
                if f_idx <= b_idx {
                    Some(id)
                } else {
                    None
                }
            },
        )
        .enumerate()
        .map(|(idx, id)| idx.checked_mul(id).expect("Overflow"))
        .reduce(|acc, e| acc.checked_add(e).expect("Overflow"))
        .expect("Empty input");
    println!("{}", output);
}
