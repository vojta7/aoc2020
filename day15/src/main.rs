use std::{collections::HashMap, env::args, fs::read_to_string, num::ParseIntError};

fn parse_input(input: &str) -> impl Iterator<Item = Result<i32, ParseIntError>> + '_ {
    input.trim().split(',').map(str::parse::<i32>)
}

fn run(start_round: i32, max_round: i32, played: &mut HashMap<i32, i32>, mut n: i32) -> i32 {
    for current_round in start_round..max_round {
        if let Some(last_played) = played.get(&n).cloned() {
            played.insert(n, current_round);
            n = current_round - last_played;
        } else {
            played.insert(n, current_round);
            n = 0;
        }
    }
    n
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let nums = parse_input(&input).collect::<Result<Vec<_>, _>>()?;
    let mut played: HashMap<_, _> = nums[..nums.len() - 1]
        .iter()
        .enumerate()
        .map(|(i, v)| (*v, i as i32 + 1))
        .collect();
    let start_round = nums.len() as i32;

    let mut played1 = played.clone();
    let part1 = run(start_round, 2020, &mut played1, *nums.last().unwrap());
    println!("{}", part1);

    let part2 = run(start_round, 30000000, &mut played, *nums.last().unwrap());
    println!("{}", part2);
    Ok(())
}
