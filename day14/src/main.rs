use std::{collections::HashMap, env::args, fs::read_to_string, num::ParseIntError};

#[derive(Debug)]
enum Instruction {
    Memory(i64, i64),
    Mask(Vec<char>),
}

fn parse_input(input: &str) -> impl Iterator<Item = Result<Instruction, ParseIntError>> + '_ {
    input
        .lines()
        .filter_map(|l| {
            let mut s = l.split(" = ");
            Some((s.next()?, s.next()?))
        })
        .map(|(ins, val)| {
            Ok(match ins {
                "mask" => {
                    let val = val.chars().rev().collect();
                    Instruction::Mask(val)
                }
                _ => {
                    let addr = ins.strip_prefix("mem[").unwrap();
                    let addr = addr.strip_suffix("]").unwrap();
                    Instruction::Memory(addr.parse()?, val.parse()?)
                }
            })
        })
}

fn run1(memory: &mut HashMap<i64, i64>, instructions: &[Instruction]) {
    let mut mask = Vec::new();
    for ins in instructions {
        match ins {
            Instruction::Memory(a, v) => {
                let mut val = *v;
                mask.iter().enumerate().for_each(|(idx, v)| match v {
                    '0' => val &= !(1 << idx),
                    '1' => val |= 1 << idx,
                    _ => (),
                });
                memory.insert(*a, val);
            }
            Instruction::Mask(v) => mask = v.clone(),
        }
    }
}

fn generate_addr(addr: &[char], pos: usize, mut new_addr: String, adressess: &mut Vec<String>) {
    if pos >= addr.len() {
        adressess.push(new_addr);
        return;
    }
    match addr[pos] {
        'X' => {
            let mut m1 = new_addr.clone();
            let mut m2 = new_addr;
            m1.push('1');
            m2.push('0');
            generate_addr(addr, pos + 1, m1, adressess);
            generate_addr(addr, pos + 1, m2, adressess);
        }
        c => {
            new_addr.push(c);
            generate_addr(addr, pos + 1, new_addr, adressess);
        }
    }
}

fn run2(memory: &mut HashMap<i64, i64>, instructions: &[Instruction]) {
    let mut mask = Vec::new();
    for ins in instructions {
        match ins {
            Instruction::Memory(a, v) => {
                let mut addr = Vec::new();
                for i in (0..36).rev() {
                    let c = match a & 1 << i {
                        0 => '0',
                        _ => '1',
                    };
                    match mask[i] {
                        '0' => addr.push(c),
                        '1' => addr.push('1'),
                        _ => addr.push('X'),
                    }
                }
                let mut addresses = Vec::new();
                generate_addr(&addr, 0, String::new(), &mut addresses);
                for addr in addresses {
                    let a = addr.bytes().fold(0, |acc, v| match v {
                        b'0' => acc << 1,
                        _ => acc << 1 | 1,
                    });
                    memory.insert(a, *v);
                }
            }
            Instruction::Mask(v) => {
                mask = v.clone();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let instructions = parse_input(&input).collect::<Result<Vec<Instruction>, _>>()?;

    let mut memory = HashMap::new();
    run1(&mut memory, &instructions);
    let result1: i64 = memory.values().sum();

    let mut memory = HashMap::new();
    run2(&mut memory, &instructions);
    let result2: i64 = memory.values().sum();

    println!("part1: {}", result1);
    println!("part2: {}", result2);
    Ok(())
}
