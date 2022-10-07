use crate::common::Day;

pub struct Day10;

fn expand_string(s: &str) -> String {
    let mut out_str = String::new();
    let mut run_len = 1;
    let mut iter = s.chars();
    let mut run_digit = iter.next().expect("Input can not be empty");
    for c in iter {
        if c != run_digit {
            out_str.push_str(&run_len.to_string());
            out_str.push(run_digit);
            run_len = 1;
            run_digit = c;
        } else {
            run_len += 1;
        }
    }
    out_str.push_str(&run_len.to_string());
    out_str.push(run_digit);
    out_str
}

impl Day for Day10 {
    type Input = String;
    type Output = usize;

    fn day_number() -> usize {
        10
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut out = input.to_string();
        for _ in 0..40 {
            out = expand_string(&out);
        }
        out.len()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut out = input.to_string();
        for _ in 0..50 {
            out = expand_string(&out);
        }
        out.len()
    }

    fn parse(input: &str) -> Self::Input {
        input.to_string()
    }
}