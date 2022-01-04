use crate::common::Day;
use std::cmp;

pub struct Present {
    width: u32,
    height: u32,
    length: u32,
}

pub struct Day2;
impl Day for Day2 {
    type Input = Vec::<Present>;
    type Output = u32;

    fn part1(input: &Self::Input) -> Self::Output {
        input.iter().fold(0, |acc, present| {
            let s1 = present.width*present.length;
            let s2 = present.width*present.height;
            let s3= present.height*present.length;

            acc + 2*(s1 + s2 + s3) + cmp::min(s1, cmp::min(s2, s3))
        })
    }

    fn part2(input: &Self::Input) -> Self::Output {
        input.iter().fold(0, |acc, present| {
            let s1 = present.width;
            let s2 = present.height;
            let s3= present.length;

            let smallest = cmp::min(s1, cmp::min(s2, s3));
            let second_smallest = cmp::max(cmp::min(s1, s2), cmp::min(cmp::max(s1, s2), s3));
            acc + s1*s2*s3 + 2*smallest + 2*second_smallest
        })
    }

    fn parse(input: &str) -> Self::Input {
        input.lines().map(|line| {
            let mut tokens = line.split("x");
            let width = tokens.next().unwrap().parse::<u32>().unwrap();
            let height = tokens.next().unwrap().parse::<u32>().unwrap();
            let length = tokens.next().unwrap().parse::<u32>().unwrap();

            Present {
                width,
                height,
                length,
            }
        }).collect()
    }
}