use itertools::Itertools;

fn load_program(code: &str) -> Vec<i64> {
    code.trim()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect()
}

#[derive(Default)]
struct Machine {
    pub mem: Vec<i64>,
    pub inputs: Vec<i64>,
    pub output: i64,
    pub ip: usize,
    pub halted: bool,
}

impl Machine {
    pub fn new(mem: &[i64], inputs: Vec<i64>) -> Machine {
        Machine {
            mem: mem.to_vec(),
            inputs,
            ..Default::default()
        }
    }

    pub fn eval(&mut self) -> i64 {
        while !self.halted {
            self.step();
        }
        self.output
    }

    pub fn step(&mut self) {
        loop {
            match self.mem[self.ip] % 100 {
                1 => {
                    let a = self.arg(1);
                    let b = self.arg(2);
                    self.put(3, a + b);
                    self.ip += 4;
                }
                2 => {
                    let a = self.arg(1);
                    let b = self.arg(2);
                    self.put(3, a * b);
                    self.ip += 4;
                }
                3 => {
                    let input = self.inputs.remove(0);
                    self.put(1, input);
                    self.ip += 2;
                }
                4 => {
                    self.output = self.arg(1);
                    self.ip += 2;
                    return;
                }
                5 => {
                    if self.arg(1) != 0 {
                        self.ip = self.arg(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.arg(1) == 0 {
                        self.ip = self.arg(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    if self.arg(1) < self.arg(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                8 => {
                    if self.arg(1) == self.arg(2) {
                        self.put(3, 1);
                    } else {
                        self.put(3, 0);
                    }
                    self.ip += 4;
                }
                99 => {
                    self.halted = true;
                    return;
                }
                _ => {
                    panic!("this should not happen");
                }
            }
        }
    }

    fn arg(&self, off: usize) -> i64 {
        let arg_modes = self.mem[self.ip] / 100;
        let val = self.mem[self.ip + off];
        match arg_modes / 10i64.pow((off - 1) as u32) % 10 {
            0 => self.mem[val as usize],
            1 => val,
            _ => panic!("wat"),
        }
    }

    fn put(&mut self, off: usize, val: i64) {
        let out = self.mem[self.ip + off] as usize;
        self.mem[out] = val;
    }
}

fn try_permutations<F: FnMut(Vec<i64>) -> i64>(mut func: F) -> i64 {
    (0..5)
        .permutations(5)
        .map(|permutations| func(permutations))
        .max()
        .unwrap()
}

fn find_max_amplification(code: &[i64]) -> i64 {
    try_permutations(|x| {
        x.into_iter()
            .fold(0, |i, s| Machine::new(code, vec![s, i]).eval())
    })
}

fn find_max_amplification_feedback(code: &[i64]) -> i64 {
    try_permutations(|x| {
        let mut machines: Vec<_> = x
            .into_iter()
            .map(|x| Machine::new(code, vec![x + 5]))
            .collect();
        let mut input = 0;
        while !machines[4].halted {
            for machine in machines.iter_mut() {
                machine.inputs.push(input);
                machine.step();
                input = machine.output;
            }
        }
        machines[4].output
    })
}

fn main() {
    let instructions = load_program(include_str!("../input.txt"));
    println!("part 1: {}", find_max_amplification(&instructions));
    println!("part 2: {}", find_max_amplification_feedback(&instructions));
}
