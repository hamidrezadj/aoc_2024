fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin Error"))
        .reduce(|acc, line| acc + " " + &line)
        .expect("Empty input");
    let output = input
        .chars()
        .scan(
            (String::new(), String::new(), String::new(), true),
            |(state, num_1, num_2, is_enabled), ch| match ch {
                'm' => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    state.push(ch);
                    Some(0)
                }
                'u' if state == "m" => {
                    state.push(ch);
                    Some(0)
                }
                'l' if state == "mu" => {
                    state.push(ch);
                    Some(0)
                }
                '(' if state == "mul" => {
                    state.push(ch);
                    Some(0)
                }
                ch @ '0'..='9' if state == "mul(" => {
                    num_1.push(ch);
                    Some(0)
                }
                ',' if state == "mul(" => {
                    state.push(ch);
                    Some(0)
                }
                ch @ '0'..='9' if state == "mul(," => {
                    num_2.push(ch);
                    Some(0)
                }
                ')' if state == "mul(," => {
                    if !(*is_enabled)
                        || num_1.is_empty()
                        || num_1.len() > 3
                        || num_2.is_empty()
                        || num_2.len() > 3
                    {
                        state.clear();
                        num_1.clear();
                        num_2.clear();
                        return Some(0);
                    }
                    let n1 = num_1.parse::<u32>().unwrap();
                    let n2 = num_2.parse::<u32>().unwrap();
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    Some(n1 * n2)
                }
                'd' => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    state.push(ch);
                    Some(0)
                }
                'o' if state == "d" => {
                    state.push(ch);
                    Some(0)
                }
                'n' if state == "do" => {
                    state.push(ch);
                    Some(0)
                }
                '\'' if state == "don" => {
                    state.push(ch);
                    Some(0)
                }
                't' if state == "don'" => {
                    state.push(ch);
                    Some(0)
                }
                '(' if state == "don't" => {
                    state.push(ch);
                    Some(0)
                }
                ')' if state == "don't(" => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    *is_enabled = false;
                    Some(0)
                }
                '(' if state == "do" => {
                    state.push(ch);
                    Some(0)
                }
                ')' if state == "do(" => {
                    state.clear();
                    num_1.clear();
                    num_2.clear();
                    *is_enabled = true;
                    Some(0)
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
