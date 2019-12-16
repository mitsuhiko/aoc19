use interpreter::{parse_ascii_program, Machine};

fn eval(code: &[i64], input: i64) -> i64 {
    let mut machine = Machine::new(code);
    machine.feed(input);
    machine.eval()
}

fn main() {
    let instructions = parse_ascii_program(include_str!("../input.txt"));
    println!("part 1: {}", eval(&instructions, 1));
    println!("part 2: {}", eval(&instructions, 2));
}
