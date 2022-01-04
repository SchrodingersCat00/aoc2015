use crate::common::Day;

pub struct Day1;
impl Day for Day1 {
    type Input = String;
    type Output = i32;

    fn part1(input: &Self::Input) -> Self::Output {
        input.chars().fold(0, |acc, c| {
            match c {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => panic!("Something is wrong")
            }
        })
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut pos = 1;
        let mut floor: i32 = 0;
        for c in input.chars() {
            if c == '(' {
                floor += 1;
            } else if c == ')' {
                floor -= 1;
            }

            if floor == -1 {
                break;
            }
            pos += 1;
        }

        pos
    }

    fn parse(input: &str) -> Self::Input {
        String::from(input)
    }
}