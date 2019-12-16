pub fn parse_ascii_program(code: &str) -> Vec<i64> {
    code.trim()
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect()
}

#[derive(Default)]
pub struct Machine {
    mem: Vec<i64>,
    inputs: Vec<i64>,
    mem_input: i64,
    output: i64,
    relative_base: i64,
    ip: usize,
    halted: bool,
}

impl Machine {
    /// Creates the machine from the given memory instruction slice.
    pub fn new(mem: &[i64]) -> Machine {
        Machine {
            mem: mem.to_vec(),
            ..Default::default()
        }
    }

    /// Loads a machine from ASCII code.
    pub fn from_ascii_program(code: &str) -> Machine {
        Machine::new(&parse_ascii_program(code))
    }

    /// Feed some input into the machine.
    pub fn feed(&mut self, value: i64) {
        self.inputs.push(value);
    }

    /// Sets memory input.
    pub fn set_mem_input(&mut self, value: i64) {
        self.mem_input = value;
    }

    /// Returns an immutable view of the memory.
    pub fn mem(&self) -> &[i64] {
        &self.mem
    }

    /// Returns the last output produced
    pub fn last_output(&self) -> i64 {
        self.output
    }

    /// Returns the instruction pointer.
    pub fn ip(&self) -> usize {
        self.ip
    }

    /// Returns true if the machine halted.
    pub fn halted(&self) -> bool {
        self.halted
    }

    /// Runs until the machine stops returning the last output.
    pub fn eval(&mut self) -> i64 {
        while !self.halted() {
            self.step();
        }
        self.last_output()
    }

    /// Runs until the machine stops returning a vector of outputs.
    pub fn eval_multi(&mut self) -> Vec<i64> {
        let mut rv = Vec::new();
        loop {
            self.step();
            if self.halted() {
                break;
            } else {
                rv.push(self.last_output());
            }
        }
        rv
    }

    /// Runs a single iteration until either output happens or the
    /// machine halts without output.
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
                    let input = if self.inputs.is_empty() {
                        self.mem_input
                    } else {
                        self.inputs.remove(0)
                    };
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

    pub fn mem_get(&self, addr: usize) -> i64 {
        self.mem.get(addr).copied().unwrap_or(0)
    }

    pub fn mem_set(&mut self, addr: usize, value: i64) {
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
