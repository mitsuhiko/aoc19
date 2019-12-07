use std::collections::BTreeMap;
use std::iter::successors;

type Graph<'a> = BTreeMap<&'a str, &'a str>;

fn parse_graph(s: &str) -> Graph<'_> {
    s.lines()
        .filter_map(|line| Some((line.find(')')?, line)))
        .map(|(idx, line)| (&line[idx + 1..], &line[..idx]))
        .collect()
}

fn walk<'a>(graph: &'a Graph, to: &'a str) -> impl Iterator<Item = &'a str> {
    successors(Some(to), move |&to| graph.get(to).copied()).skip(1)
}

fn count_orbits(graph: &Graph) -> usize {
    graph.keys().fold(0, |x, to| x + walk(graph, to).count())
}

fn measure_path(graph: &Graph, a: &str, b: &str) -> Option<usize> {
    let b_orbits: Vec<_> = walk(graph, b).collect();
    walk(graph, a)
        .enumerate()
        .filter_map(|(d, p)| Some(d + b_orbits.iter().rposition(|&x| x == p)?))
        .next()
}

fn main() {
    let graph = parse_graph(include_str!("../input.txt"));
    println!("part 1: {}", count_orbits(&graph));
    println!("part 2: {}", measure_path(&graph, "YOU", "SAN").unwrap());
}
