use std::collections::BTreeMap;

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
    pub fn new(mem: &[i64]) -> Machine {
        Machine {
            mem: mem.to_vec(),
            ..Default::default()
        }
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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn right(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn v(self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

type Point = (i64, i64);

struct Robot {
    machine: Machine,
    pos: Point,
    dir: Direction,
    colors: BTreeMap<Point, Color>,
    default_color: Color,
}

impl Robot {
    fn new(instructions: &[i64], default_color: Color) -> Robot {
        Robot {
            machine: Machine::new(instructions),
            pos: (0, 0),
            dir: Direction::Up,
            colors: BTreeMap::default(),
            default_color,
        }
    }

    fn look(&self) -> Color {
        self.colors
            .get(&self.pos)
            .copied()
            .unwrap_or(self.default_color)
    }

    fn step(&mut self) {
        self.machine.inputs.push(match self.look() {
            Color::Black => 0,
            Color::White => 1,
        });
        self.machine.step();
        let color = match self.machine.output {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        self.colors.insert(self.pos, color);
        self.machine.step();
        self.dir = match self.machine.output {
            0 => self.dir.left(),
            1 => self.dir.right(),
            _ => unreachable!(),
        };
        self.pos.0 += self.dir.v().0;
        self.pos.1 += self.dir.v().1;
    }

    fn run(&mut self) {
        while !self.machine.halted {
            self.step();
        }
    }

    fn colored_squares(&self) -> usize {
        self.colors.len()
    }

    fn draw(&self) {
        let min_x = self.colors.keys().map(|x| x.0).min().unwrap();
        let min_y = self.colors.keys().map(|x| x.1).min().unwrap();
        let max_x = self.colors.keys().map(|x| x.0).max().unwrap();
        let max_y = self.colors.keys().map(|x| x.1).max().unwrap();
        let mut rv = String::new();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.colors.get(&(x, y)) {
                    Some(Color::Black) | None => rv.push(' '),
                    Some(Color::White) => rv.push('*'),
                }
            }
            rv.push('\n');
        }

        println!("{}", rv.trim_end());
    }
}

fn main() {
    let instructions = load_program(include_str!("../input.txt"));

    let mut robot = Robot::new(&instructions, Color::Black);
    robot.run();
    println!("part 1: {}", robot.colored_squares());

    let mut robot = Robot::new(&instructions, Color::White);
    robot.run();
    println!("part 2:");
    robot.draw();
}
