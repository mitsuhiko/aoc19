use interpreter::{parse_ascii_program, Machine};
use std::collections::BTreeMap;

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
        self.machine.feed(match self.look() {
            Color::Black => 0,
            Color::White => 1,
        });
        self.machine.step();
        let color = match self.machine.last_output() {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!(),
        };
        self.colors.insert(self.pos, color);
        self.machine.step();
        self.dir = match self.machine.last_output() {
            0 => self.dir.left(),
            1 => self.dir.right(),
            _ => unreachable!(),
        };
        self.pos.0 += self.dir.v().0;
        self.pos.1 += self.dir.v().1;
    }

    fn run(&mut self) {
        while !self.machine.halted() {
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
    let instructions = parse_ascii_program(include_str!("../input.txt"));

    let mut robot = Robot::new(&instructions, Color::Black);
    robot.run();
    println!("part 1: {}", robot.colored_squares());

    let mut robot = Robot::new(&instructions, Color::White);
    robot.run();
    println!("part 2:");
    robot.draw();
}
