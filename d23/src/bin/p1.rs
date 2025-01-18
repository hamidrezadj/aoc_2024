use std::collections::{HashMap, HashSet};

fn main() {
    let graph: HashMap<String, HashSet<String>> = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (lhs, rhs) = line.split_once('-').expect("Couldn't find dash in line");
            (lhs.to_owned(), rhs.to_owned())
        })
        .fold(HashMap::new(), |mut graph, (lhs, rhs)| {
            graph.entry(lhs.clone()).or_default().insert(rhs.clone());
            graph.entry(rhs).or_default().insert(lhs);
            graph
        });
    let output = graph
        .iter()
        .map(|(node, neighbours)| (node, combinations_of_two_of(neighbours)))
        .flat_map(|(n1, combinations_of_n2s_and_n3s)| {
            combinations_of_n2s_and_n3s
                .into_iter()
                .map(|(n2, n3)| [n1.as_str(), n2, n3])
        })
        .filter(|[_n1, n2, n3]| graph.get(*n2).unwrap().contains(*n3))
        .filter(|nodes| nodes.iter().any(|node| node.starts_with('t')))
        .map(|mut nodes| {
            nodes.sort();
            nodes
        })
        .collect::<HashSet<_>>()
        .len();
    println!("{:?}", output);
}

fn combinations_of_two_of(neighbours: &HashSet<String>) -> Vec<(&str, &str)> {
    let neighbours: Vec<&str> = neighbours.iter().map(|s| s.as_str()).collect();
    (0..neighbours.len())
        .flat_map(|i| (i + 1..neighbours.len()).map(move |j| (i, j)))
        .map(|(i, j)| (neighbours[i], neighbours[j]))
        .collect()
}
