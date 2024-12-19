fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin Error"))
        .reduce(|acc, line| acc + " " + &line)
        .expect("Empty input");
    let output = input
        .chars()
        .scan(
            (String::new(), String::new(), String::new()),
            |(state, num_1, num_2), ch| match state.as_str() {
                _ if ch == 'm' => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    state.push(ch);
                    Some(0)
                }
                "m" if ch == 'u' => {
                    state.push(ch);
                    Some(0)
                }
                "mu" if ch == 'l' => {
                    state.push(ch);
                    Some(0)
                }
                "mul" if ch == '(' => {
                    state.push(ch);
                    Some(0)
                }
                "mul(" if ch.is_ascii_digit() => {
                    num_1.push(ch);
                    Some(0)
                }
                "mul(" if ch == ',' => {
                    state.push(ch);
                    Some(0)
                }
                "mul(," if ch.is_ascii_digit() => {
                    num_2.push(ch);
                    Some(0)
                }
                "mul(," if ch == ')' => {
                    if num_1.is_empty() || num_1.len() > 3 || num_2.is_empty() || num_2.len() > 3 {
                        state.clear();
                        num_1.clear();
                        num_2.clear();
                        return Some(0);
                    }
                    let num_1_parsed = num_1.parse::<u32>().unwrap();
                    let num_2_parsed = num_2.parse::<u32>().unwrap();
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    Some(num_1_parsed * num_2_parsed)
                }
                _ => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    Some(0)
                }
            },
        )
        .sum::<u32>();
    println!("{}", output);
}
