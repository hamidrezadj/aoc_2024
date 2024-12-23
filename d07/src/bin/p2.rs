fn main() {
    let output = std::io::stdin()
        .lines()
        .map(|line_result| line_result.unwrap())
        .map(|line| {
            let (result, numbers) = line.split_once(':').expect("No colon found in line");
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|n| {
                    n.parse::<u64>()
                        .expect("Not an unsigned integer that fits in 64 bits")
                })
                .collect();
            let result = result
                .parse::<u64>()
                .expect("Not an unsigned integer that fits in 64 bits");
            (result, numbers)
        })
        .filter(|(result, numbers)| {
            let all_possibilities = numbers.iter().fold(Vec::new(), |mut acc, num| {
                if acc.is_empty() {
                    acc.push(*num);
                    return acc;
                }
                let sum_possibilites = acc
                    .iter()
                    .map(|possibility| possibility.checked_add(*num).expect("Overflow"));
                let product_possibilites = acc
                    .iter()
                    .map(|possibility| possibility.checked_mul(*num).expect("Overflow"));
                let concatenation_possibilites = acc.iter().map(|possibility| {
                    let lhs = possibility.to_string();
                    let rhs = num.to_string();
                    let res = lhs + &rhs;
                    res.parse::<u64>().expect("Overflow")
                });
                sum_possibilites
                    .chain(product_possibilites)
                    .chain(concatenation_possibilites)
                    .filter(|possibility| possibility <= result)
                    .collect()
            });
            all_possibilities.contains(result)
        })
        .map(|(result, _numbers)| result)
        .sum::<u64>();
    println!("{}", output);
}
