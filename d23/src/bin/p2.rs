use std::collections::{HashMap, HashSet};

type Node = u16;
type Graph = HashMap<Node, HashSet<Node>>;

fn main() {
    let graph: Graph = std::io::stdin()
        .lines()
        .map(|line_result| line_result.expect("Stdin error"))
        .map(|line| {
            let (lhs, rhs) = line.split_once('-').expect("Couldn't find dash in line");
            (lhs.to_owned(), rhs.to_owned())
        })
        .map(|(lhs, rhs)| (encode(lhs), encode(rhs)))
        .fold(HashMap::new(), |mut graph, (lhs, rhs)| {
            graph.entry(lhs).or_default().insert(rhs);
            graph.entry(rhs).or_default().insert(lhs);
            graph
        });
    let all_nodes = graph.keys().map(|n| n.to_owned()).collect();
    let mut maximal_cliques = Vec::new();
    bron_kerbosch(
        &mut maximal_cliques,
        &graph,
        Vec::new(),
        all_nodes,
        Vec::new(),
    );
    let mut maximum_clique = maximal_cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .expect("No clique found")
        .into_iter()
        .map(decode)
        .collect::<Vec<String>>();
    maximum_clique.sort();
    let output = maximum_clique.join(",");
    println!("{}", output);
}

fn bron_kerbosch(
    maximal_cliques: &mut Vec<Vec<Node>>,
    graph: &Graph,
    checking: Vec<Node>,
    mut neighbours: Vec<Node>,
    mut excluded: Vec<Node>,
) {
    if neighbours.is_empty() && excluded.is_empty() {
        maximal_cliques.push(checking);
        return;
    }
    while let Some(node) = neighbours.pop() {
        let node_neighbours = graph.get(&node).unwrap();
        bron_kerbosch(
            maximal_cliques,
            graph,
            with_added_node(&checking, &node),
            intersection(&neighbours, node_neighbours),
            intersection(&excluded, node_neighbours),
        );
        excluded.push(node);
    }
}

fn intersection(a: &[Node], b: &HashSet<Node>) -> Vec<Node> {
    a.iter()
        .map(|n| n.to_owned())
        .filter(|n| b.contains(n))
        .collect()
}

fn with_added_node(a: &[Node], n: &Node) -> Vec<Node> {
    let mut a = a.to_vec();
    a.push(*n);
    a
}

fn encode(node_name: String) -> Node {
    assert_eq!(node_name.len(), 2);
    assert!(node_name.is_ascii());
    let bytes = node_name.into_bytes();
    let lhs = (bytes[0] as u16) << 8;
    let rhs = bytes[1] as u16;
    lhs + rhs
}

fn decode(node: Node) -> String {
    let lhs = (node >> 8) as u8 as char;
    let rhs = (node & 0xFF) as u8 as char;
    format!("{}{}", lhs, rhs)
}
