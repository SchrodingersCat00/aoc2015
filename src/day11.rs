use crate::common::Day;
use std::str;

pub struct Day11;

fn next_possibility(pwd: &mut [u8]) {
    let mut i: usize = pwd.len() - 1;
    while i > 0 && pwd[i] == b'z' {
        pwd[i] = b'a';
        i -= 1;
    }
    pwd[i] = pwd[i] + 1;
}

fn is_valid(pwd: &[u8]) -> bool {
    let mut run_found = false;
    for i in 0..pwd.len()-2 {
        if pwd[i] == pwd[i+1] - 1 && pwd[i] == pwd[i+2] - 2 {
            run_found = true;
        }
    }

    if ! run_found {
        return false;
    }
    if pwd.contains(&b'i') || pwd.contains(&b'o') || pwd.contains(&b'l') {
        return false;
    }
    let mut i: usize = 0;
    let mut pair_count = 0;
    while i < pwd.len()-1 {
        if pwd[i] == pwd[i+1] {
            pair_count += 1;
            i += 2;
        } else {
            i += 1;
        }
    }
    if pair_count != 2 {
        return false;
    }
    
    return true;
}

impl Day for Day11 {
    type Input = String;
    type Output = String;

    fn day_number() -> usize {
        11
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let mut pwd = input.as_bytes().to_vec();
        next_possibility(&mut pwd);
        while ! is_valid(&pwd) {
            next_possibility(&mut pwd);
        }
        std::str::from_utf8(&pwd).unwrap().to_string()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut pwd = input.as_bytes().to_vec();
        for _ in 0..2 {
            next_possibility(&mut pwd);
            while ! is_valid(&pwd) {
                next_possibility(&mut pwd);
            }
        }
        std::str::from_utf8(&pwd).unwrap().to_string()
    }

    fn parse(input: &str) -> Self::Input {
        input.to_string()
    }
}