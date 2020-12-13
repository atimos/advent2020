use super::Seat;
pub fn run() {
    let mut prev_seats = super::seats();
    loop {
        let seats = next(&prev_seats);
        if prev_seats == seats {
            break;
        }
        prev_seats = seats;
    }
    let occupied = prev_seats
        .iter()
        .flat_map(|column| column.iter())
        .filter(|seat| **seat == Seat::Occupied)
        .count();

    dbg!(occupied);
}

fn next(seats: &[Vec<Seat>]) -> Vec<Vec<Seat>> {
    seats
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(seat_idx, seat)| {
                    match (seat, adjacent_seats_occupied(seats, row_idx, seat_idx)) {
                        (Seat::Vacant, occupied) if occupied == 0 => Seat::Occupied,
                        (Seat::Occupied, occupied) if occupied > 4 => Seat::Vacant,
                        (seat, _) => *seat,
                    }
                })
                .collect()
        })
        .collect()
}

fn adjacent_seats_occupied(
    seats: &[Vec<Seat>],
    current_row: usize,
    current_column: usize,
) -> usize {
    fn walk(
        seats: &[Vec<Seat>],
        row_idx: isize,
        column_idx: isize,
        row_step: isize,
        column_step: isize,
    ) -> Seat {
        if row_idx > -1 && column_idx > -1 {
            if let Some(row) = seats.get(row_idx as usize) {
                if let Some(seat) = row.get(column_idx as usize) {
                    return if *seat == Seat::Floor {
                        walk(
                            seats,
                            row_idx + row_step,
                            column_idx + column_step,
                            row_step,
                            column_step,
                        )
                    } else {
                        *seat
                    };
                }
            }
        }

        Seat::Floor
    }

    [(1, 0), (0, 1), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)]
        .iter()
        .map(|(row_step, column_step)| {
            walk(
                seats,
                current_row as isize + row_step,
                current_column as isize + column_step,
                *row_step,
                *column_step,
            )
        })
        .filter(|seat| seat == &Seat::Occupied)
        .count()
}
