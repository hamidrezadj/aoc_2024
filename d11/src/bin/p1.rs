fn main() {
    let mut input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .flat_map(|line| {
            line.split_whitespace()
                .map(|s| {
                    s.parse::<u64>()
                        .expect("Not an unsigned integer that fits in 64 bits")
                })
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<u64>>();
    for _ in 0..25 {
        input = input
            .into_iter()
            .flat_map(|n| match n {
                0 => vec![1],
                n if n.to_string().len() % 2 == 0 => {
                    let n = n.to_string();
                    let (n1, n2) = n.split_at(n.len() / 2);
                    let n1 = n1.parse::<u64>().unwrap();
                    let n2 = n2.parse::<u64>().unwrap();
                    vec![n1, n2]
                }
                n => vec![n.checked_mul(2024).expect("Overflow")],
            })
            .collect();
    }
    let output = input.len();
    println!("{}", output);
}
