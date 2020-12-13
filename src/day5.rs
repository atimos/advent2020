enum TopBottom {
    Top,
    Bottom,
}

fn row(input: &str) -> nom::IResult<&str, (Vec<TopBottom>, Vec<TopBottom>), ()> {
    use nom::{
        bytes::complete::take,
        character::complete::newline,
        combinator::map_res,
        sequence::{terminated, tuple},
    };
    terminated(
        tuple((map_res(take(7_usize), top_bottom), map_res(take(3_usize), top_bottom))),
        newline,
    )(input)
}

fn top_bottom(input: &str) -> Result<Vec<TopBottom>, ()> {
    input
        .bytes()
        .map(|byte| match byte {
            b'F' | b'L' => Ok(TopBottom::Bottom),
            b'B' | b'R' => Ok(TopBottom::Top),
            _ => Err(()),
        })
        .collect()
}

fn get_pos(steps: &[TopBottom], range: std::ops::RangeInclusive<u16>) -> u16 {
    let column = steps.iter().fold(range, |pos, step| {
        let split: u16 = (f32::from(pos.end() - pos.start()) / 2.).ceil() as u16;
        match step {
            TopBottom::Bottom => *pos.start()..=pos.end() - split,
            TopBottom::Top => pos.start() + split..=*pos.end(),
        }
    });
    *column.start()
}

pub mod part1 {
    use super::{get_pos, row};
    use nom::combinator::iterator;

    pub fn run() {
        let highest = iterator(crate::data("day5").as_str(), row)
            .map(|(row, column)| (get_pos(&row, 0..=127), get_pos(&column, 0..=7)))
            .map(|(row, column)| row * 8 + column)
            .max();
        dbg!(highest);
    }
}

pub mod part2 {
    use super::{get_pos, row};
    use nom::combinator::iterator;

    pub fn run() {
        let mut list: Vec<u16> = iterator(crate::data("day5").as_str(), row)
            .map(|(row, column)| (get_pos(&row, 0..=127), get_pos(&column, 0..=7)))
            .map(|(row, column)| row * 8 + column)
            .collect();

        list.sort_unstable();
        let mut previous = list.iter().min().unwrap() - 1;
        let missing = list
            .into_iter()
            .find_map(|item| {
                (item - previous != 1).then_some(item).or_else(|| {
                    previous = item;
                    None
                })
            })
            .map(|item| item - 1);
        dbg!(missing);
    }
}
