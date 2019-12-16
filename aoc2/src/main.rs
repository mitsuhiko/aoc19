use interpreter::{parse_ascii_program, Machine};

fn main() {
    let instructions = parse_ascii_program(include_str!("../input.txt"));
    let mut machine = Machine::new(&instructions);
    machine.mem_set(1, 12);
    machine.mem_set(2, 2);
    machine.eval();

    println!("part 1: {}", machine.mem_get(0));

    for a in 0..=99 {
        for b in 0..=99 {
            let mut machine = Machine::new(&instructions);
            machine.mem_set(1, a);
            machine.mem_set(2, b);
            machine.eval();
            if machine.mem_get(0) == 19_690_720 {
                println!("part 2: {}", 100 * machine.mem_get(1) + machine.mem_get(2));
                return;
            }
        }
    }
}
