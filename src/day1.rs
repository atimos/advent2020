pub mod part1 {
    pub fn run() {
        if let Ok(items) = crate::lines(&crate::data("day1"))
            .filter_map(|line| (!line.is_empty()).then_some(line.parse::<u32>()))
            .collect::<Result<Vec<_>, _>>()
        {
            println!(
                "{:?}",
                items
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, first)| items
                        .iter()
                        .skip(idx + 1)
                        .map(move |second| (first, second)))
                    .skip_while(|(first, second)| *first + *second != 2020)
                    .map(|(first, second)| *first * *second)
                    .next()
            );
        }
    }
}

pub mod part2 {
    pub fn run() {
        if let Ok(items) = crate::lines(&crate::data("day1"))
            .filter_map(|line| (!line.is_empty()).then_some(str::parse::<u32>(line)))
            .collect::<Result<Vec<_>, _>>()
        {
            println!(
                "{:?}",
                items
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, first)| items
                        .iter()
                        .skip(idx + 1)
                        .map(move |second| (idx, first, second)))
                    .flat_map(|(idx, first, second)| items
                        .iter()
                        .skip(idx + 1)
                        .map(move |third| (first, second, third)))
                    .skip_while(|(first, second, third)| *first + *second + *third != 2020)
                    .map(|(first, second, third)| *first * *second * *third)
                    .next()
            );
        }
    }
}
