pub mod part1 {
    use itertools::Itertools;
    pub fn run() {
        let count: usize = crate::split(&crate::data("day6"), "\n\n")
            .map(|data| data.split('\n').flat_map(str::chars).sorted().dedup().count())
            .sum();
        dbg!(count);
    }
}

pub mod part2 {
    use itertools::Itertools;
    pub fn run() {
        let count: usize = crate::split(&crate::data("day6"), "\n\n")
            .map(|data| {
                let person_count = data.split('\n').count();
                data.split('\n')
                    .flat_map(str::chars)
                    .sorted()
                    .dedup_with_count()
                    .filter(|(count, _)| *count == person_count)
                    .count()
            })
            .sum();
        dbg!(count);
    }
}
