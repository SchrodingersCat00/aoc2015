use crate::common::Day;
use std::cmp;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub type City = String;
pub type Map = HashMap<(City, City), usize>;

pub struct Day9;

fn parse_map_entry(s: &str) -> ((City, City), usize) {
    let mut iter = s.split(' ');
    let orig = iter.next().unwrap();
    iter.next();
    let dest = iter.next().unwrap();
    let words: Vec<&str> = s.split(' ').collect();
    (
        (
            cmp::min(orig, dest).to_string(),
            cmp::max(orig, dest).to_string(),
        ),
        words[4].parse::<usize>().unwrap(),
    )
}

impl Day for Day9 {
    type Input = Map;
    type Output = usize;

    fn day_number() -> usize {
        9
    }

    fn part1(input: &Self::Input) -> Self::Output {
        println!("{:?}", input);
        0
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &str) -> Self::Input {
        Map::from_iter(input.lines().map(parse_map_entry))
    }
}
