mod day1;
mod day2;
mod day3;
mod day5;
mod day6;
mod day7;
pub mod common;

use std::fs;

fn run_day<D: common::Day>(content: &str) -> Option<()> {
    let parsed = D::parse(content);
    let part1_result = D::part1(&parsed);
    let part2_result = D::part2(&parsed);
    println!("Part 1: {:?}", part1_result);
    println!("Part 2: {:?}", part2_result);
    Some(())
}

fn main() {
    let content = fs::read_to_string("input/day7.txt")
        .expect("Error while reading input file");
    run_day::<day7::Day7>(&content).unwrap();
}
