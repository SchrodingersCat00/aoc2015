use std::{collections::HashMap};

use crate::common::Day;

pub struct Day7;

type Wire = String;
#[derive(Debug, Clone)]
pub struct Circuit {
    connections: HashMap<Wire, Signal>,
}

impl Circuit {
    fn new(connections_vector: Vec<(Wire, Signal)>) -> Circuit {
        Circuit {
            connections: HashMap::from_iter(connections_vector),
        }
    }

    fn set_wire(&mut self, wire: Wire, signal: Signal) {
        self.connections.insert(wire, signal);
    }

    fn eval_basic_signal(&mut self, basic_signal: &BasicSignal) -> Option<u16> {
        match basic_signal {
            BasicSignal::Literal(x) => Some(*x),
            BasicSignal::Wire(wire) => {
                let val = self.eval_wire(&wire);
                self.connections.insert(
                    wire.clone(),
                    Signal::BasicSignal(BasicSignal::Literal(val.unwrap())),
                );
                val
            }
        }
    }

    fn eval_wire(&mut self, wire: &Wire) -> Option<u16> {
        println!("Evaluating wire: {}", wire);
        match self.connections.get(wire)?.clone() {
            Signal::BasicSignal(x) => self.eval_basic_signal(&x),
            Signal::Gate(gate) => match gate {
                Gate::And(x, y) => Some(self.eval_basic_signal(&x)? & self.eval_basic_signal(&y)?),
                Gate::Or(x, y) => Some(self.eval_basic_signal(&x)? | self.eval_basic_signal(&y)?),
                Gate::LShift(x, y) => {
                    Some(self.eval_basic_signal(&x)? << self.eval_basic_signal(&y)?)
                }
                Gate::RShift(x, y) => {
                    Some(self.eval_basic_signal(&x)? >> self.eval_basic_signal(&y)?)
                }
                Gate::Not(x) => Some(!self.eval_basic_signal(&x)?),
            },
        }
    }
}

pub type Connection = (Wire, Signal);

#[derive(Debug, Clone)]
pub enum BasicSignal {
    Literal(u16),
    Wire(Wire),
}

#[derive(Debug, Clone)]
pub enum Gate {
    And(BasicSignal, BasicSignal),
    Or(BasicSignal, BasicSignal),
    Not(BasicSignal),
    LShift(BasicSignal, BasicSignal),
    RShift(BasicSignal, BasicSignal),
}

#[derive(Debug, Clone)]
pub enum Signal {
    Gate(Gate),
    BasicSignal(BasicSignal),
}

fn parse_basic_signal(basic_signal: &str) -> BasicSignal {
    match basic_signal.parse::<u16>() {
        Ok(x) => BasicSignal::Literal(x),
        Err(_) => BasicSignal::Wire(String::from(basic_signal)),
    }
}

fn parse_signal(signal: &str) -> Signal {
    let parts: Vec<&str> = signal.split(' ').collect();
    if parts.len() == 1 {
        Signal::BasicSignal(parse_basic_signal(parts[0]))
    } else if parts.len() == 2 {
        Signal::Gate(Gate::Not(parse_basic_signal(parts[1])))
    } else if parts.len() == 3 {
        let op1 = parse_basic_signal(parts[0]);
        let op2 = parse_basic_signal(parts[2]);
        match parts[1] {
            "AND" => Signal::Gate(Gate::And(op1, op2)),
            "OR" => Signal::Gate(Gate::Or(op1, op2)),
            "LSHIFT" => Signal::Gate(Gate::LShift(op1, op2)),
            "RSHIFT" => Signal::Gate(Gate::RShift(op1, op2)),
            _ => panic!("Invalid gate, {}", parts[1]),
        }
    } else {
        panic!("Invalid input: {}", signal);
    }
}

fn parse_connection(connection: &str) -> Connection {
    let mut parts = connection.split(" -> ");
    let signal = parse_signal(parts.next().unwrap());
    let wire = parts.next().unwrap().to_string();
    (wire, signal)
}

impl Day for Day7 {
    type Input = Circuit;
    type Output = u16;

    fn part1(input: &Self::Input) -> Self::Output {
        let mut input_clone= input.clone();
        input_clone.eval_wire(&"a".to_string()).unwrap()
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut input_clone= input.clone();
        input_clone.set_wire("b".to_string(), Signal::BasicSignal(BasicSignal::Literal(3176)));
        input_clone.eval_wire(&"a".to_string()).unwrap()
    }

    fn parse(input: &str) -> Self::Input {
        Circuit::new(input.lines().map(parse_connection).collect())
    }
}
