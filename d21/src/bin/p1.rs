use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .inspect(|line| assert!(line.len() == 4, "Line longer than 4 character long"))
        .inspect(|line| {
            assert!(
                line.chars().all(|ch| ch.is_ascii_digit() || ch == 'A'),
                "Invalid character in line"
            )
        })
        .inspect(|line| {
            assert_eq!(
                line.chars().filter(|ch| *ch == 'A').count(),
                1,
                "Invalid A button count in line"
            )
        })
        .inspect(|line| assert!(line.ends_with('A'), "Invalid A button placement"))
        .collect::<Vec<String>>();
    let numbers = input
        .iter()
        .map(|line| &line[0..3])
        .map(|line| line.parse::<u64>().unwrap());
    let output = input
        .iter()
        .scan(HashMap::new(), |memo, line| {
            let mut line = line.clone().into_bytes();
            line.insert(0, b'A');
            let shortest_path_length = line
                .windows(2)
                .map(|w| shortest_path_length_depth_0(w[0], w[1], memo))
                .sum::<u64>();
            Some(shortest_path_length)
        })
        .zip(numbers)
        .map(|(length, number)| length * number)
        .sum::<u64>();
    println!("{}", output);
}

type Depth = u8;

fn shortest_path_length_depth_0(
    start: u8,
    end: u8,
    memo: &mut HashMap<(u8, u8, Depth), u64>,
) -> u64 {
    if let Some(length) = memo.get(&(start, end, 0)) {
        return *length;
    }
    let map_to_coordinates = |ch: u8| -> (usize, usize) {
        match ch {
            b'7' => (0, 0),
            b'8' => (0, 1),
            b'9' => (0, 2),
            b'4' => (1, 0),
            b'5' => (1, 1),
            b'6' => (1, 2),
            b'1' => (2, 0),
            b'2' => (2, 1),
            b'3' => (2, 2),
            b'0' => (3, 1),
            b'A' => (3, 2),
            _ => unreachable!(),
        }
    };
    let bad_midpoint = (3, 0);
    let start_pos = map_to_coordinates(start);
    let end_pos = map_to_coordinates(end);
    let (high_road, low_road) = roads(start_pos, end_pos, bad_midpoint);
    let length = [high_road, low_road]
        .into_iter()
        .flatten()
        .map(|road| {
            let mut road = road.into_bytes();
            road.insert(0, b'A');
            road.windows(2)
                .map(|w| shortest_path_length_depth_1(w[0], w[1], memo))
                .sum::<u64>()
        })
        .min()
        .unwrap();
    memo.insert((start, end, 0), length);
    length
}

fn shortest_path_length_depth_1(
    start: u8,
    end: u8,
    memo: &mut HashMap<(u8, u8, Depth), u64>,
) -> u64 {
    if let Some(length) = memo.get(&(start, end, 1)) {
        return *length;
    }
    let map_to_coordinates = |ch: u8| -> (usize, usize) {
        match ch {
            b'^' => (0, 1),
            b'A' => (0, 2),
            b'<' => (1, 0),
            b'v' => (1, 1),
            b'>' => (1, 2),
            _ => unreachable!(),
        }
    };
    let bad_midpoint = (0, 0);
    let start_pos = map_to_coordinates(start);
    let end_pos = map_to_coordinates(end);
    let (high_road, low_road) = roads(start_pos, end_pos, bad_midpoint);
    let length = [high_road, low_road]
        .into_iter()
        .flatten()
        .map(|road| {
            let mut road = road.into_bytes();
            road.insert(0, b'A');
            road.windows(2)
                .map(|w| shortest_path_length_depth_2(w[0], w[1]))
                .sum::<u64>()
        })
        .min()
        .unwrap();
    memo.insert((start, end, 1), length);
    length
}

fn shortest_path_length_depth_2(start: u8, end: u8) -> u64 {
    let map_to_coordinates = |ch: u8| -> (usize, usize) {
        match ch {
            b'^' => (0, 1),
            b'A' => (0, 2),
            b'<' => (1, 0),
            b'v' => (1, 1),
            b'>' => (1, 2),
            _ => unreachable!(),
        }
    };
    let (si, sj) = map_to_coordinates(start);
    let (ei, ej) = map_to_coordinates(end);
    (si.abs_diff(ei) + sj.abs_diff(ej) + 1) as u64
}

fn roads(
    (si, sj): (usize, usize),
    (ei, ej): (usize, usize),
    bad_midpoint: (usize, usize),
) -> (Option<String>, Option<String>) {
    let high_road = Some((ei, sj))
        .filter(|midpoint| *midpoint != bad_midpoint)
        .map(|_| match si.cmp(&ei) {
            Ordering::Less => "v".repeat(si.abs_diff(ei)),
            Ordering::Greater => "^".repeat(si.abs_diff(ei)),
            Ordering::Equal => String::new(),
        })
        .map(|rest| match sj.cmp(&ej) {
            Ordering::Less => rest + ">".repeat(sj.abs_diff(ej)).as_str(),
            Ordering::Greater => rest + "<".repeat(sj.abs_diff(ej)).as_str(),
            Ordering::Equal => rest,
        })
        .map(|rest| rest + "A");
    let low_road = Some((si, ej))
        .filter(|midpoint| *midpoint != bad_midpoint)
        .map(|_| match sj.cmp(&ej) {
            Ordering::Less => ">".repeat(sj.abs_diff(ej)),
            Ordering::Greater => "<".repeat(sj.abs_diff(ej)),
            Ordering::Equal => String::new(),
        })
        .map(|rest| match si.cmp(&ei) {
            Ordering::Less => rest + "v".repeat(si.abs_diff(ei)).as_str(),
            Ordering::Greater => rest + "^".repeat(si.abs_diff(ei)).as_str(),
            Ordering::Equal => rest,
        })
        .map(|rest| rest + "A");
    (high_road, low_road)
}
