use crate::common::Day;

pub struct Day6;

type Coord = (usize, usize);
type Rectangle = (Coord, Coord);

#[derive(Debug)]
pub enum Instruction {
    TurnOn(Rectangle),
    TurnOff(Rectangle),
    Toggle(Rectangle),
}

struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T: Clone + Default> Grid<T> {
    fn new() -> Grid<T> {
        Grid {
            data: vec![vec![T::default(); 1000]; 1000],
        }
    }
}

impl Grid<bool> {
    fn count_lights(&self) -> u32 {
        let mut count = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                if self.data[i][j] {
                    count += 1;
                }
            }
        }
        count
    }

    fn exec_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::TurnOn(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] = true;
                    }
                }
            }
            Instruction::TurnOff(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] = false;
                    }
                }
            }
            Instruction::Toggle(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] = !self.data[i][j];
                    }
                }
            }
        }
    }
}

impl Grid<u32> {
    fn count_lights(&self) -> u32 {
        let mut count = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                count += self.data[i][j];
            }
        }
        count
    }

    fn exec_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::TurnOn(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] += 1;
                    }
                }
            }
            Instruction::TurnOff(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] = self.data[i][j].saturating_sub(1);
                    }
                }
            }
            Instruction::Toggle(((begin_row, begin_col), (end_row, end_col))) => {
                for i in *begin_row..end_row + 1 {
                    for j in *begin_col..end_col + 1 {
                        self.data[i][j] += 2;
                    }
                }
            }
        }
    }
}

fn parse_coord(s: &str) -> Option<Coord> {
    let mut coords = s.split(',');
    let x = coords.next()?.parse::<usize>().ok()?;
    let y = coords.next()?.parse::<usize>().ok()?;
    Some((x, y))
}

fn parse_rect(line: &str) -> Option<Rectangle> {
    let mut words = line.split(' ');
    let begin = words.next().map(parse_coord)??;
    words.next();
    let end = words.next().map(parse_coord)??;
    Some((begin, end))
}

fn parse_instruction(line: &str) -> Option<Instruction> {
    if let Some(rest) = line.strip_prefix("turn on ") {
        Some(Instruction::TurnOn(parse_rect(rest)?))
    } else if let Some(rest) = line.strip_prefix("turn off ") {
        Some(Instruction::TurnOff(parse_rect(rest)?))
    } else if let Some(rest) = line.strip_prefix("toggle ") {
        Some(Instruction::Toggle(parse_rect(rest)?))
    } else {
        panic!("This should never happen");
    }
}

impl Day for Day6 {
    type Input = Vec<Instruction>;
    type Output = usize;

    fn part1(input: &Self::Input) -> Self::Output {
        let mut grid = Grid::<bool>::new();
        for instruction in input {
            grid.exec_instruction(instruction);
        }

        grid.count_lights() as usize
    }

    fn part2(input: &Self::Input) -> Self::Output {
        let mut grid: Grid<u32> = Grid::new();
        for instruction in input {
            grid.exec_instruction(instruction);
        }

        grid.count_lights() as usize
    }

    fn parse(input: &str) -> Self::Input {
        input
            .lines()
            .map(parse_instruction)
            .map(|x| x.unwrap())
            .collect()
    }
}
