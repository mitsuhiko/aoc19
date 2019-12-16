use std::cmp::Ordering;

use interpreter::Machine;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

pub struct Game {
    machine: Machine,
    score: usize,
}

impl Game {
    fn new() -> Game {
        Game {
            machine: Machine::from_ascii_program(include_str!("../input.txt")),
            score: 0,
        }
    }

    pub fn set_free_play(&mut self) {
        self.machine.mem_set(0, 2);
    }

    fn step(&mut self) -> Option<i64> {
        self.machine.step();
        if self.machine.halted() {
            None
        } else {
            Some(self.machine.last_output())
        }
    }

    fn input(&mut self, input: i64) -> Option<(u8, u8, Tile)> {
        loop {
            self.machine.set_mem_input(input);
            let x = self.step()? as u8;
            let y = self.step()? as u8;
            if x == !0 && y == 0 {
                self.score = self.step()? as _;
            } else {
                let tile = match self.step()? {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => Tile::Paddle,
                    4 => Tile::Ball,
                    _ => panic!("oh no"),
                };
                return Some((x, y, tile));
            }
        }
    }
}

fn find_starting_blocks() -> usize {
    let mut game = Game::new();
    let mut count = 0;
    while let Some((_, _, tile)) = game.input(0) {
        if tile == Tile::Block {
            count += 1;
        }
    }
    count
}

fn play_perfect_game() -> usize {
    let mut game = Game::new();
    game.set_free_play();

    let mut input = 0;
    let mut paddle_x = 0;
    let mut ball_x = 0;

    while let Some((x, _, tile)) = game.input(input) {
        match tile {
            Tile::Paddle => paddle_x = x,
            Tile::Ball => ball_x = x,
            _ => {}
        }

        input = match ball_x.cmp(&paddle_x) {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        };
    }

    game.score
}

fn main() {
    println!("part 1: {}", find_starting_blocks());
    println!("part 2: {}", play_perfect_game());
}
