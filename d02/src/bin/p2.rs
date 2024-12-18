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
        .map(|entry| {
            let first_number = entry.first().expect("Line empty");
            let last_number = entry.iter().last().expect("Line empty");
            let ordering = last_number.cmp(first_number);
            (entry, ordering)
        })
        .map(|(entry, ordering)| {
            let mut problematic_idx = None;
            entry
                .windows(2)
                .enumerate()
                .all(|(idx, window)| match ordering {
                    Ordering::Less
                        if window[0] > window[1] && (1..=3).contains(&(window[0] - window[1])) =>
                    {
                        true
                    }
                    Ordering::Greater
                        if window[0] < window[1] && (1..=3).contains(&(window[1] - window[0])) =>
                    {
                        true
                    }
                    _ => {
                        problematic_idx = Some(idx);
                        false
                    }
                });
            (entry, ordering, problematic_idx)
        })
        .filter(|(entry, ordering, problematic_idx)| match problematic_idx {
            None => true,
            Some(idx) => {
                let mut try_1 = entry.clone();
                try_1.remove(*idx);
                let is_try_1_good = try_1.windows(2).all(|window| match ordering {
                    Ordering::Less
                        if window[0] > window[1] && (1..=3).contains(&(window[0] - window[1])) =>
                    {
                        true
                    }
                    Ordering::Greater
                        if window[0] < window[1] && (1..=3).contains(&(window[1] - window[0])) =>
                    {
                        true
                    }
                    _ => false,
                });
                let mut try_2 = entry.clone();
                try_2.remove(*idx + 1);
                let is_try_2_good = try_2.windows(2).all(|window| match ordering {
                    Ordering::Less
                        if window[0] > window[1] && (1..=3).contains(&(window[0] - window[1])) =>
                    {
                        true
                    }
                    Ordering::Greater
                        if window[0] < window[1] && (1..=3).contains(&(window[1] - window[0])) =>
                    {
                        true
                    }
                    _ => false,
                });
                is_try_1_good || is_try_2_good
            }
        })
        .count();
    println!("{}", safe_entry_count);
}
