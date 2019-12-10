use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::PI;

const PRECISION: f64 = 100_000.0;
type Point = (i64, i64);

fn parse_asteroids(data: &str) -> Vec<Point> {
    data.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect()
}

fn get_angle(p1: Point, p2: Point) -> i64 {
    ((p2.0 as f64 - p1.0 as f64)
        .atan2(p1.1 as f64 - p2.1 as f64)
        .rem_euclid(2.0 * PI)
        * PRECISION) as i64
}

fn count_visible(asteroids: &[Point], p: Point) -> i64 {
    asteroids
        .iter()
        .filter(|x| **x != p)
        .map(|x| get_angle(p, *x))
        .collect::<BTreeSet<_>>()
        .len() as i64
}

fn find_max_reachable(asteroids: &[Point]) -> (Point, i64) {
    asteroids
        .iter()
        .map(|&p| (p, count_visible(asteroids, p)))
        .max_by_key(|x| x.1)
        .unwrap()
}

fn find_sweeping_hits(asteroids: &[Point]) -> Vec<(i64, i64)> {
    let t = find_max_reachable(asteroids).0;
    let mut asteroids = asteroids
        .iter()
        .copied()
        .filter(|&p| p != t)
        .collect::<Vec<_>>();
    asteroids.sort_by_key(|(x, y)| (x - t.0) * (x - t.0) + (y - t.1) * (y - t.1));
    let sort_keys = asteroids
        .iter()
        .enumerate()
        .map(|(idx, &p)| {
            let angle = get_angle(t, p);
            let rank = asteroids[..idx]
                .iter()
                .filter(|&&x| angle == get_angle(t, x))
                .count();
            (p, (rank, angle))
        })
        .collect::<BTreeMap<_, _>>();
    asteroids.sort_by_key(|p| sort_keys[p]);
    asteroids
}

fn main() {
    let asteroids = parse_asteroids(include_str!("../input.txt"));
    println!("part 1: {}", find_max_reachable(&asteroids).1);
    let hit = find_sweeping_hits(&asteroids)[199];
    println!("part 2: {}", hit.0 * 100 + hit.1);
}
