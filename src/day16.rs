use std::ops::RangeInclusive;

type Ranges<'a> = Vec<(&'a str, Vec<RangeInclusive<usize>>)>;
type Tickets = Vec<Vec<usize>>;

fn data(data: &str) -> Result<(Ranges, Tickets), ()> {
    if let [ranges, my_ticket, other_tickets] =
        crate::split(data, "\n\n").collect::<Vec<&str>>().as_slice()
    {
        let mut tickets = parse_tickets(my_ticket)?;
        tickets.append(&mut parse_tickets(other_tickets)?);
        Ok((parse_ranges(ranges)?, tickets))
    } else {
        Err(())
    }
}

fn parse_tickets(data: &str) -> Result<Tickets, ()> {
    use std::num::ParseIntError;
    data.split('\n')
        .skip(1)
        .map(|line| line.split(',').map(str::parse).collect())
        .collect::<Result<_, ParseIntError>>()
        .map_err(|_| ())
}

fn parse_ranges(data: &str) -> Result<Ranges, ()> {
    data.split('\n')
        .map(|line| {
            let mut parts = line.split(": ");
            Ok((
                parts.next().ok_or(())?,
                parts
                    .next()
                    .map(|ranges| {
                        Ok(ranges
                            .split(" or ")
                            .map(|range| {
                                let mut numbers = range.split('-');
                                Ok(RangeInclusive::new(
                                    numbers
                                        .next()
                                        .ok_or(())
                                        .and_then(|number| str::parse(number).map_err(|_| ()))?,
                                    numbers
                                        .next()
                                        .ok_or(())
                                        .and_then(|number| str::parse(number).map_err(|_| ()))?,
                                ))
                            })
                            .collect::<Result<_, ()>>()?)
                    })
                    .ok_or(())??,
            ))
        })
        .collect()
}

pub mod part1 {
    use super::data;
    pub fn run() {
        if let Ok((ranges, tickets)) = data(&crate::data("day16")) {
            let ranges: Vec<_> = ranges.iter().flat_map(|(_, ranges)| ranges.iter()).collect();

            let result = tickets
                .iter()
                .flat_map(|ticket| {
                    ticket
                        .iter()
                        .filter(|number| !ranges.iter().any(|range| range.contains(number)))
                })
                .sum::<usize>();
            dbg!(result);
        }
    }
}

pub mod part2 {
    use super::data;
    use itertools::Itertools;
    pub fn run() {
        if let Ok((ranges, tickets)) = data(&crate::data("day16")) {
            let range_list: Vec<_> = ranges.iter().flat_map(|(_, ranges)| ranges.iter()).collect();

            let my_ticket = tickets.get(0).unwrap();

            let ticket_numbers = tickets
                .iter()
                .filter(|ticket| {
                    ticket
                        .iter()
                        .all(|number| range_list.iter().any(|range| range.contains(number)))
                })
                .fold(vec![Vec::new(); ranges.len()], |mut map, ticket| {
                    ticket.iter().enumerate().for_each(|(idx, value)| {
                        map[idx].push(value);
                    });
                    map
                });

            let mut found = Vec::new();
            let result: usize = ranges
                .into_iter()
                .map(|(name, ranges)| {
                    (
                        name,
                        ticket_numbers
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, numbers)| {
                                numbers
                                    .iter()
                                    .all(|number| ranges.iter().any(|range| range.contains(number)))
                                    .then_some(idx)
                            })
                            .collect(),
                    )
                })
                .sorted_by(|(_, a): &(_, Vec<usize>), (_, b): &(_, _)| a.len().cmp(&b.len()))
                .map(|(name, idx_list)| {
                    idx_list.iter().find(|idx| !found.contains(*idx)).map(|idx| {
                        found.push(*idx);
                        (name, *idx)
                    })
                })
                .filter_map(|field| {
                    field.and_then(|field| {
                        field.0.starts_with("departure").then_some(my_ticket[field.1])
                    })
                })
                .product();

            dbg!(result);
        }
    }
}
