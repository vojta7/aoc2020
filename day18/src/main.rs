use std::{env::args, fs::read_to_string};

#[derive(Debug, Clone)]
enum Expr {
    Term(i64),
    Adition(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Lex {
    LB,
    RB,
    Num(i64),
    Plus,
    Mult,
    Eof,
}

#[derive(Debug)]
enum ParsingError {
    InvalidPrimaryExpression(Lex),
    ExpectedBracket(Lex),
}

type ParsingResult<'a> = Result<(Expr, &'a [Lex]), ParsingError>;

fn lex(input: &str) -> Vec<Lex> {
    input
        .trim()
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '(' => Lex::LB,
            ')' => Lex::RB,
            '+' => Lex::Plus,
            '*' => Lex::Mult,
            d if d.is_digit(10) => Lex::Num(d.to_digit(10).unwrap() as i64),
            c => panic!("Unexpected char {}", c),
        })
        .chain(Some(Lex::Eof).into_iter())
        .collect()
}

fn parse<'a>(rest: &'a [Lex], op_precedence: &'static dyn Fn(&Lex) -> i8) -> ParsingResult<'a> {
    let (lhs, rest) = parse_primary(rest, op_precedence)?;
    parse_binary_ops_rhs(rest, 0, lhs, op_precedence)
}

fn parse_primary<'a>(
    rest: &'a [Lex],
    op_precedence: &'static dyn Fn(&Lex) -> i8,
) -> ParsingResult<'a> {
    match rest[0] {
        Lex::Num(n) => Ok((Expr::Term(n), &rest[1..])),
        Lex::LB => parse_paren(&rest[1..], op_precedence),
        l => Err(ParsingError::InvalidPrimaryExpression(l)),
    }
}

fn parse_paren<'a>(
    rest: &'a [Lex],
    op_precedence: &'static dyn Fn(&Lex) -> i8,
) -> ParsingResult<'a> {
    let (expr, rest) = parse(rest, op_precedence)?;
    match rest[0] {
        Lex::RB => Ok((expr, &rest[1..])),
        l => Err(ParsingError::ExpectedBracket(l)),
    }
}

fn parse_binary_ops_rhs<'a>(
    mut rest: &'a [Lex],
    expr_precedence: i8,
    mut lhs: Expr,
    op_precedence: &'static dyn Fn(&Lex) -> i8,
) -> ParsingResult<'a> {
    loop {
        let token = rest[0];
        let tok_precedence = op_precedence(&token);
        if tok_precedence < expr_precedence {
            return Ok((lhs, rest));
        } else {
            let (mut right_expr, new_rest) = parse_primary(&rest[1..], op_precedence)?;
            rest = new_rest;
            if tok_precedence < op_precedence(&rest[0]) {
                let (new_rhs, new_rest) =
                    parse_binary_ops_rhs(rest, expr_precedence + 1, right_expr, op_precedence)?;
                rest = new_rest;
                right_expr = new_rhs;
            }
            lhs = binary_op_expr(&token, lhs, right_expr)
        }
    }
}

fn binary_op_expr(binary_op: &Lex, lhs: Expr, rhs: Expr) -> Expr {
    match *binary_op {
        Lex::Plus => Expr::Adition(Box::new(lhs), Box::new(rhs)),
        Lex::Mult => Expr::Mult(Box::new(lhs), Box::new(rhs)),
        _ => unreachable!(),
    }
}

fn op_precedence_part1(tok: &Lex) -> i8 {
    match *tok {
        Lex::Plus => 10,
        Lex::Mult => 10,
        Lex::Num(_) => -1,
        Lex::LB => -1,
        Lex::RB => -1,
        Lex::Eof => -1,
    }
}

fn op_precedence_part2(tok: &Lex) -> i8 {
    match *tok {
        Lex::Plus => 20,
        Lex::Mult => 10,
        Lex::Num(_) => -1,
        Lex::LB => -1,
        Lex::RB => -1,
        Lex::Eof => -1,
    }
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Term(n) => *n,
        Expr::Adition(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Mult(lhs, rhs) => eval(lhs) * eval(rhs),
    }
}

#[allow(unused)]
fn print(expr: &Expr) {
    match expr {
        Expr::Term(n) => print!("{}", n),
        Expr::Adition(lhs, rhs) => {
            print!("(");
            print(lhs);
            print!(" + ");
            print(rhs);
            print!(")");
        }
        Expr::Mult(lhs, rhs) => {
            print!("(");
            print(lhs);
            print!(" * ");
            print(rhs);
            print!(")");
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;

    let sum1: i64 = input
        .lines()
        .map(lex)
        .map(|lexems| parse(&lexems, &op_precedence_part1).map(|expr| eval(&expr.0)))
        .sum::<Result<i64, ParsingError>>()
        .unwrap();
    println!("part1: {}", sum1);

    let sum2: i64 = input
        .lines()
        .map(lex)
        .map(|lexems| parse(&lexems, &op_precedence_part2).map(|expr| eval(&expr.0)))
        .sum::<Result<i64, ParsingError>>()
        .unwrap();
    println!("part2: {}", sum2);
    Ok(())
}
