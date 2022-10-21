use crate::common::Day;

pub struct Day14 {}

#[derive(Debug)]
pub struct Reindeer<'a> {
    name: &'a str,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

#[derive(Debug, Clone)]
pub enum Status {
    Flying,
    Resting,
}

#[derive(Debug, Clone)]
pub struct ReindeerState<'a> {
    reindeer: &'a Reindeer<'a>,
    status: Status,
    busy_time: usize,
    travel_dist: usize,
    points: usize,
}

impl<'a> ReindeerState<'a> {
    fn new(reindeer: &'a Reindeer) -> Self {
        ReindeerState {
            reindeer,
            status: Status::Resting,
            busy_time: 0,
            travel_dist: 0,
            points: 0,
        }
    }

    fn progress_second(&mut self) {
        if self.busy_time == 0 {
            self.status = match self.status {
                Status::Flying => {
                    self.busy_time = self.reindeer.rest_time;
                    Status::Resting
                },
                Status::Resting => {
                    self.busy_time = self.reindeer.fly_time;
                    Status::Flying
                }
            }
        }
        if let Status::Flying = self.status {
            self.travel_dist += self.reindeer.speed;
        }
        self.busy_time -= 1;
    }
}

fn parse_reindeer(s: &str) -> Reindeer {
    let mut iter = s.split(' ');
    let name = iter.next().unwrap();
    let speed = iter.nth(2).unwrap().parse::<usize>().unwrap();
    let fly_time = iter.nth(2).unwrap().parse::<usize>().unwrap();
    let rest_time = iter.nth(6).unwrap().parse::<usize>().unwrap();
    Reindeer {
        name,
        speed,
        fly_time,
        rest_time,
    }
}

fn find_furthest_dist(states: &Vec<ReindeerState>) -> usize {
    let mut max: usize = 0;
    for state in states {
        if state.travel_dist > max {
            max = state.travel_dist;
        }
    }
    max
}

fn simulate<'a>(init_states: &Vec<ReindeerState<'a>>, seconds: usize) -> Vec<ReindeerState<'a>> {
    let mut out_states: Vec<ReindeerState> = init_states.clone();
    for _ in 0..seconds {
        for state in &mut out_states {
            state.progress_second();
        }
        let furthest = find_furthest_dist(&out_states);
        for state in &mut out_states {
            if state.travel_dist == furthest {
                state.points += 1;
            }
        }
        
    }
    out_states
}

impl<'a> Day<'a> for Day14 {
    type Input = Vec<Reindeer<'a>>;
    type Output = usize;

    fn day_number() -> usize {
        14
    }

    fn part1(input: &Self::Input) -> Self::Output {
        let states: Vec<ReindeerState> = input.iter().map(ReindeerState::new).collect();
        let new_states = simulate(&states, 2503);
        find_furthest_dist(&new_states)
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let states: Vec<ReindeerState> = input.iter().map(ReindeerState::new).collect();
        let new_states = simulate(&states, 2503);
        let mut max: usize = 0;
        let mut name: &str = "none";
        for state in new_states {
            if state.points > max {
                max = state.points;
                name = state.reindeer.name;
            }
        }
        println!("{}", name);
        max
    }

    fn parse(input: &'a str) -> Self::Input {
        input.lines().map(parse_reindeer).collect()
    }
}
