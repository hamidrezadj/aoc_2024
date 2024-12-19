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
                .filter(|(_j, ch)| **ch == 'X')
                .map(|(j, _)| {
                    let right_side = j
                        .checked_add(1)
                        .filter(|j| *j <= max_j)
                        .filter(|j| input[i][*j] == 'M')
                        .and_then(|j| j.checked_add(1))
                        .filter(|j| *j <= max_j)
                        .filter(|j| input[i][*j] == 'A')
                        .and_then(|j| j.checked_add(1))
                        .filter(|j| *j <= max_j)
                        .filter(|j| input[i][*j] == 'S')
                        .is_some();
                    let left_side = j
                        .checked_sub(1)
                        .filter(|j| input[i][*j] == 'M')
                        .and_then(|j| j.checked_sub(1))
                        .filter(|j| input[i][*j] == 'A')
                        .and_then(|j| j.checked_sub(1))
                        .filter(|j| input[i][*j] == 'S')
                        .is_some();
                    let top_side = i
                        .checked_sub(1)
                        .filter(|i| input[*i][j] == 'M')
                        .and_then(|i| i.checked_sub(1))
                        .filter(|i| input[*i][j] == 'A')
                        .and_then(|i| i.checked_sub(1))
                        .filter(|i| input[*i][j] == 'S')
                        .is_some();
                    let bottom_side = i
                        .checked_add(1)
                        .filter(|i| *i <= max_i)
                        .filter(|i| input[*i][j] == 'M')
                        .and_then(|i| i.checked_add(1))
                        .filter(|i| *i <= max_i)
                        .filter(|i| input[*i][j] == 'A')
                        .and_then(|i| i.checked_add(1))
                        .filter(|i| *i <= max_i)
                        .filter(|i| input[*i][j] == 'S')
                        .is_some();
                    let top_right_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'A')
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    let bottom_right_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'A')
                        .and_then(|(i, j)| j.checked_add(1).filter(|j| *j <= max_j).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    let top_left_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'A')
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_sub(1).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    let bottom_left_side = Some((i, j))
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'M')
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'A')
                        .and_then(|(i, j)| j.checked_sub(1).map(|j| (i, j)))
                        .and_then(|(i, j)| i.checked_add(1).filter(|i| *i <= max_i).map(|i| (i, j)))
                        .filter(|(i, j)| input[*i][*j] == 'S')
                        .is_some();
                    [
                        right_side,
                        left_side,
                        top_side,
                        bottom_side,
                        top_right_side,
                        bottom_right_side,
                        top_left_side,
                        bottom_left_side,
                    ]
                    .into_iter()
                    .filter(|e| *e)
                    .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", output);
}
