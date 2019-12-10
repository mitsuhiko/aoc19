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
    pub relative_base: i64,
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

    pub fn eval_all(&mut self) -> Vec<i64> {
        let mut rv = vec![];
        loop {
            self.step();
            if self.halted {
                break;
            }
            rv.push(self.output);
        }
        rv
    }

    pub fn step(&mut self) {
        loop {
            match self.mem_get(self.ip) % 100 {
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
                9 => {
                    self.relative_base += self.arg(1);
                    self.ip += 2;
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

    fn mem_get(&self, addr: usize) -> i64 {
        self.mem.get(addr).copied().unwrap_or(0)
    }

    fn mem_set(&mut self, addr: usize, value: i64) {
        self.mem.resize(self.mem.len().max(addr + 1), 0);
        self.mem[addr] = value;
    }

    fn get_mode(&self, arg: usize) -> i64 {
        let arg_modes = self.mem[self.ip] / 100;
        arg_modes / 10i64.pow((arg - 1) as u32) % 10
    }

    fn arg(&self, off: usize) -> i64 {
        let val = self.mem_get(self.ip + off);
        match self.get_mode(off) {
            0 => self.mem_get(val as usize),
            1 => val,
            2 => self.mem_get((self.relative_base + val) as usize),
            _ => panic!("wat"),
        }
    }

    fn put(&mut self, off: usize, val: i64) {
        let out = match self.get_mode(off) {
            0 => self.mem_get(self.ip + off) as usize,
            2 => (self.relative_base + self.mem_get(self.ip + off)) as usize,
            _ => panic!("wat"),
        };
        self.mem_set(out, val);
    }
}

fn main() {
    let instructions = load_program(include_str!("../input.txt"));
    println!("part 1: {:?}", Machine::new(&instructions, vec![1]).eval_all());
    println!("part 2: {:?}", Machine::new(&instructions, vec![2]).eval_all());
}
