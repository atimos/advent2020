pub mod part1 {
    pub fn run() {
        let count = crate::lines(&crate::data("day3"))
            .enumerate()
            .map(|(idx, line)| line.chars().cycle().nth(idx * 3).unwrap())
            .filter(|item| *item == '#')
            .count();
        dbg!(count);
    }
}

pub mod part2 {
    pub fn run() {
        fn calc(row: usize, column: usize) -> usize {
            crate::lines(&crate::data("day3"))
                .enumerate()
                .filter_map(|(idx, lines)| (idx % row == 0).then_some(lines))
                .enumerate()
                .map(|(idx, line)| line.chars().cycle().nth(idx * column).unwrap())
                .filter(|item| *item == '#')
                .count()
        };

        dbg!(calc(1, 1) * calc(1, 3) * calc(1, 5) * calc(1, 7) * calc(2, 1));
    }
}
