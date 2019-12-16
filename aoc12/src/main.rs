use std::cmp::Ordering;
use std::collections::HashSet;

use num::Integer;
use regex::Regex;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Momentum {
    pos: i16,
    vel: i16,
}

impl Momentum {
    fn new(pos: i16) -> Momentum {
        Momentum { pos, vel: 0 }
    }
}

fn step(axis: &mut Vec<Momentum>) {
    for i in 0..axis.len() {
        for j in 0..i {
            let (dvi, dvj) = match axis[i].pos.cmp(&axis[j].pos) {
                Ordering::Less => (1, -1),
                Ordering::Greater => (-1, 1),
                Ordering::Equal => (0, 0),
            };
            axis[i].vel += dvi;
            axis[j].vel += dvj;
        }
    }

    for momentum in axis.iter_mut() {
        momentum.pos += momentum.vel;
    }
}

fn cycle(axis: &mut Vec<Momentum>) -> usize {
    let mut seen = HashSet::new();

    loop {
        seen.insert(axis.clone());
        step(axis);
        if seen.contains(axis) {
            break seen.len();
        }
    }
}

fn main() {
    let input = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>")
        .unwrap()
        .captures_iter(include_str!("../input.txt"))
        .map(|m| {
            (
                m[1].parse().unwrap(),
                m[2].parse().unwrap(),
                m[3].parse().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut axes = [
        input.iter().map(|p| Momentum::new(p.0)).collect::<Vec<_>>(),
        input.iter().map(|p| Momentum::new(p.1)).collect::<Vec<_>>(),
        input.iter().map(|p| Momentum::new(p.2)).collect::<Vec<_>>(),
    ];

    for a in axes.iter_mut() {
        for _ in 0..1000 {
            step(a);
        }
    }

    let energy: i16 = (0..input.len()).fold(0, |acc, i| {
        acc + axes.iter().fold(0, |m, a| m + a[i].pos.abs())
            * axes.iter().fold(0, |m, a| m + a[i].vel.abs())
    });
    println!("part 1: {}", energy);

    let cycles = axes.iter_mut().fold(1, |acc, a| acc.lcm(&cycle(a)));
    println!("part 2: {}", cycles);
}
