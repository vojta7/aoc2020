use std::{env::args, fs::read_to_string};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Occupied,
    Empty,
    Floor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Strategy {
    Visible,
    Close,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "w: {}", self.width)?;
        writeln!(f, "h: {}", self.height)?;
        for x in 0..self.height {
            for y in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.tiles[x][y] {
                        Tile::Empty => 'L',
                        Tile::Occupied => '#',
                        Tile::Floor => '.',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    const DIRECTIONS: [(i32, i32); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    pub fn round(&self, strategy: Strategy) -> Map {
        let mut new_map = self.clone();
        let (neighbors_f, threshold): (&dyn Fn(&Map, usize, usize) -> usize, usize) = match strategy
        {
            Strategy::Close => (&Map::count_neighbors, 4),
            Strategy::Visible => (&Map::count_visible_neighbors, 5),
        };
        for x in 0..self.height {
            for y in 0..self.width {
                if let Tile::Empty | Tile::Occupied = self.tiles[x][y] {
                    let cnt = neighbors_f(&self, x, y);
                    match cnt {
                        0 => new_map.tiles[x][y] = Tile::Occupied,
                        n if n >= threshold => new_map.tiles[x][y] = Tile::Empty,
                        _ => (),
                    }
                }
            }
        }
        new_map
    }
    fn count_visible_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for &(xo, yo) in Map::DIRECTIONS.iter() {
            let (mut x, mut y) = (x as i32, y as i32);
            loop {
                x += xo;
                y += yo;
                match self
                    .tiles
                    .get(x as usize)
                    .map(|row| row.get(y as usize))
                    .flatten()
                {
                    Some(Tile::Occupied) => {
                        count += 1;
                        break;
                    }
                    None | Some(Tile::Empty) => break,
                    _ => (),
                }
            }
        }
        count
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let (x, y) = (x as i32, y as i32);
        let mut count = 0;
        for &(xo, yo) in Map::DIRECTIONS.iter() {
            if xo == 0 && yo == 0 || x + xo < 0 || y + yo < 0 {
                continue;
            }
            if std::matches!(
                self.tiles
                    .get((x + xo) as usize)
                    .map(|row| row.get((y + yo) as usize)),
                Some(Some(Tile::Occupied))
            ) {
                count += 1;
            }
        }
        count
    }

    pub fn count(&self, tile: Tile) -> usize {
        self.tiles
            .iter()
            .map(|row| row.iter().filter(|&&t| t == tile).count())
            .sum()
    }
}

fn parse_input(input: &str) -> Map {
    Map {
        tiles: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'L' => Tile::Empty,
                        '#' => Tile::Occupied,
                        _ => Tile::Floor,
                    })
                    .collect()
            })
            .collect(),
        width: input.lines().next().map(|l| l.chars().count()).unwrap_or(0),
        height: input.lines().count(),
    }
}

fn solve(map: &Map, strategy: Strategy) -> usize {
    let mut map = map.clone();
    loop {
        let new_map = map.round(strategy);
        if new_map == map {
            break map.count(Tile::Occupied);
        }
        map = new_map;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let map = parse_input(&input);

    let part1 = solve(&map, Strategy::Close);
    let part2 = solve(&map, Strategy::Visible);
    println!("part1: {}", part1);
    println!("part2: {}", part2);

    Ok(())
}
