fn run(nth: usize) -> Result<usize, ()> {
    let mut numbers = crate::split(&crate::data("day15"), ",")
        .map(str::parse)
        .collect::<Result<Vec<usize>, _>>()
        .map_err(|_| ())?;

    let first_number = numbers.pop().ok_or(())?;

    let mut history: std::collections::HashMap<_, _> =
        numbers.iter().enumerate().map(|(idx, value)| (*value, idx + 1)).collect();

    Ok((numbers.len() + 1..nth).fold(first_number, |number, turn| {
        history.insert(number, turn).map_or(0, |last_turn| turn - last_turn)
    }))
}

pub mod part1 {
    pub fn run() {
        dbg!(super::run(2020).unwrap());
    }
}

pub mod part2 {
    pub fn run() {
        dbg!(super::run(30000000).unwrap());
    }
}
