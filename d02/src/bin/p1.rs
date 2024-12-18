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
            entry.windows(2).all(|window| match ordering {
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
            })
        })
        .filter(|condition| *condition)
        .count();
    println!("{}", safe_entry_count);
}
