use std::{env::args, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Position {
    fn scal_mult(&self, v: i32) -> Position {
        Position {
            x: self.x * v,
            y: self.y * v,
        }
    }
    fn rotate(&self, mut val: i32) -> Position {
        val = val.rem_euclid(360);
        assert!(val % 90 == 0);
        if val == 0 {
            *self
        } else {
            match val {
                90 => Position {
                    x: -self.y,
                    y: self.x,
                },
                180 => Position {
                    x: -self.x,
                    y: -self.y,
                },
                270 => Position {
                    x: self.y,
                    y: -self.x,
                },
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, mut val: i32) -> Direction {
        val = val.rem_euclid(360);
        assert!(val % 90 == 0);
        if val == 0 {
            *self
        } else {
            match self {
                Direction::North => Direction::East.rotate(val - 90),
                Direction::East => Direction::South.rotate(val - 90),
                Direction::South => Direction::West.rotate(val - 90),
                Direction::West => Direction::North.rotate(val - 90),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Forward(i32),
    Rotate(i32),
    Direction(Direction, i32),
}

fn parse_input(input: &str) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    input
        .lines()
        .map(|l| {
            let (ins, val) = l.split_at(1);
            let val = val.parse()?;
            Ok(match ins {
                "N" => Instruction::Direction(Direction::North, val),
                "E" => Instruction::Direction(Direction::East, val),
                "S" => Instruction::Direction(Direction::South, val),
                "W" => Instruction::Direction(Direction::West, val),
                "F" => Instruction::Forward(val),
                "R" => Instruction::Rotate(val),
                "L" => Instruction::Rotate(-val),
                _ => return Err(format!("Invalid instruction: {}", ins).into()),
            })
        })
        .collect()
}

fn dir_len_to_vec(length: i32, rotation: Direction) -> Position {
    match rotation {
        Direction::North => Position { x: length, y: 0 },
        Direction::East => Position { x: 0, y: length },
        Direction::South => Position { x: -length, y: 0 },
        Direction::West => Position { x: 0, y: -length },
    }
}

fn run_ship1(
    start: Position,
    rotation: Direction,
    instructions: &[Instruction],
) -> (Position, Direction) {
    instructions
        .iter()
        .fold((start, rotation), |(mut pos, mut rot), ins| {
            match ins {
                Instruction::Forward(v) => pos += dir_len_to_vec(*v, rot),
                Instruction::Rotate(v) => rot = rot.rotate(*v),
                Instruction::Direction(Direction::North, val) => pos.x += val,
                Instruction::Direction(Direction::East, val) => pos.y += val,
                Instruction::Direction(Direction::South, val) => pos.x -= val,
                Instruction::Direction(Direction::West, val) => pos.y -= val,
            };
            (pos, rot)
        })
}

fn run_ship2(
    start: Position,
    waypoint: Position,
    instructions: &[Instruction],
) -> (Position, Position) {
    instructions
        .iter()
        .fold((start, waypoint), |(mut pos, mut w), ins| {
            match ins {
                Instruction::Forward(v) => pos += w.scal_mult(*v),
                Instruction::Rotate(v) => w = w.rotate(*v),
                Instruction::Direction(Direction::North, val) => w.x += val,
                Instruction::Direction(Direction::East, val) => w.y += val,
                Instruction::Direction(Direction::South, val) => w.x -= val,
                Instruction::Direction(Direction::West, val) => w.y -= val,
            };
            (pos, w)
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let instructions = parse_input(&input)?;
    let start = Position { x: 0, y: 0 };
    let end = run_ship1(start, Direction::East, &instructions);
    println!("{}", end.0.x.abs() + end.0.y.abs());

    let end = run_ship2(start, Position { x: 1, y: 10 }, &instructions);
    println!("{}", end.0.x.abs() + end.0.y.abs());
    Ok(())
}
