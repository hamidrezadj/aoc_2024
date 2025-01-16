fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            line.parse::<u64>()
                .expect("Not an unsigned integer that fits in 64 bits")
        })
        .map(|initial_n| {
            (0..2000).fold(initial_n, |n, _idx| {
                let n = ((n * 64) ^ n) % 16777216;
                let n = ((n / 32) ^ n) % 16777216;
                ((n * 2048) ^ n) % 16777216
            })
        })
        .sum::<u64>();
    println!("{}", output);
}
