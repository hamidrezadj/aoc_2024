use std::cmp::Ordering;

fn main() {
    let safe_entry_count = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.split_whitespace()
                .map(|word| {
                    word.parse::<u32>()
                        .expect("Not an unsigned integer that fits in 32 bits")
                })
                .collect::<Vec<u32>>()
        })
        .inspect(|entry| {
            if entry.len() < 2 {
                panic!("Less than two numbers in line");
            }
        })
        .map(|entry| {
            let first_number = entry.first().expect("Line empty");
            let last_number = entry.last().expect("Line empty");
            let ordering = last_number.cmp(first_number);
            (entry, ordering)
        })
        .map(|(entry, ordering)| {
            let problematic_idxs =
                entry
                    .windows(2)
                    .enumerate()
                    .try_for_each(|(idx, window)| match ordering {
                        Ordering::Less if is_window_safely_descending(window) => Ok(()),
                        Ordering::Greater if is_window_safely_ascending(window) => Ok(()),
                        Ordering::Equal => {
                            let problematic_idxs = vec![0, entry.len() - 1];
                            Err(problematic_idxs)
                        }
                        _ if idx == 0 => {
                            let problematic_idxs = vec![0, 1, entry.len() - 1];
                            Err(problematic_idxs)
                        }
                        _ => {
                            let problematic_idxs = vec![idx, idx + 1];
                            Err(problematic_idxs)
                        }
                    });
            (entry, problematic_idxs)
        })
        .filter(|(entry, problematic_idx)| match problematic_idx {
            Ok(()) => true,
            Err(problematic_idxs) => problematic_idxs.iter().any(|problematic_idx| {
                let mut entry = entry.clone();
                entry.remove(*problematic_idx);
                let first_number = entry.first().expect("Line empty");
                let last_number = entry.last().expect("Line empty");
                let ordering = last_number.cmp(first_number);
                entry.windows(2).all(|window| match ordering {
                    Ordering::Less if is_window_safely_descending(window) => true,
                    Ordering::Greater if is_window_safely_ascending(window) => true,
                    _ => false,
                })
            }),
        })
        .count();
    println!("{}", safe_entry_count);
}

fn is_window_safely_ascending(window: &[u32]) -> bool {
    window[0] < window[1] && (1..=3).contains(&(window[1] - window[0]))
}

fn is_window_safely_descending(window: &[u32]) -> bool {
    window[0] > window[1] && (1..=3).contains(&(window[0] - window[1]))
}
