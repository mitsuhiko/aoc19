use std::collections::BTreeMap;

enum DistanceMethod {
    Manhattan,
    WireLength,
}

fn find_intersection(input: &str, method: DistanceMethod) -> i32 {
    let mut grid: BTreeMap<(i32, i32), BTreeMap<u8, i32>> = Default::default();
    let mut wires = 0u8;

    for line in input.lines() {
        let mut x = 0i32;
        let mut y = 0i32;
        let wire = wires;
        wires += 1;
        let mut distance = 0;
        for item in line.split(',') {
            let (step_x, step_y) = match item.chars().next().unwrap() {
                'L' => (-1, 0),
                'R' => (1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("bad input"),
            };

            for _ in 0..item[1..].parse::<i32>().unwrap() {
                x += step_x;
                y += step_y;
                distance += step_x.abs() + step_y.abs();
                *grid.entry((x, y)).or_default().entry(wire).or_default() += distance;
            }
        }
    }

    grid.into_iter()
        .filter(|(_, matches)| matches.len() as u8 == wires)
        .map(|((x, y), wires)| match method {
            DistanceMethod::Manhattan => x.abs() + y.abs(),
            DistanceMethod::WireLength => wires.values().sum::<i32>(),
        })
        .min()
        .unwrap_or(0)
}

fn main() {
    let input = include_str!("../input.txt");
    println!(
        "part 1: {}",
        find_intersection(input, DistanceMethod::Manhattan)
    );
    println!(
        "part 2: {}",
        find_intersection(input, DistanceMethod::WireLength)
    );
}
