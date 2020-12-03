use std::{env::args, fs::read_to_string};

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Tree,
}

struct Move {
    x: usize,
    y: usize,
}

impl Move {
    fn new(x: usize, y: usize) -> Self {
        Move { x, y }
    }
}

type Map = Vec<Vec<Tile>>;

fn parse_input(input: &str) -> Map {
    input.trim().chars().fold(vec![Vec::new()], |mut acc, c| {
        match c {
            '.' => acc.last_mut().unwrap().push(Tile::Empty),
            '#' => acc.last_mut().unwrap().push(Tile::Tree),
            '\n' => acc.push(Vec::new()),
            _ => (),
        }
        acc
    })
}

fn solve(map: &Map, moves: &Move) -> i64 {
    let map_width = map[0].len();
    (0..map.len())
        .step_by(moves.x)
        .fold((0, 0), |(trees, y), x| match map[x][y] {
            Tile::Tree => (trees + 1, (y + moves.y) % map_width),
            Tile::Empty => (trees, (y + moves.y) % map_width),
        })
        .0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let map = parse_input(&input);
    let moves = vec![
        Move::new(1, 1),
        Move::new(1, 3),
        Move::new(1, 5),
        Move::new(1, 7),
        Move::new(2, 1),
    ];

    let result1 = solve(&map, &Move { x: 1, y: 3 });
    println!("part1: {}", result1);

    let result2: i64 = moves.iter().map(|mov| solve(&map, mov)).product();
    println!("part2: {}", result2);
    Ok(())
}
