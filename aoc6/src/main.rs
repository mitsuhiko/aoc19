use std::collections::BTreeMap;
use std::iter::successors;

type Graph<'a> = BTreeMap<&'a str, &'a str>;

fn parse_graph(s: &str) -> Graph<'_> {
    s.trim()
        .lines()
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
    let b_orbits = walk(graph, b)
        .enumerate()
        .map(|(distance, point)| (point, distance))
        .collect::<BTreeMap<_, _>>();
    walk(graph, a)
        .enumerate()
        .filter_map(|(distance, point)| Some(distance + b_orbits.get(point)?))
        .next()
}

fn main() {
    let graph = parse_graph(include_str!("../input.txt"));
    println!("part 1: {}", count_orbits(&graph));
    println!("part 2: {}", measure_path(&graph, "YOU", "SAN").unwrap());
}
