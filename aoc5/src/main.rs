use interpreter::Machine;

fn eval(code: &str, input: i64) -> Vec<i64> {
    let mut machine = Machine::from_ascii_program(&code);
    machine.feed(input);
    machine.eval_multi()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {:?}", eval(input, 1));
    println!("part 2: {:?}", eval(input, 5));
}
