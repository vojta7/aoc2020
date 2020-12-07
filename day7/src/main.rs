use std::{collections::VecDeque, env::args, fs::read_to_string};

fn split_at_once(input: &str, sep: &str) -> Option<(String, String)> {
    let mut s = input.splitn(2, sep);
    s.next()
        .and_then(|n| s.next().map(|v| (n.to_owned(), v.to_owned())))
}

fn parse_input(input: &str) -> impl Iterator<Item = (String, Vec<(i32, String)>)> + '_ {
    input
        .lines()
        .map(|line| {
            line.trim()
                .trim_end_matches('.')
                .replace(" bags", "")
                .replace(" bag", "")
        })
        .filter_map(|line| split_at_once(&line, " contain "))
        .map(|(first, rest)| match rest.as_str() {
            "no other" => (first, Vec::new()),
            rest => (
                first,
                rest.split(", ")
                    .filter_map(|v| {
                        split_at_once(v, " ").map(|(n, color)| (n.parse::<i32>().unwrap(), color))
                    })
                    .collect::<Vec<_>>(),
            ),
        })
}

fn part1(my_bag: &str, bags: &[(String, Vec<(i32, String)>)]) -> usize {
    let mut queue = VecDeque::new();
    let mut result = std::collections::HashSet::new();

    queue.push_back(my_bag);
    while let Some(current) = queue.pop_front() {
        for (bag, inner) in bags.iter() {
            if inner.iter().find(|&(_n, c)| c == current).is_some() {
                if !result.contains(bag) {
                    result.insert(bag);
                    queue.push_back(bag);
                }
            }
        }
    }

    result.len()
}

fn part2(my_bag: &str, bags: &[(String, Vec<(i32, String)>)]) -> i32 {
    let mut queue = VecDeque::new();
    let mut result = 0;

    queue.push_back((1, my_bag));
    while let Some((cnt, current)) = queue.pop_front() {
        if let Some((_bag, inner)) = bags.iter().find(|(bag, _)| bag == current) {
            for (count, color) in inner {
                result += cnt * count;
                queue.push_back((cnt * count, color));
            }
        }
    }

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let input = parse_input(&input).collect::<Vec<_>>();

    let my_bag = "shiny gold";

    let result1 = part1(my_bag, &input);
    println!("part1: {}", result1);

    let result2 = part2(my_bag, &input);
    println!("part2 {}", result2);

    Ok(())
}
