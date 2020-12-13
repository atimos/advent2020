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
                        (Seat::Occupied, occupied) if occupied > 3 => Seat::Vacant,
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
    let (skip_rows, take_rows) = match current_row {
        0 => (0, 2),
        row => (row - 1, 3),
    };
    let (skip_columns, take_columns) = match current_column {
        0 => (0, 2),
        column => (column - 1, 3),
    };

    seats
        .iter()
        .enumerate()
        .skip(skip_rows)
        .take(take_rows)
        .flat_map(|(row_idx, row)| {
            row.iter().enumerate().skip(skip_columns).take(take_columns).filter(
                move |(column_idx, _)| !(row_idx == current_row && *column_idx == current_column),
            )
        })
        .filter(|(_, seat)| seat == &&Seat::Occupied)
        .count()
}
