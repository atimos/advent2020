const SIZE: usize = 25;

#[derive(Debug)]
struct Ring {
    size: usize,
    inner: std::collections::VecDeque<isize>,
}

impl Ring {
    fn new(init_data: &[isize]) -> Self {
        let mut inner = std::collections::VecDeque::new();
        inner.extend(init_data.iter());
        Self { size: init_data.len(), inner }
    }

    fn push_all(&mut self, items: &[isize]) -> Result<(), isize> {
        for item in items {
            if self.is_valid(*item) {
                if self.inner.len() == self.size {
                    self.inner.pop_front();
                }
                self.inner.push_back(*item);
            } else {
                return Err(*item);
            }
        }
        Ok(())
    }

    fn is_valid(&self, item: isize) -> bool {
        if self.inner.len() < self.size {
            true
        } else {
            for inner_item in &self.inner {
                if self.inner.contains(&(item - inner_item)) {
                    return true;
                }
            }
            false
        }
    }
}

pub mod part1 {
    use super::Ring;
    use std::str::FromStr;
    pub fn run() {
        if let Ok(list) = crate::lines(&crate::data("day9"))
            .map(isize::from_str)
            .collect::<Result<Vec<_>, _>>()
        {
            let (init, data) = list.split_at(super::SIZE);
            let mut store = Ring::new(init);
            if let Err(item) = store.push_all(data) {
                dbg!(item);
            }
        }
    }
}

pub mod part2 {
    use super::Ring;
    use std::str::FromStr;
    pub fn run() {
        if let Ok(list) = crate::lines(&crate::data("day9"))
            .map(isize::from_str)
            .collect::<Result<Vec<_>, _>>()
        {
            let (init, data) = list.split_at(super::SIZE);
            let mut store = Ring::new(init);
            if let Err(error) = store.push_all(data) {
                let mut start = 0;
                let mut end = 1;

                let items = loop {
                    let items = &list[start..=end];

                    match items.iter().sum::<isize>() {
                        sum if sum == error => {
                            break items;
                        }
                        sum if sum > error => {
                            start += 1;
                            if start == end {
                                end += 1;
                            }
                        }
                        _ => end += 1,
                    }
                };
                let mut items = items.to_owned();
                items.sort_unstable();
                dbg!(items.first().zip(items.last()).map(|(first, last)| first + last));
            }
        }
    }
}
