struct Interpreter {
    memory: Vec<i64>,
}

impl Interpreter {
    fn new(instructions: &str) -> Interpreter {
        let instructions: Vec<i64> = instructions
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Interpreter {
            memory: instructions,
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
                    self.put(ip, 3, self.get_arg(ip, 1) + self.get_arg(ip, 2));
                    ip += 4;
                }
                2 => {
                    self.put(ip, 3, self.get_arg(ip, 1) * self.get_arg(ip, 2));
                    ip += 4;
                }
                3 => {
                    self.put(ip, 1, input);
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

fn eval(code: &str, input: i64) -> Vec<i64> {
    Interpreter::new(code).run(input)
}

fn main() {
    let input = include_str!("../input.txt");
    println!("part 1: {:?}", eval(input, 1));
    println!("part 2: {:?}", eval(input, 5));
}

#[test]
fn test_eval() {
    assert_eq!(
        eval(include_str!("../input.txt"), 1).last(),
        Some(&13_346_482)
    );
    assert_eq!(eval("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 0), vec![0]);
    assert_eq!(
        eval("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", 42),
        vec![1]
    );
    assert_eq!(eval("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 0), vec![0]);
    assert_eq!(eval("3,3,1105,-1,9,1101,0,0,12,4,12,99,1", 42), vec![1]);
    assert_eq!(eval("3,9,8,9,10,9,4,9,99,-1,8", 8), vec![1]);
    assert_eq!(eval("3,9,8,9,10,9,4,9,99,-1,8", 42), vec![0]);
    assert_eq!(eval("3,9,7,9,10,9,4,9,99,-1,8", 7), vec![1]);
    assert_eq!(eval("3,9,7,9,10,9,4,9,99,-1,8", 8), vec![0]);
    assert_eq!(eval("3,3,1108,-1,8,3,4,3,99", 8), vec![1]);
    assert_eq!(eval("3,3,1108,-1,8,3,4,3,99", 7), vec![0]);
    assert_eq!(eval("3,3,1107,-1,8,3,4,3,99", 7), vec![1]);
    assert_eq!(eval("3,3,1107,-1,8,3,4,3,99", 8), vec![0]);
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            7
        ),
        vec![999]
    );
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            8
        ),
        vec![1000]
    );
    assert_eq!(
        eval(
            "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
            9
        ),
        vec![1001]
    );
}
