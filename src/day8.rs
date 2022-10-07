use crate::common::Day;

pub struct Day8;

fn decode_string(s: &str) -> String {
    let mut out_string = String::new();
    let stripped = &s[1..s.len() - 1];
    let mut it = stripped.chars();
    loop {
        match it.next() {
            Some('\\') => match it.next() {
                Some('x') => {
                    it.next();
                    it.next();
                    out_string.push('\'');
                },
                Some(z) => {
                    out_string.push(z);
                },
                None => break
            },
            Some(y) => {
                out_string.push(y);
            }
            None => break,
        }
    }
    out_string
}

fn encode_string(s: &str) -> String {
    let mut out_string = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => {
                out_string.push_str("\\\"");
            },
            '\\' => {
                out_string.push_str("\\\\");
            },
            x => {
                out_string.push(x);
            }
        }
    }
    out_string.push('"');
    out_string
}

impl Day for Day8 {
    type Input = Vec<String>;
    type Output = usize;

    fn day_number() -> usize {
        8
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let code_count: usize = input.iter().map(|x| x.len()).sum();
        let string_count: usize = input
            .iter()
            .map(|x| decode_string(&x))
            .map(|x| x.len())
            .sum();
        code_count - string_count
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let code_count: usize = input.iter().map(|x| x.len()).sum();
        let string_count: usize = input
            .iter()
            .map(|x| encode_string(&x))
            .map(|x| x.len())
            .sum();

        string_count - code_count
    }

    fn parse(input: &str) -> Self::Input {
        input.lines().map(String::from).collect()
    }
}
