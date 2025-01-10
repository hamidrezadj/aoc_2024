use std::{
    collections::{HashMap, HashSet},
    ops::Not,
};

fn main() {
    let mut input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"));
    let input_first_line: String = input
        .next()
        .expect("No first line, which is the available patterns, received in input");
    assert!(
        input_first_line.chars().all(|ch| " ,wbrgu".contains(ch)),
        "Invalid color character at first line"
    );
    let available_patterns: HashMap<&str, Vec<Option<char>>> = input_first_line
        .split(", ")
        .flat_map(|pattern| (0..=pattern.len()).map(|idx| pattern.split_at(idx)))
        .map(|(left_pattern, right_pattern)| (left_pattern, right_pattern.chars().next()))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<&str, HashSet<Option<char>>>, (pattern, next_color)| {
                acc.entry(pattern).or_default().insert(next_color);
                acc
            },
        )
        .into_iter()
        .map(|(pattern, next_colors)| (pattern, next_colors.into_iter().collect()))
        .collect();
    if available_patterns.is_empty() {
        eprintln!("Warning: No available patterns given. Output defaults to zero.");
        println!("0");
        return;
    }
    if input.next().is_none_or(|line| line.is_empty().not()) {
        panic!("No second line, which is empty, was received in input");
    }
    let output = input
        .inspect(|line| {
            assert!(
                line.chars().all(|ch| "wbrgu".contains(ch)),
                "Invalid color character at first line"
            )
        })
        .map(|desired_pattern| count_possibilities(&desired_pattern, &available_patterns))
        .sum::<u64>();
    println!("{}", output);
}

fn count_possibilities(
    desired_pattern: &str,
    available_patterns: &HashMap<&str, Vec<Option<char>>>,
) -> u64 {
    // Needs to be one character that is not a color.
    let terminator = ",";
    format!("{desired_pattern}{terminator}")
        .chars()
        .fold(
            HashMap::from([(String::new(), (String::new(), 1))]),
            |map_complete_to_incomplete: HashMap<String, (String, u64)>, next_color| {
                map_complete_to_incomplete
                    .into_iter()
                    .flat_map(|(complete, (incomplete, count))| {
                        let available_colors = available_patterns.get(incomplete.as_str()).unwrap();
                        let continue_incomplete = Some(())
                            .filter(|_| available_colors.contains(&Some(next_color)))
                            .map(|_| {
                                (
                                    complete.clone(),
                                    (format!("{incomplete}{next_color}").to_owned(), count),
                                )
                            });
                        let merge_incomplete = Some(())
                            .filter(|_| available_colors.contains(&None))
                            .map(|_| {
                                (
                                    format!("{complete}{incomplete}").to_owned(),
                                    (next_color.to_string(), count),
                                )
                            });
                        [continue_incomplete, merge_incomplete]
                    })
                    .flatten()
                    .fold(
                        HashMap::new(),
                        |mut acc, (complete, (incomplete, count))| {
                            acc.entry(complete).or_insert((incomplete, 0)).1 += count;
                            acc
                        },
                    )
            },
        )
        .into_iter()
        .find(|(complete, (incomplete, _count))| {
            complete == desired_pattern && incomplete == terminator
        })
        .map(|(_complete, (_incomplete, count))| count)
        .unwrap_or(0)
}
