#[derive(Clone, Debug)]
enum Expression {
    Num(u64),
    Add,
    Mul,
    Group(Vec<Expression>),
}

fn parse(data: &str, mut exprs: Vec<Expression>) -> (&str, Vec<Expression>) {
    let (rest, expr) = match data.chars().next() {
        Some(current) if current.is_digit(10) => {
            let (number, rest) = data.find(|char: char| !char.is_numeric()).map_or_else(
                || (current.to_string(), ""),
                |split_at| {
                    let (number, rest) = data.split_at(split_at);
                    (number.to_string(), rest)
                },
            );
            (rest, Expression::Num(number.parse::<u64>().unwrap()))
        }
        Some('*') => (&data[1..], Expression::Mul),
        Some('+') => (&data[1..], Expression::Add),
        Some('(') => {
            let (rest, exprs) = parse(&data[1..], Vec::new());
            (rest, Expression::Group(exprs))
        }
        Some(')') => return (&data[1..], exprs),
        None => return ("", exprs),
        Some(_) => unreachable!(),
    };

    exprs.push(expr);
    parse(rest, exprs)
}

pub mod part1 {
    use super::{parse, Expression};

    pub fn run() {
        let data = crate::data("day18");

        let result: u64 = crate::lines(&data)
            .map(|line| {
                let line: String = line.chars().filter(|char| *char != ' ').collect();
                calculate(0, &parse(&line, Vec::new()).1)
            })
            .sum();

        dbg!(result);
    }

    fn calculate(number: u64, expr: &[Expression]) -> u64 {
        match expr {
            [Expression::Num(number), rest @ ..] => calculate(*number, rest),
            [Expression::Add, Expression::Num(second), rest @ ..] => {
                calculate(number + second, rest)
            }
            [Expression::Add, Expression::Group(second), rest @ ..] => {
                calculate(number + calculate(0, second), rest)
            }
            [Expression::Mul, Expression::Num(second), rest @ ..] => {
                calculate(number * second, rest)
            }
            [Expression::Mul, Expression::Group(second), rest @ ..] => {
                calculate(number * calculate(0, second), rest)
            }
            [Expression::Group(group), rest @ ..] => calculate(calculate(0, group), rest),
            _ => number,
        }
    }
}

pub mod part2 {
    use super::{parse, Expression};

    pub fn run() {
        let data = crate::data("day18");

        let result: u64 = crate::lines(&data)
            .map(|line| {
                let line: String = line.chars().filter(|char| *char != ' ').collect();
                calculate(0, &parse(&line, Vec::new()).1)
            })
            .sum();

        dbg!(result);
    }

    fn calculate(number: u64, expr: &[Expression]) -> u64 {
        match expr {
            [Expression::Num(number), rest @ ..] => calculate(*number, rest),
            [Expression::Add, Expression::Num(second), rest @ ..] => {
                calculate(number + second, rest)
            }
            [Expression::Add, Expression::Group(second), rest @ ..] => {
                calculate(number + calculate(0, second), rest)
            }
            [Expression::Mul, Expression::Num(second), rest @ ..] => {
                number * calculate(*second, rest)
            }
            [Expression::Mul, Expression::Group(second), rest @ ..] => {
                number * calculate(calculate(0, second), rest)
            }
            [Expression::Group(group), rest @ ..] => calculate(calculate(0, group), rest),
            _ => number,
        }
    }
}
