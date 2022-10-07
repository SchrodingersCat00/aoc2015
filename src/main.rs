pub mod common;
mod day1;
mod day2;
mod day3;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

use std::fs;

use common::Day;

fn run_day<D: common::Day>(content: &str) {
    let parsed = D::parse(content);
    let part1_result = D::part1(&parsed);
    let part2_result = D::part2(&parsed);
    println!("Part 1: {:?}", part1_result);
    println!("Part 2: {:?}", part2_result);
}

fn main() {
    let content = fs::read_to_string(format!("input/day{}.txt", day12::Day12::day_number()))
        .expect("Error while reading input file");
    run_day::<day12::Day12>(&content);
}
