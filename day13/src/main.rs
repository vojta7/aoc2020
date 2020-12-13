use std::{env::args, fs::read_to_string};

fn parse_bus_ids(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input
        .split(',')
        .filter(|&v| v != "x")
        .map(str::parse::<i32>)
        .collect()
}

fn part1(input: &str) -> Result<i32, std::num::ParseIntError> {
    let lines: Vec<_> = input.lines().collect();
    let arrival = lines[0].parse::<i32>()?;
    let bus_ids = parse_bus_ids(&lines[1])?;
    let (wait, id) = bus_ids
        .iter()
        .map(|v| {
            let m = arrival % v;
            if m != 0 {
                (v - m, v)
            } else {
                (m, v)
            }
        })
        .min_by(|(v1, _), (v2, _)| v1.cmp(v2))
        .unwrap();
    Ok(wait * id)
}

fn parse_bus_ids2(input: &str) -> Result<Vec<(i64, i64)>, std::num::ParseIntError> {
    input
        .split(',')
        .enumerate()
        .filter(|&(_, v)| v != "x")
        .map(|(idx, v)| {
            let v = str::parse::<i64>(v)?;
            Ok((v, v - idx as i64))
        })
        .collect()
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn part2(input: &str) -> Result<i64, std::num::ParseIntError> {
    let lines: Vec<_> = input.lines().collect();
    let bus_ids = parse_bus_ids2(&lines[1])?;
    let remainders: Vec<_> = bus_ids.iter().map(|(_, r)| r).collect();

    let prod: i64 = bus_ids.iter().map(|(v, _)| v).product();

    let inv = bus_ids
        .iter()
        .map(|(v, _)| {
            let (_, _, mut i) = egcd(*v, prod / v);
            if i < 0 {
                i += v;
            }
            i * (prod / v)
        })
        .collect::<Vec<_>>();

    let res: i64 = remainders
        .iter()
        .zip(inv.iter())
        .map(|(&r, &i)| *r as i64 * i)
        .sum();
    Ok(res % prod)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let p1 = part1(&input)?;
    let p2 = part2(&input)?;
    println!("part1: {}", p1);
    println!("part2: {}", p2);
    Ok(())
}
