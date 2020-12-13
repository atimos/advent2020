fn data() -> (usize, Vec<Option<usize>>) {
    let data = &crate::data("day13");
    let mut lines = crate::lines(data);
    (
        lines.next().map(str::parse).map(Result::unwrap).unwrap(),
        lines.next().unwrap().split(',').map(str::parse).map(Result::ok).collect(),
    )
}

pub mod part1 {
    use super::data;
    pub fn run() {
        let (time, buses) = data();
        if let Some((time, bus)) = buses
            .iter()
            .filter_map(|bus| {
                bus.map(|bus| {
                    for i in 0.. {
                        let bus_time = bus * i;
                        if bus_time > time {
                            return (bus_time - time, bus);
                        }
                    }
                    unreachable!();
                })
            })
            .min_by(|(a, _), (b, _)| a.cmp(b))
        {
            dbg!(time * bus);
        }
    }
}

pub mod part2 {
    use super::data;

    pub fn run() {
        slow();
        // fast();
    }

    fn slow() {
        let (_, buses) = data();

        let mut buses: Vec<_> = buses
            .iter()
            .enumerate()
            .filter_map(|(idx, bus)| bus.map(|bus| (idx as i64, bus as i64)))
            .collect();

        buses.sort_by(|(_, a), (_, b)| b.cmp(a));

        let last = buses.remove(0);

        let buses: Vec<_> = buses.iter().map(|(idx, bus)| (idx - last.0, *bus)).collect();

        let mut t = 0;

        loop {
            t += last.1;
            if buses.iter().all(|(idx, bus)| (t as i64 + idx) % *bus == 0) {
                break;
            }
        }
        dbg!(t - last.0);
    }

    pub fn fast() {
        let (_, buses) = data();

        let mut buses: Vec<_> =
            buses.iter().enumerate().filter_map(|(idx, bus)| bus.map(|bus| (idx, bus))).collect();

        let mut t = 0;
        let mut steps = buses.remove(0).1;

        for (idx, bus) in buses {
            steps = loop {
                t += steps;
                if (t + idx) % bus == 0 {
                    break steps * bus;
                }
            };
        }
        dbg!(t);
    }
}
