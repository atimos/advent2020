pub mod part1;
pub mod part2;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Seat {
    Vacant,
    Occupied,
    Floor,
}

impl Seat {
    fn from_byte(data: u8) -> Self {
        match data {
            b'L' => Self::Vacant,
            b'#' => Self::Occupied,
            b'.' => Self::Floor,
            _ => unreachable!(),
        }
    }
}

fn seats() -> Vec<Vec<Seat>> {
    crate::lines(&crate::data("day11"))
        .map(|row| row.bytes().map(Seat::from_byte).collect())
        .collect()
}
