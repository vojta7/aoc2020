use std::{env::args, fs::read_to_string, num::ParseIntError, ops::RangeInclusive};

#[derive(Debug, Clone)]
struct Rule<'a> {
    name: &'a str,
    r1: RangeInclusive<i32>,
    r2: RangeInclusive<i32>,
}

fn split_at_once<'a, 'b>(input: &'a str, sep: &'b str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, sep);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

fn split_input(input: &str) -> Option<(&str, &str, &str)> {
    let mut split = input.trim().split("\n\n");
    Some((split.next()?, split.next()?, split.next()?))
}

fn parse_range(input: &str) -> Result<RangeInclusive<i32>, ParseIntError> {
    let (start, end) = split_at_once(input, "-").unwrap();
    Ok(RangeInclusive::new(start.parse()?, end.parse()?))
}

fn parse_rules(rules: &str) -> impl Iterator<Item = Rule> {
    rules.lines().map(|line| {
        let (name, rest) = split_at_once(line, ": ").unwrap();
        let (r1, r2) = split_at_once(rest, " or ").unwrap();
        Rule {
            name,
            r1: parse_range(r1).unwrap(),
            r2: parse_range(r2).unwrap(),
        }
    })
}

fn parse_tickest(input: &str) -> Result<Vec<Vec<i32>>, ParseIntError> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            line.trim()
                .split(',')
                .map(str::parse::<i32>)
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect()
}

fn find_invalid<'a>(ticket: &'a [i32], rules: &'a [Rule]) -> impl Iterator<Item = &'a i32> {
    ticket.iter().filter(move |val| {
        !rules
            .iter()
            .any(|rule| rule.r1.contains(val) || rule.r2.contains(val))
    })
}

fn part1(nearby: &[Vec<i32>], rules: &[Rule]) -> i32 {
    nearby.iter().fold(0, |acc, ticket| {
        acc + find_invalid(ticket, rules).sum::<i32>()
    })
}

fn part2(nearby: &[Vec<i32>], mut rules: Vec<Rule>, my_ticket: &[i32]) -> i64 {
    let only_valid: Vec<_> = nearby
        .iter()
        .filter(|ticket| find_invalid(ticket, &rules).next().is_none())
        .enumerate()
        .collect();

    let mut sorted_rules = Vec::new();
    while !rules.is_empty() {
        let mut rule_used = false;
        for idx in 0..only_valid[0].1.len() {
            let mut total = 0;
            let mut valid = Vec::new();
            for (ridx, rule) in rules.iter().enumerate() {
                let n: Vec<_> = only_valid.iter().map(|t| t.1[idx]).collect();
                if n.iter()
                    .all(|val| rule.r1.contains(&val) || rule.r2.contains(&val))
                {
                    total += 1;
                    valid.push(ridx);
                }
            }
            if total == 1 {
                sorted_rules.push((only_valid[idx].0, rules.remove(valid[0])));
                rule_used = true;
            }
        }
        if !rule_used {
            panic!("Flawed alghoritm")
        }
    }

    assert_eq!(sorted_rules.len(), my_ticket.len());
    let mut result: i64 = 1;
    for (column_idx, rule) in sorted_rules {
        if rule.name.starts_with("departure") {
            result *= my_ticket[column_idx] as i64;
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let (rules, my, nearby) = split_input(&input).unwrap();
    let rules = parse_rules(&rules).collect::<Vec<_>>();
    let my_ticket = parse_tickest(&my)?[0].clone();
    let nearby = parse_tickest(&nearby)?;

    println!("part1: {}", part1(&nearby, &rules));
    println!("part2: {}", part2(&nearby, rules.clone(), &my_ticket));

    Ok(())
}
