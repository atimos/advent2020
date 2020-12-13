#[derive(Debug, Clone)]
enum Op {
    Noop(isize),
    Acc(isize),
    Jump(isize),
}

impl std::str::FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..=2] {
            "jmp" => Ok(Self::Jump(s[4..].parse().map_err(|_| ())?)),
            "acc" => Ok(Self::Acc(s[4..].parse().map_err(|_| ())?)),
            "nop" => Ok(Self::Noop(s[4..].parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

fn get_ops(data: &str) -> Result<Vec<Op>, ()> {
    crate::lines(data).map(str::parse).collect::<Result<Vec<Op>, _>>()
}

pub mod part1 {
    use super::{get_ops, Op};
    use std::collections::HashSet;
    pub fn run() {
        if let Ok(ops) = get_ops(&crate::data("day8")) {
            dbg!(run_ops(&ops));
        }
    }

    fn run_ops(ops: &[Op]) -> isize {
        let mut prev_pos: HashSet<usize> = HashSet::new();
        let mut pos = 0;
        let mut acc = 0;
        loop {
            if prev_pos.contains(&pos) {
                break;
            }
            prev_pos.insert(pos);
            match &ops.get(pos) {
                Some(Op::Acc(val)) => acc += val,
                Some(Op::Jump(val)) => {
                    pos = (pos as isize + val) as usize;
                    continue;
                }
                Some(Op::Noop(_)) => {}
                None => break,
            }

            pos += 1;
        }
        acc
    }
}

pub mod part2 {
    use super::{get_ops, Op};
    use std::collections::HashSet;
    pub fn run() {
        if let Ok(ops) = get_ops(&crate::data("day8")) {
            if let Ok(acc) = run_op(&ops, 0, 0, HashSet::new()) {
                dbg!(acc);
            }
        }
    }

    fn run_op(
        ops: &[Op],
        pos: usize,
        acc: isize,
        mut prev_pos: HashSet<usize>,
    ) -> Result<isize, ()> {
        if prev_pos.contains(&pos) {
            return Err(());
        }
        prev_pos.insert(pos);
        match &ops.get(pos) {
            Some(Op::Acc(val)) => run_op(ops, pos + 1, acc + val, prev_pos),
            Some(Op::Jump(val)) => {
                match run_op(ops, (pos as isize + val) as usize, acc, prev_pos.clone()) {
                    Ok(acc) => Ok(acc),
                    Err(_) => run_op(ops, pos + 1, acc, prev_pos),
                }
            }
            Some(Op::Noop(val)) => match run_op(ops, pos + 1, acc, prev_pos.clone()) {
                Ok(acc) => Ok(acc),
                Err(_) => run_op(ops, (pos as isize + val) as usize, acc, prev_pos),
            },
            None => Ok(acc),
        }
    }
}
