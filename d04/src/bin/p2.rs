fn main() {
    let input: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| line.chars().collect())
        .collect();
    assert!(!input.is_empty(), "Empty input");
    assert!(!input[0].is_empty(), "Empty line");
    assert!(
        input.iter().all(|row| row.len() == input[0].len()),
        "Uneven input"
    );
    let max_i = input.len() - 1;
    let max_j = input[0].len() - 1;
    let output = input
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_j, ch)| **ch == 'A')
                .map(|(j, _)| {
                    let top_right_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_sub(2).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(2).filter(|i| *i <= max_j).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    let bottom_left_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .and_then(|(i, j)| j.checked_sub(2).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(2).filter(|i| *i <= max_j).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .is_some();
                    let bottom_right_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_sub(2).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(2).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    let top_left_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .and_then(|(i, j)| j.checked_sub(2).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(2).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .is_some();
                    let diag_count = [
                        top_right_side,
                        bottom_right_side,
                        top_left_side,
                        bottom_left_side,
                    ]
                    .into_iter()
                    .filter(|e| *e)
                    .count();
                    if diag_count == 2 {
                        1usize
                    } else {
                        0usize
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", output);
}
