use std::str::FromStr;
use std::fmt::Debug;

pub trait Day {
    type Input: FromStr;
    type Output: Debug;
    fn parse(input: &str) -> Self::Input;
    fn part1(input: &Self::Input) -> Self::Output;
    fn part2(input: &Self::Input) -> Self::Output;
}