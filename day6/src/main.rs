use std::{collections::HashSet, env::args, fs::read_to_string};

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.trim().chars().collect::<HashSet<_>>())
                .fold(None, |acc, ans| match acc {
                    None => Some(ans),
                    Some(acc) => Some(acc.intersection(&ans).cloned().collect()),
                })
                .map(|v| v.len())
                .unwrap_or(0)
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;

    let total1 = part1(&input);
    let total2 = part2(&input);

    println!("part1: {}", total1);
    println!("part2: {}", total2);

    Ok(())
}
