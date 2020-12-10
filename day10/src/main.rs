use std::{env::args, fs::read_to_string, num::ParseIntError};

fn parse_input(input: &str) -> Result<Vec<i64>, ParseIntError> {
    input.lines().map(|l| l.parse()).collect()
}

fn solve(numbers: &[i64]) -> impl Iterator<Item = i64> + '_ {
    numbers.iter().scan(0, |state, &val| {
        let res = val - *state;
        *state = val;
        if res > 3 {
            panic!(res);
        }
        Some(res)
    })
}

fn part1(numbers: &[i64]) -> i32 {
    let (ones, threes) = solve(&numbers).fold((0, 1), |(ones, threes), val| match val {
        1 => (ones + 1, threes),
        3 => (ones, threes + 1),
        _ => (ones, threes),
    });
    ones * threes
}

fn part2(numbers: &[i64], cache: &mut Vec<Option<i64>>, idx: usize, last: i64, target: i64) -> i64 {
    let mut total = 0;
    if let Some(val) = cache.get(idx).cloned().flatten() {
        return val;
    }
    for (i, n) in numbers.iter().enumerate() {
        if n - last <= 3 {
            let res = part2(&numbers[i + 1..], cache, idx + i + 1, *n, target);
            total += res;
        } else {
            break;
        }
    }
    if target - last <= 3 {
        total += 1;
    } else if total != 0 {
        cache[idx] = Some(total);
    }
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let mut numbers = parse_input(&input)?;
    numbers.sort_unstable();

    let part1 = part1(&numbers);
    let mut cache = vec![None; numbers.len()];
    let part2 = part2(&numbers, &mut cache, 0, 0, numbers.last().unwrap() + 3);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
    Ok(())
}
