use crate::common::Day;
use json::JsonValue;

pub struct Day12;

fn get_value<'a>(kv: (&str, &'a JsonValue)) -> &'a JsonValue {
    kv.1
}

fn has_red(v: &json::object::Object) -> bool {
    v.iter().any(|(_, v)| {
        if let Some(x) = v.as_str() {
            x == "red"
        } else {
            false
        }
    })
}

fn count_numbers(value: &JsonValue, exclude_red: bool) -> i32 {
    match value {
        JsonValue::Number(x) => f64::from(*x) as i32,
        JsonValue::Array(x) => x.iter().map(|x| count_numbers(x, exclude_red)).sum(),
        JsonValue::Object(x) => {
            if exclude_red && has_red(x) {
                0
            } else {
                x.iter()
                    .map(get_value)
                    .map(|x| count_numbers(x, exclude_red))
                    .sum()
            }
        }
        _ => 0,
    }
}

impl Day for Day12 {
    type Input = json::JsonValue;
    type Output = i32;

    fn day_number() -> usize {
        12
    }

    fn part1(input: &Self::Input) -> Self::Output {
        count_numbers(input, false)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        count_numbers(input, true)
    }

    fn parse(input: &str) -> Self::Input {
        json::parse(input).expect("Invalid json input.")
    }
}
