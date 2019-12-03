use std::collections::BTreeMap;

trait DistanceMethod {
    type WireDistanceType: Default;
    fn store_wire_distance(_state: &mut Self::WireDistanceType, _distance: i32) {}
    fn calculate_distance(
        x: i32,
        y: i32,
        distances: &mut dyn Iterator<Item = &Self::WireDistanceType>,
    ) -> i32;
}

struct ManhattanMethod;

impl DistanceMethod for ManhattanMethod {
    type WireDistanceType = ();

    fn calculate_distance(
        x: i32,
        y: i32,
        _state: &mut dyn Iterator<Item = &Self::WireDistanceType>,
    ) -> i32 {
        x.abs() + y.abs()
    }
}

struct WireLengthMethod;

impl DistanceMethod for WireLengthMethod {
    type WireDistanceType = i32;

    fn store_wire_distance(distances: &mut Self::WireDistanceType, distance: i32) {
        *distances = distance;
    }

    fn calculate_distance(
        _x: i32,
        _y: i32,
        distances: &mut dyn Iterator<Item = &Self::WireDistanceType>,
    ) -> i32 {
        distances.sum()
    }
}

fn find_best_intersection<M: DistanceMethod>(input: &str) -> Option<i32> {
    let mut grid: BTreeMap<(i32, i32), BTreeMap<u8, M::WireDistanceType>> = Default::default();
    let mut wire_count = 0;

    for (wire, line) in input.lines().enumerate() {
        let mut x = 0i32;
        let mut y = 0i32;
        let mut distance = 0;
        for item in line.split(',') {
            let (step_x, step_y) = match item.chars().next()? {
                'L' => (-1, 0),
                'R' => (1, 0),
                'D' => (0, -1),
                'U' => (0, 1),
                _ => panic!("bad input"),
            };

            for _ in 0..item.get(1..)?.parse::<i32>().ok()? {
                x += step_x;
                y += step_y;
                distance += step_x.abs() + step_y.abs();
                M::store_wire_distance(
                    grid.entry((x, y))
                        .or_default()
                        .entry(wire as u8)
                        .or_default(),
                    distance,
                );
            }
        }
        wire_count = wire + 1;
    }

    grid.into_iter()
        .filter(|(_, matches)| matches.len() == wire_count)
        .map(|((x, y), wires)| M::calculate_distance(x, y, &mut wires.values()))
        .min()
}

fn main() {
    let input = include_str!("../input.txt");

    println!(
        "part 1: {}",
        find_best_intersection::<ManhattanMethod>(input).unwrap()
    );
    println!(
        "part 2: {}",
        find_best_intersection::<WireLengthMethod>(input).unwrap()
    );
}
