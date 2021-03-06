use std::{env::args, fs::read_to_string};

fn parse_input(input: &str) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(match c {
                        '#' => true,
                        '.' => false,
                        c => return Err(format!("Invalid char in input: {}", c).into()),
                    })
                })
                .collect()
        })
        .collect()
}

fn initialize_map(initial_state: &[Vec<bool>], rounds: usize) -> Vec<Vec<Vec<bool>>> {
    let mut map = vec![
        vec![
            vec![false; initial_state[0].len() + rounds * 2];
            initial_state.len() + rounds * 2
        ];
        rounds * 2 + 1
    ];

    for row in 0..initial_state.len() {
        for tile in 0..initial_state[row].len() {
            map[rounds][row + rounds][tile + rounds] = initial_state[row][tile];
        }
    }

    map
}

fn count_neighbors(map: &[Vec<Vec<bool>>], (xp, yp, zp): (usize, usize, usize)) -> i32 {
    let mut neighbors = 0;
    for x in xp as i32 - 1..=xp as i32 + 1 {
        if x < 0 {
            continue;
        }
        for y in yp as i32 - 1..=yp as i32 + 1 {
            if y < 0 {
                continue;
            }
            for z in zp as i32 - 1..=zp as i32 + 1 {
                if z < 0 {
                    continue;
                }
                if let Some(true) = map
                    .get(x as usize)
                    .map(|p| p.get(y as usize).map(|r| r.get(z as usize)))
                    .flatten()
                    .flatten()
                {
                    neighbors += 1;
                }
            }
        }
    }
    if map[xp as usize][yp as usize][zp as usize] {
        neighbors -= 1;
    }
    neighbors
}

fn simulate_round(map: &[Vec<Vec<bool>>]) -> Vec<Vec<Vec<bool>>> {
    let height = map.len();
    let width = map[0].len();
    let depth = map[0][0].len();
    let mut new_map = vec![vec![vec![false; depth]; width]; height];

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            for z in 0..map[x][y].len() {
                match count_neighbors(map, (x, y, z)) {
                    2 | 3 if map[x][y][z] => new_map[x][y][z] = true,
                    3 if !map[x][y][z] => new_map[x][y][z] = true,
                    _ => new_map[x][y][z] = false,
                }
            }
        }
    }

    new_map
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let initial_state = parse_input(&input)?;
    let rounds = 6;

    let mut map = initialize_map(&initial_state, rounds);

    for _i in 0..rounds {
        map = simulate_round(&map);
    }

    let total: usize = map
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|row| row.iter().filter(|&&v| v).count())
                .sum::<usize>()
        })
        .sum();

    println!("{}", total);
    Ok(())
}
