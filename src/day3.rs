use crate::common::Day;
use std::collections::hash_set::HashSet;

type Pos = (i32, i32);

pub struct Day3;

impl Day3 {
    fn folding_function(acc: (Pos, HashSet<Pos>), c: char) -> (Pos, HashSet<Pos>) {
        let (row, col) = acc.0;
        let mut seen_positions = acc.1;

        let pn = match c {
            '^' => (row - 1, col),
            '>' => (row, col + 1),
            'v' => (row + 1, col),
            '<' => (row, col - 1),
            _ => panic!("Unknown character '{}'", c),
        };

        seen_positions.insert(pn);

        (pn, seen_positions)
    }
}

impl Day for Day3 {
    type Input = String;
    type Output = u32;

    fn part1(input: &Self::Input) -> Self::Output {
        let seen_positions = {
            let mut inner = HashSet::new();
            inner.insert((0, 0));
            inner
        };

        input.chars().fold(((0, 0), seen_positions), Day3::folding_function).1.len() as u32
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut seen_positions = HashSet::new();
        seen_positions.insert((0, 0));
        seen_positions = input
            .chars()
            .step_by(2)
            .fold(((0, 0), seen_positions), Day3::folding_function).1;

        input
            .chars()
            .skip(1)
            .step_by(2)
            .fold(((0, 0), seen_positions), Day3::folding_function).1.len() as u32
    }

    fn parse(input: &str) -> Self::Input {
        String::from(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(Day3::part1(&String::from(">")), 2);
        assert_eq!(Day3::part1(&String::from("^>v<")), 4);
        assert_eq!(Day3::part1(&String::from("^v^v^v^v^v")), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(Day3::part2(&String::from("^v")), 3);
        assert_eq!(Day3::part2(&String::from("^>v<")), 3);
        assert_eq!(Day3::part2(&String::from("^v^v^v^v^v")), 11);
    }
}