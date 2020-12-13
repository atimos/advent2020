pub mod part1 {
    use nom::{
        bytes::complete::{is_a, is_not, tag},
        character::complete::{alpha1, digit1, newline, space1},
        combinator::{map_parser, map_res},
        sequence::{preceded, separated_pair, terminated, tuple},
    };
    use std::{num::ParseIntError, ops::RangeInclusive};

    pub fn run() {
        let data = &crate::data("day2");
        let data: Result<_, nom::Err<()>> = nom::multi::separated_list1(
            newline,
            tuple((
                map_res(
                    map_parser(is_not(" "), separated_pair(digit1, tag("-"), digit1)),
                    |(start, end): (&str, &str)| -> Result<RangeInclusive<usize>, ParseIntError> {
                        Ok(start.parse()?..=end.parse()?)
                    },
                ),
                terminated(preceded(space1, alpha1), is_a(": ")),
                is_not("\n"),
            )),
        )(data);

        if let Ok(lines) = data {
            let result = lines
                .1
                .iter()
                .map(|(range, control_char, password)| {
                    (
                        range,
                        password
                            .chars()
                            .filter(move |char| &char.to_string() == control_char)
                            .count(),
                    )
                })
                .filter(|(range, count)| range.contains(count))
                .count();
            println!("{}", result);
        }
    }
}

pub mod part2 {
    use nom::{
        bytes::complete::{is_a, is_not, tag},
        character::complete::{alpha1, digit1, newline, space1},
        combinator::{map_parser, map_res},
        sequence::{preceded, separated_pair, terminated, tuple},
    };
    use std::num::ParseIntError;

    pub fn run() {
        let data = &crate::data("day2");
        let data: Result<_, nom::Err<()>> = nom::multi::separated_list1(
            newline,
            tuple((
                map_res(
                    map_parser(is_not(" "), separated_pair(digit1, tag("-"), digit1)),
                    |(start, end): (&str, &str)| -> Result<(usize, usize), ParseIntError> {
                        Ok((start.parse()?, end.parse()?))
                    },
                ),
                terminated(preceded(space1, alpha1), is_a(": ")),
                is_not("\n"),
            )),
        )(data);

        if let Ok(lines) = data {
            let result = lines
                .1
                .iter()
                .map(|((first, second), control_char, password)| {
                    (
                        &password.chars().nth(first - 1).unwrap().to_string() == control_char,
                        &password.chars().nth(second - 1).unwrap().to_string() == control_char,
                    )
                })
                .filter(|(first, second)| first ^ second && (*first || *second))
                .count();
            println!("{}", result);
        }
    }
}
