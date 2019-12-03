fn run_intcode(instructions: &[i64], a: i64, b: i64) -> Vec<i64> {
    let mut memory = instructions.to_owned();
    memory[1] = a;
    memory[2] = b;

    let mut ip = 0;
    loop {
        match memory[ip] {
            op @ 1 ..= 2 => {
                let a = memory[memory[ip + 1] as usize];
                let b = memory[memory[ip + 2] as usize];
                let out = memory[ip + 3] as usize;
                memory[out] = if op == 1 { a + b } else { a * b };
                ip += 4;
            }
            99 => { break; }
            _ => { panic!("this should not happen"); }
        }
    }

    memory
}


fn main() {
    let instructions: Vec<i64> = include_str!("../input.txt")
        .split(',')
        .filter_map(|x| x.parse().ok())
        .collect();

    let memory = run_intcode(&instructions, 12, 2);
    println!("part 1: {}", memory[0]);

    for a in 0..=99 {
        for b in 0..=99 {
            let memory = run_intcode(&instructions, a, b);
            if memory[0] == 19_690_720 {
                println!("part 2: {}", 100 * memory[1] + memory[2]);
                return;
            }
        }
    }
}
