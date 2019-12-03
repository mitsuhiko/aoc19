use std::collections::{BTreeMap, BTreeSet};

fn walk_wires<'a>(
    input: &'a str,
) -> impl Iterator<Item = impl Iterator<Item = (i32, i32)> + 'a> + 'a {
    input.lines().take(2).map(|wire| {
        wire.split(',').flat_map(|item| {
            std::iter::repeat(match item.chars().next().unwrap() {
                'L' => (-1, 0),
                'R' => (1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("bad input"),
            })
            .take(item[1..].parse().unwrap())
        })
    })
}

fn find_manhattan_intersection(input: &str) -> Option<i32> {
    let mut grid: BTreeMap<_, u8> = Default::default();

    for wire in walk_wires(input) {
        let mut x = 0;
        let mut y = 0;
        let mut seen = BTreeSet::new();
        for (step_x, step_y) in wire {
            x += step_x;
            y += step_y;
            if seen.insert((x, y)) {
                *grid.entry((x, y)).or_default() += 1;
            }
        }
    }

    grid.into_iter()
        .filter(|(_, wires)| *wires == 2)
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
}

fn find_closest_intersection(input: &str) -> Option<i32> {
    let mut grid: BTreeMap<_, BTreeMap<_, _>> = Default::default();

    for (idx, wire) in walk_wires(input).enumerate() {
        let mut x = 0;
        let mut y = 0;
        let mut dist = 0;
        for (step_x, step_y) in wire {
            x += step_x;
            y += step_y;
            dist += step_x.abs() + step_y.abs();
            *grid.entry((x, y)).or_default().entry(idx == 0).or_default() += dist;
        }
    }

    grid.into_iter()
        .filter(|(_, wires)| wires.len() == 2)
        .map(|(_, wires)| wires.values().sum())
        .min()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {}", find_manhattan_intersection(input).unwrap());
    println!("part 2: {}", find_closest_intersection(input).unwrap());
}
