type Position = Vec<isize>;
type PositionRef<'a> = &'a [isize];

type World = std::collections::HashMap<Position, bool>;

fn potential_neighbours(current: PositionRef) -> Vec<Position> {
    fn build_neighbours(pos: &[isize], mut path: Vec<isize>) -> Vec<Position> {
        match pos {
            [current, rest @ ..] => {
                let mut left = path.clone();
                left.push(current - 1);
                let mut right = path.clone();
                right.push(current + 1);
                path.push(*current);

                let mut result = Vec::new();
                result.append(&mut build_neighbours(rest, left));
                result.append(&mut build_neighbours(rest, path));
                result.append(&mut build_neighbours(rest, right));

                result
            }
            [] => {
                vec![path]
            }
        }
    }

    build_neighbours(current, Vec::new()).into_iter().filter(|pos| pos != current).collect()
}

fn alive_neighbours(world: &World, current: PositionRef) -> Vec<Position> {
    potential_neighbours(current)
        .into_iter()
        .filter(|pos| *world.get(pos).unwrap_or(&false))
        .collect()
}

fn new_neighbours(world: &World, current: PositionRef) -> Vec<Position> {
    potential_neighbours(current).into_iter().filter(|pos| !world.contains_key(pos)).collect()
}

fn next(world: World) -> World {
    let mut new_world = World::default();

    world.iter().for_each(|(position, alive)| {
        let neighbours = alive_neighbours(&world, position).len();

        new_world.insert(
            position.clone(),
            (*alive && [2, 3].contains(&neighbours)) || (!alive && neighbours == 3),
        );
    });

    world.iter().flat_map(|(position, _)| new_neighbours(&world, position)).for_each(|position| {
        let neighbours = alive_neighbours(&world, &position).len();

        new_world.insert(position, neighbours == 3);
    });

    new_world
}

pub mod part1 {
    use super::{next, World};
    pub fn run() {
        let data = crate::data("day17");
        let mut world: World = crate::lines(&data)
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(y, char)| (vec![x as isize, y as isize, 0], (char == '#')))
            })
            .collect();

        for _ in 0..6 {
            world = next(world);
        }
        dbg!(world.values().filter(|alive| **alive).count());
    }
}

pub mod part2 {
    use super::{next, World};
    pub fn run() {
        let data = crate::data("day17");
        let mut world: World = crate::lines(&data)
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(y, char)| (vec![x as isize, y as isize, 0, 0], (char == '#')))
            })
            .collect();

        for _ in 0..6 {
            world = next(world);
        }
        dbg!(world.values().filter(|alive| **alive).count());
    }
}
