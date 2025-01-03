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
            if entry.is_empty() {
                panic!("Line empty");
            }
        })
        .map(|entry| {
            if entry.len() == 1 {
                // Safety of an entry with one number is ambiguous.
                // In this implementation they are considered safe.
                return (entry, Ok(()));
            }
            let first_number = entry.first().unwrap();
            let last_number = entry.last().unwrap();
            let ordering = last_number.cmp(first_number);
            let entry_safety_result =
                entry
                    .windows(2)
                    .enumerate()
                    .try_for_each(|(idx, window)| match ordering {
                        Ordering::Less if is_window_safely_descending(window) => Ok(()),
                        Ordering::Greater if is_window_safely_ascending(window) => Ok(()),
                        Ordering::Equal => {
                            // This means that the first and last number are equal.
                            // So both need to be checked.
                            let problematic_idxs = vec![0, entry.len() - 1];
                            Err(problematic_idxs)
                        }
                        _ if idx == 0 => {
                            // This might mean that the last number has thrown off
                            // the ordering. So the last number has to be checked
                            // as well.
                            let problematic_idxs = vec![0, 1, entry.len() - 1];
                            Err(problematic_idxs)
                        }
                        _ => {
                            let problematic_idxs = vec![idx, idx + 1];
                            Err(problematic_idxs)
                        }
                    });
            (entry, entry_safety_result)
        })
        .filter(|(entry, entry_safety_result)| match entry_safety_result {
            Ok(()) => true,
            Err(problematic_idxs) => problematic_idxs.iter().any(|problematic_idx| {
                if entry.len() == 2 {
                    // Fixing an entry that has two numbers results in an
                    // entry with one number in it, which its safety is ambiguous.
                    // In this implementation they are considered safe.
                    return true;
                }
                let mut entry = entry.clone();
                entry.remove(*problematic_idx);
                let first_number = entry.first().unwrap();
                let last_number = entry.last().unwrap();
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
