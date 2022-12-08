mod model {
    use std::{fmt::Display, str::FromStr};

    #[derive(Debug, Clone, Copy)]
    pub struct Crate(pub char);
    impl ToString for Crate {
        fn to_string(&self) -> String {
            format!("[{}]", self.0)
        }
    }

    #[derive(Default, Debug, Clone)]
    pub struct CargoStack(Vec<Crate>);
    impl ToString for CargoStack {
        fn to_string(&self) -> String {
            self.0.iter().map(ToString::to_string).collect()
        }
    }

    #[derive(Default, Debug)]
    pub struct CargoStacks(Vec<CargoStack>);

    impl Display for CargoStacks {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for (i, t) in self.0.iter().enumerate() {
                f.write_fmt(format_args!("{} {}\n", i + 1, t.to_string()))?;
            }
            Ok(())
        }
    }

    impl CargoStacks {
        pub fn take_top(&self) -> Vec<Option<Crate>> {
            self.0
                .iter()
                .map(|CargoStack(c)| c.last().cloned())
                .collect()
        }

        pub fn perform_move_9001(&mut self, command: &Move) -> color_eyre::Result<()> {
            let Self(cargo_stacks_vec) = self;

            let from_stack = cargo_stacks_vec
                .get_mut(command.from - 1)
                .ok_or(eyre::eyre!("Missing from stack"))?;
            let mut top_crates = from_stack
                .0
                .split_off(from_stack.0.len() - command.how_many);

            let to_stack = cargo_stacks_vec
                .get_mut(command.to - 1)
                .ok_or(eyre::eyre!("Missing to stack"))?;
            to_stack.0.append(&mut top_crates);
            Ok(())
        }

        pub fn perform_move_9000(&mut self, command: &Move) -> color_eyre::Result<()> {
            let Self(cargo_stacks_vec) = self;

            for _i in 0..command.how_many {
                let from_stack = cargo_stacks_vec
                    .get_mut(command.from - 1)
                    .ok_or(eyre::eyre!("Missing from stack"))?;
                let top_crate = from_stack.0.pop();

                if let Some(f) = top_crate {
                    let to_stack = cargo_stacks_vec
                        .get_mut(command.to - 1)
                        .ok_or(eyre::eyre!("Missing to stack"))?;
                    to_stack.0.push(f);
                } else {
                    Err(eyre::eyre!("No more crates at {}", command.from))?
                }
            }
            Ok(())
        }

        pub fn from_lines<'a, I>(mut it: I) -> color_eyre::Result<Self>
        where
            I: Iterator<Item = &'a str>,
        {
            let column_labels = it.next().ok_or(eyre::eyre!("Missing column labels"))?;
            let num_of_columns = column_labels.split_whitespace().count();

            let mut cargo_stacks_vec = vec![CargoStack::default(); num_of_columns];

            for row_str in it {
                let mut rit = row_str.chars().peekable();
                let mut col_index: usize = 0;
                while rit.peek().is_some() {
                    // either "   " or "[X]"
                    rit.next();
                    let cell = rit.next();
                    if let Some(i) = cell {
                        if i.is_ascii_alphabetic() {
                            if let Some(col_stack) = cargo_stacks_vec.get_mut(col_index) {
                                col_stack.0.push(Crate(i));
                            } else {
                                let mut new_stack = CargoStack::default();
                                new_stack.0.push(Crate(i));
                                cargo_stacks_vec[col_index] = new_stack;
                            }
                        }
                    }
                    rit.next();

                    // eat the empty space
                    rit.next();
                    col_index += 1;
                }
            }
            Ok(CargoStacks(cargo_stacks_vec))
        }
    }

    #[derive(Debug)]
    pub struct Move {
        how_many: usize,
        from: usize,
        to: usize,
    }

    impl Display for Move {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!(
                "({}) {} -> {}",
                self.how_many, self.from, self.to
            ))
        }
    }

    impl FromStr for Move {
        type Err = color_eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut it = s.split_whitespace();
            let _cmd = it.next();
            let how_many: usize = it.next().ok_or(eyre::eyre!("missing how_many"))?.parse()?;
            let _from = it.next();
            let from: usize = it.next().ok_or(eyre::eyre!("missing from"))?.parse()?;
            let _to = it.next();
            let to: usize = it.next().ok_or(eyre::eyre!("missing to"))?.parse()?;
            Ok(Move { how_many, from, to })
        }
    }
}

mod preprocess {
    use crate::model::{CargoStacks, Move};
    use std::str::FromStr;

    pub fn read_input(input: &str) -> color_eyre::Result<(CargoStacks, Vec<Move>)> {
        let mut it = input.lines();
        let mut chart_input = it
            .by_ref()
            .take_while(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        chart_input.reverse();

        let cargo_stacks = CargoStacks::from_lines(chart_input.into_iter())?;

        let commands = it
            .map(Move::from_str)
            .collect::<eyre::Result<Vec<Move>>>()?;
        Ok((cargo_stacks, commands))
    }
}

pub fn part_one(input: &str) -> color_eyre::Result<String> {
    let (mut cargo_stacks, commands) = preprocess::read_input(input)?;

    dbg!(&cargo_stacks, &commands);
    println!("start: \n{}\n", &cargo_stacks);
    for command in commands {
        cargo_stacks.perform_move_9000(&command)?;
        println!("after command {}:\n{}\n", &command, &cargo_stacks);
    }

    Ok(cargo_stacks
        .take_top()
        .iter()
        .map(|r| match r {
            Some(c) => c.0,
            None => ' ',
        })
        .collect::<String>())
}

pub fn part_two(input: &str) -> color_eyre::Result<String> {
    let (mut cargo_stacks, commands) = preprocess::read_input(input)?;

    dbg!(&cargo_stacks, &commands);
    println!("start: \n{}\n", &cargo_stacks);
    for command in commands {
        cargo_stacks.perform_move_9001(&command)?;
        println!("after command {}:\n{}\n", &command, &cargo_stacks);
    }

    Ok(cargo_stacks
        .take_top()
        .iter()
        .map(|r| match r {
            Some(c) => c.0,
            None => ' ',
        })
        .collect::<String>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input).unwrap(), "CMZ");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input).unwrap(), "MCD");
    }
}
