use std::{env::args, fs::read_to_string};

fn parse_input<'a>(input: &'a str) -> impl Iterator<Item = u16> + 'a {
    input.trim().split('\n').map(|row| {
        row.chars()
            .map(|c| match c {
                'B' | 'R' => 1,
                _ => 0,
            })
            .fold(0, |acc, b| acc * 2 + b)
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;

    let (min, max, prod) = parse_input(&input)
        .fold((u16::MAX, u16::MIN, 0), |(min, max, prod), v| {
            (min.min(v), max.max(v), prod ^ v)
        });

    let my_seat = (min..=max).fold(prod, std::ops::BitXor::bitxor);

    println!("part1: {}", max);
    println!("part2: {}", my_seat);

    Ok(())
}
