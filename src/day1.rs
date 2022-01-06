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
        input.chars().scan(0, |floor, c| {
            if *floor == -1 {
                return None;
            }

            match c {
                '(' => {
                    *floor += 1;
                },
                ')' => {

                    *floor -= 1;
                },
                _ => panic!("Something is wrong"),
            };

            Some(*floor)

        }).count() as i32
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
        assert_eq!(Day1::part1(&String::from(")())())")), -3);
        assert_eq!(Day1::part1(&String::from("(())")), 0);
    }

    #[test]
    fn part2() {
        assert_eq!(Day1::part2(&String::from("()())")), 5);
    }
}