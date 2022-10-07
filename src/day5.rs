use crate::common::Day;

pub struct Day5;

fn check_vowels(s: &&String) -> bool {
    const VOWELS: &str = "aeiou";
    s.chars().filter(|&c| VOWELS.contains(c)).count() >= 3
}

fn check_substrings(s: &&String) -> bool {
    const BLACKLIST_STRINGS: [(char, char); 4] = [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')];
    let tail = s.chars().skip(1);
    ! s.chars().zip(tail).any(|x: (char, char)| BLACKLIST_STRINGS.contains(&x))
}

fn check_single_repetition(s: &&String) -> bool {
    let tail = s.chars().skip(1);
    s.chars().zip(tail).any(|(c1, c2)| c1 == c2)
}

fn check_single_repetition_with_skip(s: &&String) -> bool {
    let tailer = s.chars().skip(2);
    s.chars().zip(tailer).any(|(c1, c2)| c1 == c2)
}

fn check_pair_repetition(s: &&String) -> bool {
    use std::collections::HashMap;

    let mut map = HashMap::new();

    let tail = s.chars().skip(1);

    for (i, x) in s.chars().zip(tail).enumerate() {
        if let Some(v) = map.get(&x) {
            if *v < (i - 1) {
                return true;
            }
        } else {
            map.insert(x, i);
        }
    }

    return false;
}

impl Day for Day5 {
    type Input = Vec<String>;
    type Output = usize;

    fn day_number() -> usize {
        5
    }

    fn part1(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(check_vowels)
            .filter(check_substrings)
            .filter(check_single_repetition)
            .count()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input
            .iter()
            .filter(check_single_repetition_with_skip)
            .filter(check_pair_repetition)
            .count()
    }

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|s| String::from(s)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        assert_eq!(Day5::part1(&vec![String::from("ugknbfddgicrmopn")]), 1);
        assert_eq!(Day5::part1(&vec![String::from("aaa")]), 1);
        assert_eq!(Day5::part1(&vec![String::from("jchzalrnumimnmhp")]), 0);
        assert_eq!(Day5::part1(&vec![String::from("haegwjzuvuyypxyu")]), 0);
        assert_eq!(Day5::part1(&vec![String::from("dvszwmarrgswjxmb")]), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(Day5::part2(&vec![String::from("qjhvhtzxzqqjkmpb")]), 1);
        assert_eq!(Day5::part2(&vec![String::from("xxyxx")]), 1);
        assert_eq!(Day5::part2(&vec![String::from("ieodomkazucvgmuy")]), 0);
        assert_eq!(Day5::part2(&vec![String::from("uurcxstgmygtbstg")]), 0);

    }
}