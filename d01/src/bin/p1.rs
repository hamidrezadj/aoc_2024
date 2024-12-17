use std::collections::BinaryHeap;

fn main() {
    let mut column_1: BinaryHeap<u32> = BinaryHeap::new();
    let mut column_2: BinaryHeap<u32> = BinaryHeap::new();
    std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .for_each(|line| {
            let mut numbers = line
                .split_whitespace()
                .map(|word| word.parse::<u32>().expect("Not an unsigned integer"));
            let number_1 = numbers.next().expect("Less than one number in line");
            let number_2 = numbers.next().expect("Less than two numbers in line");
            if numbers.next().is_some() {
                panic!("More than two numbers in line");
            }
            column_1.push(number_1);
            column_2.push(number_2);
        });
    let column_1 = column_1.into_sorted_vec();
    let column_2 = column_2.into_sorted_vec();
    let output = column_1
        .into_iter()
        .zip(column_2)
        .map(|(num_1, num_2)| num_1.abs_diff(num_2))
        .sum::<u32>();
    println!("{}", output);
}
