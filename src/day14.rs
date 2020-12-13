const WIDTH: usize = 36;

#[derive(Debug)]
enum Op {
    Mask(String),
    Memory(u64, u64),
}

impl std::str::FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..=3] {
            "mask" => Ok(Self::Mask(s[7..].into())),
            "mem[" => {
                let split = s.find(']').ok_or(())?;
                Ok(Self::Memory(
                    s[4..split].parse().map_err(|_| ())?,
                    s[split + 4..].parse().map_err(|_| ())?,
                ))
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Computer {
    mask: String,
    memory: std::collections::HashMap<u64, u64>,
}

impl Computer {
    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

impl Default for Computer {
    fn default() -> Self {
        Self { mask: "X".repeat(WIDTH), memory: std::collections::HashMap::new() }
    }
}

pub mod part1 {
    use super::{Computer, Op, WIDTH};
    pub fn run() {
        if let Ok(ops) =
            crate::lines(&crate::data("day14")).map(str::parse).collect::<Result<Vec<Op>, _>>()
        {
            let mut computer = Computer::default();
            ops.into_iter().for_each(|op| run_op(&mut computer, op));
            dbg!(computer.memory_sum());
        }
    }

    fn run_op(computer: &mut Computer, op: Op) {
        match op {
            Op::Mask(mask) => computer.mask = mask,
            Op::Memory(pos, value) => {
                let binary = &format!("{:#0width$b}", value, width = WIDTH + 2)[2..];
                let value: String = computer
                    .mask
                    .chars()
                    .zip(binary.chars())
                    .map(|(mask, number)| match (mask, number) {
                        ('X', number) => number,
                        (mask, _) => mask,
                    })
                    .collect();
                computer.memory.insert(pos, u64::from_str_radix(&value, 2).unwrap());
            }
        }
    }
}

pub mod part2 {
    use super::{Computer, Op, WIDTH};
    pub fn run() {
        if let Ok(ops) =
            crate::lines(&crate::data("day14")).map(str::parse).collect::<Result<Vec<Op>, _>>()
        {
            let mut computer = Computer::default();
            ops.into_iter().for_each(|op| run_op(&mut computer, op));
            dbg!(computer.memory_sum());
        }
    }

    fn run_op(computer: &mut Computer, op: Op) {
        match op {
            Op::Mask(mask) => computer.mask = mask,
            Op::Memory(pos, value) => {
                let binary = &format!("{:#0width$b}", pos, width = WIDTH + 2)[2..];
                let pos: Vec<_> = computer.mask.chars().zip(binary.chars()).collect();
                for pos in apply_mask(&pos, Vec::new()) {
                    computer.memory.insert(u64::from_str_radix(&pos, 2).unwrap(), value);
                }
            }
        }
    }

    fn apply_mask(data: &[(char, char)], mut path: Vec<char>) -> Vec<String> {
        match data {
            [(mask, location), rest @ ..] => match mask {
                'X' => {
                    let mut result = Vec::new();
                    ['0', '1'].iter().for_each(|next| {
                        let mut path = path.clone();
                        path.push(*next);
                        result.append(&mut apply_mask(rest, path));
                    });
                    result
                }
                '0' => {
                    path.push(*location);
                    apply_mask(rest, path)
                }
                '1' => {
                    path.push(*mask);
                    apply_mask(rest, path)
                }
                _ => unreachable!(),
            },
            _ => vec![path.iter().collect()],
        }
    }
}
