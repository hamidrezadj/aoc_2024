fn main() {
    let (locks, keys) = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .inspect(|line| {
            assert!(
                line.chars().all(|ch| ".#".contains(ch)),
                "Invalid character received from stdin"
            )
        })
        .enumerate()
        .map(|(idx, line)| (idx % 8, line))
        .fold(Vec::new(), |mut accumulator, (stage, line)| {
            match stage {
                0 => {
                    accumulator.push(Vec::new());
                    accumulator.last_mut().unwrap().push(line);
                }
                1..=6 => accumulator.last_mut().unwrap().push(line),
                7 => assert!(line.is_empty(), "Not separated with an empty line"),
                _ => unreachable!(),
            }
            accumulator
        })
        .into_iter()
        .fold(
            (Vec::new(), Vec::new()),
            |(mut locks, mut keys), rectangle| {
                if rectangle.first().unwrap().chars().all(|ch| ch == '#') {
                    locks.push(rectangle);
                } else if rectangle.last().unwrap().chars().all(|ch| ch == '#') {
                    keys.push(rectangle);
                } else {
                    panic!("Invalid pattern: invalid top row and bottom row");
                }
                (locks, keys)
            },
        );
    let locks: Vec<[usize; 5]> = locks
        .into_iter()
        .map(|lock| {
            lock.into_iter()
                .enumerate()
                .skip(1)
                .fold([0; 5], |mut accumulator, (row, line)| {
                    line.chars().enumerate().for_each(|(column, character)| {
                        if character == '#' {
                            accumulator[column] = row;
                        }
                    });
                    accumulator
                })
        })
        .collect();
    let keys: Vec<[usize; 5]> = keys
        .into_iter()
        .map(|key| {
            key.into_iter().rev().enumerate().skip(1).fold(
                [0; 5],
                |mut accumulator, (row, line)| {
                    line.chars().enumerate().for_each(|(column, character)| {
                        if character == '#' {
                            accumulator[column] = row;
                        }
                    });
                    accumulator
                },
            )
        })
        .collect();
    let output = keys
        .iter()
        .map(|key| {
            locks
                .iter()
                .filter(|lock| {
                    key.iter()
                        .zip(lock.iter())
                        .map(|(pin_height, pin_depth)| pin_height + pin_depth)
                        .all(|sum| sum <= 5)
                })
                .count()
        })
        .sum::<usize>();
    println!("{}", output);
}
