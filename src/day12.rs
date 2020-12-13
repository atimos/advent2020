#[derive(Debug, Clone, Copy)]
enum Op {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Right(i32),
    Left(i32),
    Forward(i32),
}

impl std::str::FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse().map_err(|_| ())?;
        Ok(match &s[0..1] {
            "N" => Self::North(value),
            "E" => Self::East(value),
            "S" => Self::South(value),
            "W" => Self::West(value),
            "R" => Self::Right(value),
            "L" => Self::Left(value),
            "F" => Self::Forward(value),
            _ => return Err(()),
        })
    }
}

fn operations() -> Result<Vec<Op>, ()> {
    crate::lines(&crate::data("day12")).map(str::parse).collect::<Result<Vec<_>, _>>()
}

pub mod part1 {
    use super::Op;

    enum Direction {
        North,
        East,
        South,
        West,
    }

    impl Direction {
        fn right(self, degrees: i32) -> Self {
            let mut new_value = self;
            for _ in 0..degrees / 90 {
                new_value = match new_value {
                    Self::North => Self::East,
                    Self::East => Self::South,
                    Self::South => Self::West,
                    Self::West => Self::North,
                };
            }

            new_value
        }

        fn left(self, degrees: i32) -> Self {
            let mut new_value = self;
            for _ in 0..degrees / 90 {
                new_value = match new_value {
                    Self::North => Self::West,
                    Self::West => Self::South,
                    Self::South => Self::East,
                    Self::East => Self::North,
                }
            }
            new_value
        }
    }

    pub fn run() {
        if let Ok(ops) = super::operations() {
            let end = ops.iter().fold((0, 0, Direction::East), |mut current, op| {
                let mut op = *op;
                if let Op::Forward(value) = op {
                    op = match current.2 {
                        Direction::North => Op::North(value),
                        Direction::West => Op::West(value),
                        Direction::South => Op::South(value),
                        Direction::East => Op::East(value),
                    }
                }
                match op {
                    Op::North(dist) => current.0 += dist,
                    Op::East(dist) => current.1 += dist,
                    Op::South(dist) => current.0 -= dist,
                    Op::West(dist) => current.1 -= dist,
                    Op::Right(degrees) => current.2 = current.2.right(degrees),
                    Op::Left(degrees) => current.2 = current.2.left(degrees),
                    Op::Forward(_) => unreachable!(),
                }
                current
            });
            dbg!(end.0.abs() + end.1.abs());
        }
    }
}

pub mod part2 {
    use super::Op;
    use nalgebra::{Point2, Rotation2};
    pub fn run() {
        if let Ok(ops) = super::operations() {
            let end = ops.iter().fold(((10, 1), (0, 0)), |mut current, op| {
                match op {
                    Op::North(dist) => current.0 .1 += dist,
                    Op::East(dist) => current.0 .0 += dist,
                    Op::South(dist) => current.0 .1 -= dist,
                    Op::West(dist) => current.0 .0 -= dist,
                    Op::Forward(dist) => {
                        current.1 .0 += current.0 .0 * dist;
                        current.1 .1 += current.0 .1 * dist;
                    }
                    Op::Right(degrees) => {
                        current.0 = rotate(
                            match degrees {
                                0 | 360 => std::f64::consts::TAU,
                                90 => std::f64::consts::TAU / 4. * 3.,
                                180 => std::f64::consts::TAU / 2.,
                                270 => std::f64::consts::TAU / 4.,
                                _ => unreachable!(),
                            },
                            current.0,
                        )
                    }
                    Op::Left(degrees) => {
                        current.0 = rotate(
                            match degrees {
                                0 | 360 => std::f64::consts::TAU,
                                90 => std::f64::consts::TAU / 4.,
                                180 => std::f64::consts::TAU / 2.,
                                270 => std::f64::consts::TAU / 4. * 3.,
                                _ => unreachable!(),
                            },
                            current.0,
                        );
                    }
                }
                current
            });
            dbg!(end.1 .0.abs() + end.1 .1.abs());
        }
    }

    fn rotate(rotation: f64, (x, y): (i32, i32)) -> (i32, i32) {
        let rotation = Rotation2::new(rotation);

        let point = rotation * Point2::new(f64::from(x), f64::from(y));
        let mut point = point.iter();
        (point.next().unwrap().round() as i32, point.next().unwrap().round() as i32)
    }
}
