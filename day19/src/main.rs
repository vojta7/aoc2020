use std::{
    collections::{HashMap, VecDeque},
    env::args,
    fs::read_to_string,
};

#[derive(Debug, Clone)]
enum Rule {
    Term(char),
    NonTerm(Vec<usize>),
}

#[derive(Debug, Clone)]
struct Expr {
    rules: Vec<Rule>,
}

fn split_input(input: &str) -> Option<(&str, &str)> {
    let mut split = input.trim().split("\n\n");
    Some((split.next()?, split.next()?))
}

fn split_at_once<'a, 'b>(input: &'a str, sep: &'b str) -> Option<(&'a str, &'a str)> {
    let mut s = input.splitn(2, sep);
    s.next().and_then(|n| s.next().map(|v| (n, v)))
}

fn parse_rules(input: &str) -> HashMap<usize, Expr> {
    input
        .lines()
        .map(|line| split_at_once(line, ": ").unwrap())
        .map(|(n, rule)| (n.parse().unwrap(), rule))
        .map(|(n, rule)| {
            (
                n,
                if rule.contains('"') {
                    let p = rule.find('"').unwrap();
                    Expr {
                        rules: vec![Rule::Term(rule.chars().nth(p + 1).unwrap())],
                    }
                } else if rule.contains('|') {
                    let (r1, r2) = split_at_once(rule, " | ").unwrap();
                    Expr {
                        rules: vec![
                            Rule::NonTerm(
                                r1.split_whitespace().map(|v| v.parse().unwrap()).collect(),
                            ),
                            Rule::NonTerm(
                                r2.split_whitespace().map(|v| v.parse().unwrap()).collect(),
                            ),
                        ],
                    }
                } else {
                    Expr {
                        rules: vec![Rule::NonTerm(
                            rule.split_whitespace()
                                .map(|v| v.parse::<usize>().unwrap())
                                .collect(),
                        )],
                    }
                },
            )
        })
        .collect()
}

fn bin(rules: &HashMap<usize, Expr>) -> HashMap<usize, Expr> {
    let mut to_process: VecDeque<(usize, Expr)> =
        rules.iter().map(|(k, v)| (*k, v.clone())).collect();
    let mut max_rule_id = *rules.iter().map(|(k, _)| k).max().unwrap();
    let mut done = HashMap::new();

    while let Some((n, expr)) = to_process.pop_front() {
        //match rule {
        //    Rule::Term(_) => {
        //        done.insert(n, rule);
        //    }
        //    Rule::Rules(parts) => {
        //        let r = parts
        //            .iter()
        //            .map(|part| {
        //                if part.len() <= 2 {
        //                    part.clone()
        //                } else {
        //                    max_rule_id += 1;
        //                    let new_rule = Rule::Rules(vec![part[1..].to_vec()]);
        //                    to_process.push_back((max_rule_id, new_rule));
        //                    vec![part[0], max_rule_id]
        //                }
        //            })
        //            .collect();
        //        done.insert(n, Rule::Rules(r));
        //    }
        //}
        let rules = expr
            .rules
            .iter()
            .map(|rule| match rule {
                Rule::Term(_) => rule.clone(),
                Rule::NonTerm(part) => {
                    if part.len() <= 2 {
                        rule.clone()
                    } else {
                        max_rule_id += 1;
                        let new_expr = Expr {
                            rules: vec![Rule::NonTerm(part[1..].to_vec())],
                        };
                        to_process.push_back((max_rule_id, new_expr));
                        Rule::NonTerm(vec![part[0], max_rule_id])
                    }
                }
            })
            .collect();
        done.insert(n, Expr { rules });
    }
    done
}

fn unit(rules: &HashMap<usize, Expr>) -> HashMap<usize, Expr> {
    let mut done = HashMap::new();
    let mut to_process: VecDeque<(usize, Expr)> =
        rules.iter().map(|(k, v)| (*k, v.clone())).collect();
    while let Some((n, expr)) = to_process.pop_front() {
        let mut new_parts: Vec<Rule> = Vec::new();
        let mut change = false;
        for idx in 0..expr.rules.len() {
            match &expr.rules[idx] {
                Rule::Term(_) => {
                    new_parts.push(expr.rules[idx].clone());
                }
                Rule::NonTerm(nonterm) => {
                    if nonterm.len() == 1 {
                        let subst = rules.get(&nonterm[0]).unwrap().clone();
                        new_parts.extend(subst.rules);
                        change = true;
                    } else {
                        new_parts.push(expr.rules[idx].clone());
                    }
                }
            }
        }
        if change {
            to_process.push_back((n, Expr { rules: new_parts }));
        } else {
            done.insert(n, expr);
        }
    }
    done
}

#[allow(unused)]
fn print_rules(rules: &HashMap<usize, Expr>) {
    let mut sorted = rules.iter().collect::<Vec<_>>();
    sorted.sort_by(|(k1, _), (k2, _)| k1.cmp(&k2));

    for (n, expr) in sorted {
        print!("{} -> ", n);
        let tmp = expr
            .rules
            .iter()
            .map(|rule| match rule {
                Rule::Term(r) => format!("\"{}\" ", r),
                Rule::NonTerm(r) => r.iter().map(|r| format!("{} ", r)).collect(),
            })
            .collect::<Vec<_>>();
        print!("{}", tmp.join("| "));
        println!();
    }
}

fn ch_normal_form(rules: &HashMap<usize, Expr>) -> HashMap<usize, Expr> {
    // START
    // TERM
    // BIN
    let rules = bin(rules);
    // DEL
    // UNIT
    let rules = unit(&rules);
    rules
}

fn cyk(string: &[char], rules: &HashMap<usize, Expr>) -> bool {
    let n = string.len();
    let mut p = vec![vec![vec![false; rules.len()]; n]; n];
    for (s, c) in string.iter().enumerate() {
        for (&v, expr) in rules {
            for rule in &expr.rules {
                if let Rule::Term(a) = rule {
                    if c == a {
                        p[0][s][v] = true;
                    }
                }
            }
        }
    }

    for l in 1..n {
        for s in 0..n - l {
            for prt in 1..=l {
                for (&a, expr) in rules {
                    for rule in &expr.rules {
                        if let Rule::NonTerm(rule) = rule {
                            let b = rule[0];
                            let c = rule[1];
                            if p[prt - 1][s][b] && p[l - prt][s + prt][c] {
                                p[l][s][a] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    p[n - 1][0][0]
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let (rules, strings) = split_input(&input).unwrap();
    let rules = parse_rules(&rules);
    let rules = ch_normal_form(&rules);
    let strings = strings.lines().collect::<Vec<_>>();

    let mut sum = 0;
    for s in strings {
        let chars = s.chars().collect::<Vec<_>>();
        if cyk(chars.as_slice(), &rules) {
            sum += 1;
        }
    }

    // this solution works for both parts
    // only difference is input file
    println!("part1: {}", sum);

    Ok(())
}
