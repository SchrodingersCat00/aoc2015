use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Day;

pub struct Day13;

type Graph<'a> = HashMap<(&'a str, &'a str), i32>;
type Seating<'a> = Vec<&'a str>;

fn parse_graph_entry(s: &str) -> ((&str, &str), i32) {
    let mut words = s.split(' ');
    let p1 = words.next().unwrap();
    words.next();
    let sign = if words.next().unwrap() == "gain" {
        1
    } else {
        -1
    };
    let amount = words.next().unwrap().parse::<i32>().unwrap();
    let mut words = words.skip(6);
    let p2 = words.next().unwrap();
    ((p1, &p2[0..p2.len() - 1]), sign * amount)
}

fn parse_name(s: &str) -> &str {
    s.split(' ').next().unwrap()
}

fn optimal_happiness(graph: &Graph, n_people: usize, seating_list: &Seating) -> i32 {
    let mut seating = vec![];
    _optimal_happiness(graph, seating_list, n_people, &mut seating, 0)
}

fn _optimal_happiness(
    graph: &Graph,
    seating_list: &Seating,
    n_people: usize,
    cur_seating: &mut Vec<usize>,
    cur_cost: usize,
) -> i32 {
    let max = i32::MIN;
    for i in 0..n_people {
        cur_seating.push(i);
        let value = _optimal_happiness(graph, seating_list, n_people, cur_seating, cur_cost);
        if value > max {
            max = value;
        }
        cur_seating.pop();
    }
    max
}

impl<'a> Day<'a> for Day13 {
    type Input = (Graph<'a>, SeatingList<'a>);
    type Output = usize;

    fn day_number() -> usize {
        13
    }

    fn part1(input: &Self::Input) -> Self::Output {
        println!("{:?}", input);
        10
    }

    fn part2(input: &Self::Input) -> Self::Output {
        0
    }

    fn parse(input: &'a str) -> Self::Input {
        (
            input.lines().map(parse_graph_entry).collect(),
            input
                .lines()
                .map(parse_name)
                .collect::<HashSet<&'a str>>()
                .into_iter()
                .collect(),
        )
    }
}
