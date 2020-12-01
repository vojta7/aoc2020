use std::{env::args, fs::read_to_string};

fn solve2(numbers: &[i32], target: i32) -> Option<(i32, i32)> {
    let (mut start, mut end) = (0, numbers.len() - 1);
    while start < end {
        match (numbers[start], numbers[end]) {
            (c1, c2) if c1 + c2 == target => return Some((c1, c2)),
            (c1, c2) if c1 + c2 > target => end -= 1,
            _ => start += 1,
        }
    }
    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let target: i32 = args().nth(2).ok_or("No input target number")?.parse()?;

    let mut numbers: Vec<_> = input
        .lines()
        .filter_map(|line| str::parse::<i32>(&line).ok())
        .collect();
    numbers.sort_unstable();

    if let Some((c1, c2)) = solve2(&numbers, target) {
        println!("p1: {} * {} == {}", c1, c2, c1 * c2);
    }

    if let Some((cur, c1, c2)) = (0..numbers.len() - 2)
        .filter_map(|i| solve2(&numbers[i..], target - numbers[i]).map(|(a, b)| (numbers[i], a, b)))
        .next()
    {
        println!("p2: {} * {} * {} == {}", cur, c1, c2, cur * c1 * c2);
    }
    Ok(())
}
