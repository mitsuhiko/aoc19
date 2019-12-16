use interpreter::{parse_ascii_program, Machine};
use itertools::Itertools;

fn try_permutations<F: FnMut(Vec<i64>) -> i64>(func: F) -> i64 {
    (0..5).permutations(5).map(func).max().unwrap()
}

fn find_max_amplification(code: &[i64]) -> i64 {
    try_permutations(|x| {
        x.into_iter().fold(0, |i, s| {
            let mut machine = Machine::new(code);
            machine.feed(s);
            machine.feed(i);
            machine.eval()
        })
    })
}

fn find_max_amplification_feedback(code: &[i64]) -> i64 {
    try_permutations(|x| {
        let mut machines: Vec<_> = x
            .into_iter()
            .map(|x| {
                let mut machine = Machine::new(code);
                machine.feed(x + 5);
                machine
            })
            .collect();
        let mut input = 0;
        while !machines[4].halted() {
            for machine in machines.iter_mut() {
                machine.feed(input);
                machine.step();
                input = machine.last_output();
            }
        }
        machines[4].last_output()
    })
}

fn main() {
    let instructions = parse_ascii_program(include_str!("../input.txt"));
    println!("part 1: {}", find_max_amplification(&instructions));
    println!("part 2: {}", find_max_amplification_feedback(&instructions));
}
