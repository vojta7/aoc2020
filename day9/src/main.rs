use std::{cmp::Ordering, collections::HashSet, env::args, fs::read_to_string, num::ParseIntError};

const WINDOW: usize = 25;

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn find_subsequence(numbers: &[i64], target: i64) -> Option<(usize, usize)> {
    let (mut set_start, mut set_end) = (0, 0);
    while set_end < numbers.len() {
        if set_end - set_start < 2 {
            set_end += 1;
            continue;
        }

        let current_sum: i64 = numbers[set_start..=set_end].iter().sum();
        match target.cmp(&current_sum) {
            Ordering::Equal => return Some((set_start, set_end)),
            Ordering::Greater => set_end += 1,
            Ordering::Less => set_start += 1,
        }
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let numbers = parse_input(&input)?;

    for window in numbers.windows(WINDOW + 1) {
        let prev = &window[..WINDOW];
        let target = window[WINDOW];

        let prev_set: HashSet<_> = prev.iter().collect(); // TODO duplicates ???

        let mut found = false;
        for n in prev {
            if prev_set.contains(&(target - n)) {
                found = true;
                break;
            }
        }
        if !found {
            if let Some((s, e)) = find_subsequence(&numbers, target) {
                let range = &numbers[s..=e];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                println!("part1: {}", target);
                println!("part2: {}", min + max);
                break;
            }
        }
    }
    Ok(())
}
