#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(bool_to_option)]
#![feature(never_type)]
#![feature(try_blocks)]
#![feature(array_methods)]
#![feature(or_patterns)]
#![feature(str_split_once)]

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
mod day19;

fn main() {
    // day1::part1::run();
    // day1::part2::run();
    // day2::part1::run();
    // day2::part2::run();
    // day3::part1::run();
    // day3::part2::run();
    // day4::part1::run();
    // day4::part2::run();
    // day5::part1::run();
    // day5::part2::run();
    // day6::part1::run();
    // day6::part2::run();
    // day7::part1::run();
    // day7::part2::run();
    // day8::part1::run();
    // day8::part2::run();
    // day9::part1::run();
    // day9::part2::run();
    // day10::part1::run();
    // day10::part2::run();
    // day11::part1::run();
    // day11::part2::run();
    // day12::part1::run();
    // day12::part2::run();
    // day13::part1::run();
    // day13::part2::run();
    // day14::part1::run();
    // day14::part2::run();
    // day15::part1::run();
    // day15::part2::run();
    // day16::part1::run();
    // day16::part2::run();
    // day17::part1::run();
    // day17::part2::run();
    // day18::part1::run();
    // day18::part2::run();
    day19::part1::run();
    day19::part2::run();
}

#[allow(dead_code)]
fn data(day: &str) -> String {
    use std::fs::read_to_string;
    read_to_string(format!("./data/{}.txt", day)).unwrap()
}

#[allow(dead_code)]
fn split<'a>(data: &'a str, delimiter: &'a str) -> impl Iterator<Item = &'a str> {
    data.trim_end_matches(delimiter).trim().split(delimiter)
}

#[allow(dead_code)]
fn lines(data: &str) -> impl Iterator<Item = &str> {
    data.lines().map(str::trim)
}
