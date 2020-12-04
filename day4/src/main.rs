use std::{collections::HashMap, env::args, fs::read_to_string};

fn split_at_once(input: &str, sep: char) -> Option<(&str, &str)> {
    let mut s = input.splitn(2, sep);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

struct Passpord<'a> {
    eyr: i32,
    byr: i32,
    iyr: i32,
    hgt: (i32, &'a str),
    ecl: &'a str,
    hcl: &'a str,
    pid: &'a str,
    #[allow(unused)]
    cid: Option<&'a str>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Passpord> {
    input
        .split("\n\n")
        .map(|record| {
            record
                .split_whitespace()
                .filter_map(|field| split_at_once(field, ':'))
                .collect::<HashMap<_, _>>()
        })
        .filter_map(|passpord| {
            let hgt = passpord.get("hgt")?;
            let unit_pos = hgt
                .chars()
                .position(|c| c.is_alphabetic())
                .unwrap_or_else(|| hgt.len());
            let (val, unit) = passpord.get("hgt")?.split_at(unit_pos);
            let height = (val.parse().ok()?, unit);
            Some(Passpord {
                eyr: passpord.get("eyr")?.parse().ok()?,
                byr: passpord.get("byr")?.parse().ok()?,
                iyr: passpord.get("iyr")?.parse().ok()?,
                ecl: passpord.get("ecl")?,
                hgt: height,
                hcl: passpord.get("hcl")?,
                pid: passpord.get("pid")?,
                cid: passpord.get("cid").copied(),
            })
        })
}

fn is_valid(passpord: &Passpord) -> bool {
    let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    passpord.byr >= 1920
        && passpord.byr <= 2002
        && passpord.iyr >= 2010
        && passpord.iyr <= 2020
        && passpord.eyr >= 2020
        && passpord.eyr <= 2030
        && (match passpord.hgt {
            (val, "cm") => val >= 150 && val <= 193,
            (val, "in") => val >= 59 && val <= 76,
            _ => false,
        })
        && &passpord.hcl[..1] == "#"
        && passpord.hcl[1..]
            .chars()
            .all(|c| (c.is_digit(10) || c.is_lowercase()) && c.is_digit(16))
        && valid_ecl.contains(&passpord.ecl)
        && passpord.pid.len() == 9
        && passpord.pid.chars().all(|c| c.is_digit(10))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let passpords: Vec<_> = parse_input(&input).collect();

    let part1 = passpords.len();
    println!("Part1: {}", part1);

    let part2 = passpords.iter().filter(|p| is_valid(p)).count();
    println!("Part2: {}", part2);
    Ok(())
}
