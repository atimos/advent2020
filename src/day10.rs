fn rows() -> Result<Vec<u8>, ()> {
    if let Ok(mut rows) =
        crate::lines(&crate::data("day10")).map(str::parse).collect::<Result<Vec<_>, _>>()
    {
        rows.push(0);
        rows.sort_unstable();
        if let Some(last) = rows.last().cloned() {
            rows.push(last + 3);
        }
        Ok(rows)
    } else {
        Err(())
    }
}

pub mod part1 {
    pub fn run() {
        if let Ok(rows) = super::rows() {
            let mut one = 0;
            let mut three = 0;

            for window in rows.windows(2) {
                if let [first, second] = window {
                    match second - first {
                        1 => one += 1,
                        3 => three += 1,
                        _ => {}
                    }
                }
            }

            dbg!(one * three);
        }
    }
}

pub mod part2 {
    use std::collections::HashMap;

    pub fn run() {
        if let Ok(rows) = super::rows() {
            dbg!(run_cached(&rows, &mut HashMap::new()));
        }
    }

    fn run_cached(adapters: &[u8], cache: &mut HashMap<u8, u64>) -> u64 {
        match adapters {
            [_] => 1,
            [current, rest @ ..] => {
                let mut found = 0;
                for (idx, next) in rest.iter().enumerate() {
                    if next - current > 3 {
                        break;
                    }
                    let result =
                        cache.get(next).copied().unwrap_or_else(|| run_cached(&rest[idx..], cache));
                    cache.insert(*next, result);
                    found += result;
                }
                found
            }
            [] => unreachable!(),
        }
    }
}
