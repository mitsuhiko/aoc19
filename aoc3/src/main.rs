use std::collections::{BTreeMap, BTreeSet};

fn find_intersection(input: &str) -> i32 {
    let mut grid: BTreeMap<(i32, i32), BTreeSet<u8>> = Default::default();
    let mut wires = 0u8;

    for line in input.lines() {
        let mut x = 0;
        let mut y = 0;
        let wire = wires;
        wires += 1;
        for item in line.split(',') {
            let (step_x, step_y) = match item.chars().next().unwrap() {
                'L' => (-1, 0),
                'R' => (1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("bad input")
            };

            for _ in 0..item[1..].parse::<i32>().unwrap() {
                x += step_x;
                y += step_y;
                grid.entry((x, y)).or_default().insert(wire);
            }
        }
    }

    grid
        .into_iter()
        .filter(|(_, matches)| matches.len() as u8 == wires)
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap_or(0)
}

fn main() {
    println!("part 1: {}", find_intersection(include_str!("../input.txt")));
}
