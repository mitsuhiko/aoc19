struct Interpreter {
    memory: Vec<i64>,
}

impl Interpreter {
    fn new(instructions: &str) -> Interpreter {
        let instructions: Vec<i64> = instructions
            .split(',')
            .filter_map(|x| x.parse().ok())
            .collect();
        Interpreter {
            memory: dbg!(instructions),
        }
    }

    fn get_arg(&self, ip: usize, off: usize) -> i64 {
        let arg_modes = self.memory[ip] / 100;
        let val = self.memory[ip + off];
        match arg_modes / 10i64.pow((off - 1) as u32) % 10 {
            0 => self.memory[val as usize],
            1 => val,
            _ => panic!("wat"),
        }
    }

    fn put(&mut self, ip: usize, off: usize, val: i64) {
        let out = self.memory[ip + off] as usize;
        self.memory[out] = val;
    }

    fn run(&mut self, input: i64) -> Vec<i64> {
        let mut out = vec![];
        let mut ip = 0;

        loop {
            match self.memory[ip] % 100 {
                1 => {
                    let a = self.get_arg(ip, 1);
                    let b = self.get_arg(ip, 2);
                    self.put(ip, 3, a + b);
                    ip += 4;
                }
                2 => {
                    let a = self.get_arg(ip, 1);
                    let b = self.get_arg(ip, 2);
                    self.put(ip, 3, a * b);
                    ip += 4;
                }
                3 => {
                    let val = input;
                    self.put(ip, 1, val);
                    ip += 2;
                }
                4 => {
                    out.push(self.get_arg(ip, 1));
                    ip += 2;
                }
                5 => {
                    if self.get_arg(ip, 1) != 0 {
                        ip = self.get_arg(ip, 2) as usize;
                    } else {
                        ip += 3;
                    }
                }
                6 => {
                    if self.get_arg(ip, 1) == 0 {
                        ip = self.get_arg(ip, 2) as usize;
                    } else {
                        ip += 3;
                    }
                }
                7 => {
                    if self.get_arg(ip, 1) < self.get_arg(ip, 2) {
                        self.put(ip, 3, 1);
                    } else {
                        self.put(ip, 3, 0);
                    }
                    ip += 4;
                }
                8 => {
                    if self.get_arg(ip, 1) == self.get_arg(ip, 2) {
                        self.put(ip, 3, 1);
                    } else {
                        self.put(ip, 3, 0);
                    }
                    ip += 4;
                }
                99 => {
                    break;
                }
                _ => {
                    panic!("this should not happen");
                }
            }
        }

        out
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut interpreter = Interpreter::new(input);
    println!("part 1: {:?}", interpreter.run(1));

    let mut interpreter = Interpreter::new(input);
    println!("part 2: {:?}", interpreter.run(5));
}
