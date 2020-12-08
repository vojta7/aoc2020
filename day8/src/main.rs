use std::{env::args, fs::read_to_string};

#[derive(Debug, Clone)]
enum Ins {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn split_at_once(input: &str, sep: char) -> Option<(&str, &str)> {
    let mut s = input.splitn(2, sep);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

fn parse_input(input: &str) -> impl Iterator<Item = Ins> + '_ {
    input
        .lines()
        .flat_map(|l| split_at_once(l, ' '))
        .flat_map(|(ins, num)| match ins {
            "acc" => Some(Ins::Acc(num.parse().ok()?)),
            "jmp" => Some(Ins::Jmp(num.parse().ok()?)),
            "nop" => Some(Ins::Nop(num.parse().ok()?)),
            _ => None,
        })
}

fn eval(instructions: &[Ins]) -> (bool, i32) {
    let mut counter = 0;
    let mut idx = 0;
    let mut visited = vec![false; instructions.len()];

    loop {
        if idx >= instructions.len() {
            break;
        }
        if visited[idx] {
            return (false, counter);
        }
        visited[idx] = true;

        match &instructions[idx] {
            Ins::Acc(val) => counter += val,
            Ins::Jmp(offset) => {
                idx = (idx as i32 + *offset) as usize;
                continue;
            }
            Ins::Nop(_) => (),
        }

        idx += 1;
    }

    (true, counter)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let instructions: Vec<_> = parse_input(&input).collect();
    let (_terminated, result1) = eval(instructions.as_slice());

    println!("part1: {}", result1);

    for idx in 0..instructions.len() {
        let replacement = match instructions[idx] {
            Ins::Nop(v) => Ins::Jmp(v),
            Ins::Jmp(v) => Ins::Nop(v),
            _ => continue,
        };

        let mut new_instructions = instructions.clone();
        new_instructions[idx] = replacement;

        if let (true, val) = eval(new_instructions.as_slice()) {
            println!("part2: {}", val);
            break;
        }
    }

    Ok(())
}
