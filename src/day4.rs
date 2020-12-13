fn kvalue<'a>(name: &'a str) -> impl Fn(&'a str) -> nom::IResult<&'a str, &'a str, ()> {
    use nom::{
        bytes::complete::{is_not, tag},
        character::complete::multispace0,
        sequence::{pair, preceded, terminated},
    };
    move |input| terminated(preceded(pair(tag(name), tag(":")), is_not("\n ")), multispace0)(input)
}

pub mod part1 {
    use super::kvalue;
    use nom::{branch::permutation, combinator::opt};

    pub fn run() {
        let count = crate::split(&crate::data("day4"), "\n\n")
            .map(|data| -> Result<_, nom::Err<()>> {
                permutation((
                    kvalue("byr"),
                    kvalue("iyr"),
                    kvalue("eyr"),
                    kvalue("hgt"),
                    kvalue("hcl"),
                    kvalue("ecl"),
                    kvalue("pid"),
                    opt(kvalue("cid")),
                ))(data)
            })
            .filter_map(Result::ok)
            .count();
        dbg!(count);
    }
}

pub mod part2 {
    use super::kvalue;
    use nom::{
        branch::{alt, permutation},
        bytes::complete::tag,
        character::complete::{alphanumeric1, digit1, hex_digit1},
        combinator::{map_opt, map_parser, opt},
        sequence::{pair, preceded},
        IResult,
    };
    use std::ops::RangeInclusive;

    pub fn run() {
        let count = crate::split(&crate::data("day4"), "\n\n")
            .map(|data| -> Result<_, nom::Err<()>> {
                permutation((
                    map_parser(kvalue("byr"), range(1920..=2002)),
                    map_parser(kvalue("iyr"), range(2010..=2020)),
                    map_parser(kvalue("eyr"), range(2020..=2030)),
                    map_parser(kvalue("hgt"), height),
                    map_parser(kvalue("hcl"), haircolor),
                    map_parser(kvalue("ecl"), eyecolor),
                    map_parser(kvalue("pid"), pid),
                    opt(kvalue("cid")),
                ))(data)
            })
            .filter_map(Result::ok)
            .count();
        dbg!(count);
    }

    fn range<'a>(range: RangeInclusive<u32>) -> impl Fn(&'a str) -> IResult<&'a str, u32, ()> {
        move |input| {
            map_opt(alphanumeric1, |data: &str| {
                data.parse().ok().and_then(|date| range.contains(&date).then_some(date))
            })(input)
        }
    }

    fn height<'a>(input: &'a str) -> IResult<&'a str, (u32, &'a str), ()> {
        map_opt(
            pair(digit1, alt((tag("cm"), tag("in")))),
            |(height, kind): (&str, &str)| -> Option<(u32, &'a str)> {
                let height = height.parse().ok()?;
                match kind {
                    "cm" if (150..=193).contains(&height) => Some((height, kind)),
                    "in" if (59..=76).contains(&height) => Some((height, kind)),
                    _ => None,
                }
            },
        )(input)
    }

    fn haircolor(input: &str) -> IResult<&str, &str, ()> {
        preceded(tag("#"), map_opt(hex_digit1, |data: &str| (data.len() == 6).then_some(data)))(
            input,
        )
    }

    fn eyecolor(input: &str) -> IResult<&str, &str, ()> {
        alt((tag("amb"), tag("blu"), tag("brn"), tag("gry"), tag("grn"), tag("hzl"), tag("oth")))(
            input,
        )
    }

    fn pid(input: &str) -> IResult<&str, &str, ()> {
        map_opt(digit1, |data: &str| (data.len() == 9).then_some(data))(input)
    }
}
