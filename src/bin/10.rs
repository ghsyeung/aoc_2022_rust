use std::str::FromStr;

use model::{Command, Machine};

mod model {
    use std::str::FromStr;

    use crate::parser;

    #[derive(Debug, Clone, Copy)]
    pub enum Command {
        Noop,
        Addx(isize),
    }

    impl FromStr for Command {
        type Err = eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (_, command) =
                parser::parse(s).map_err(|_| eyre::eyre!("Unable to parse: {}", s))?;
            Ok(command)
        }
    }

    #[derive(Debug)]
    pub struct Machine {
        cycle: usize,
        history: Vec<isize>,
    }

    impl Default for Machine {
        fn default() -> Self {
            Self {
                cycle: 1,
                history: vec![1],
            }
        }
    }

    impl Machine {
        pub fn run_command(&mut self, command: &Command) {
            match command {
                Command::Noop => {
                    self.cycle += 1;
                    self.history.push(*self.history.last().unwrap());
                }
                Command::Addx(n) => {
                    self.cycle += 2;
                    let reg_x = *self.history.last().unwrap();
                    self.history.push(reg_x);
                    self.history.push(reg_x + n);
                }
            }
            dbg!(&self.cycle, &self.get_at_specific_cycles());
        }

        const CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];
        pub fn get_at_specific_cycles(&self) -> [(usize, isize); 6] {
            Self::CYCLES.map(|i| (i, *self.history.get(i - 1).unwrap_or(&0)))
        }

        const WIDTH: isize = 40;
        pub fn draw(&self) -> String {
            self.history
                .iter()
                .enumerate()
                .map(|(cycle, pos)| {
                    dbg!(cycle, pos, pos + 2);
                    let c: isize = (cycle as isize % Self::WIDTH) + 1;
                    if c >= *pos && c <= (*pos + 2) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<Vec<char>>()
                .chunks(40)
                .map(|it| format!("{}\n", it.iter().collect::<String>()))
                .collect()
        }
    }
}

mod parser {
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
        sequence::preceded, IResult,
    };

    use crate::model::Command;

    fn parse_noop(s: &str) -> IResult<&str, Command> {
        map(tag("noop"), |_| Command::Noop)(s)
    }

    fn parse_addx(s: &str) -> IResult<&str, Command> {
        map(preceded(tag("addx "), i64), |n| Command::Addx(n as isize))(s)
    }

    pub fn parse(s: &str) -> IResult<&str, Command> {
        alt((parse_noop, parse_addx))(s)
    }
}

pub fn part_one(input: &str) -> eyre::Result<isize> {
    let mut machine = Machine::default();
    for line in input.lines() {
        let command = Command::from_str(line)?;
        machine.run_command(&command);
    }

    let mut total = 0;
    for (cycle_num, reg_x) in machine.get_at_specific_cycles() {
        total += cycle_num as isize * reg_x;
    }
    Ok(total)
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    let mut machine = Machine::default();
    for line in input.lines() {
        let command = Command::from_str(line)?;
        machine.run_command(&command);
    }
    let look = machine.draw();
    println!("{}", look);

    todo!()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input).unwrap(), 13140);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        // assert_eq!(part_two(&input).unwrap(), 0);
    }
}
