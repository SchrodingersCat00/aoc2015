mod day1;
mod day2;
mod day3;
mod day5;
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
    let content = fs::read_to_string("input/day5.txt")
        .expect("Error while reading input file");
    run_day::<day5::Day5>(&content).unwrap();
}
