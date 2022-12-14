use std::cell::RefCell;

use nom::character::complete::line_ending;

use crate::model::{Monkey, Monkeys};

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{line_ending, multispace0, multispace1, not_line_ending, u16, u64},
        combinator::map,
        multi::separated_list1,
        sequence::{pair, separated_pair, tuple},
        IResult,
    };

    use crate::model::{Monkey, Operation, TestCondition};

    fn ignore_until_newline(s: &str) -> IResult<&str, ()> {
        map(not_line_ending, |_| ())(s)
    }

    fn ignore_until_include_newline(s: &str) -> IResult<&str, ()> {
        map(tuple((not_line_ending, line_ending)), |_| ())(s)
    }

    // parse usize separated by ,
    fn parse_items(s: &str) -> IResult<&str, Vec<usize>> {
        let (s, _) = tuple((multispace1, tag("Starting items:"), multispace0))(s)?;
        let (s, items) = separated_list1(pair(tag(","), multispace0), map(u16, |u| u as usize))(s)?;
        let (s, _) = ignore_until_include_newline(s)?;
        Ok((s, items))
    }

    fn parse_test_condition_divisible(s: &str) -> IResult<&str, usize> {
        let (s, _) = tuple((
            multispace1,
            tag("Test:"),
            multispace0,
            tag("divisible by"),
            multispace1,
        ))(s)?;
        let (s, divisible_by) = map(u16, |u| u as usize)(s)?;
        let (s, _) = ignore_until_include_newline(s)?;
        Ok((s, divisible_by))
    }

    fn parse_test_condition_true(s: &str) -> IResult<&str, usize> {
        let (s, _) = tuple((multispace1, tag("If true: throw to monkey"), multispace1))(s)?;
        let (s, if_true) = map(u64, |u| u as usize)(s)?;
        let (s, _) = ignore_until_include_newline(s)?;
        Ok((s, if_true))
    }

    fn parse_test_condition_false(s: &str) -> IResult<&str, usize> {
        let (s, _) = tuple((multispace1, tag("If false: throw to monkey"), multispace1))(s)?;
        let (s, if_false) = map(u64, |u| u as usize)(s)?;
        let (s, _) = ignore_until_include_newline(s)?;
        Ok((s, if_false))
    }

    fn parse_test_condition(s: &str) -> IResult<&str, TestCondition> {
        let (s, divisible_by) = parse_test_condition_divisible(s)?;
        let (s, if_true) = parse_test_condition_true(s)?;
        let (s, if_false) = parse_test_condition_false(s)?;
        Ok((
            s,
            TestCondition {
                divisible_by,
                if_true,
                if_false,
            },
        ))
    }

    pub fn parse_monkey(s: &str) -> IResult<&str, (usize, Monkey)> {
        let (s, id) = map(tuple((tag("Monkey"), multispace1, u64)), |(_, _, u)| {
            u as usize
        })(s)?;
        let (s, _) = ignore_until_include_newline(s)?;

        let (s, items) = parse_items(s)?;
        let (s, operation) = parse_operation(s)?;
        let (s, test_condition) = parse_test_condition(s)?;

        // eat the next line
        let (s, _) = ignore_until_newline(s)?;
        Ok((
            s,
            (
                id,
                Monkey {
                    items,
                    operation,
                    test: test_condition,
                    count: 0,
                },
            ),
        ))
    }

    fn parse_operation(s: &str) -> IResult<&str, Operation> {
        let (s, _) = tuple((
            multispace1,
            tag("Operation:"),
            multispace0,
            tag("new"),
            multispace0,
            tag("="),
            multispace0,
            tag("old"),
            multispace0,
        ))(s)?;

        // multiply and add
        let (s, operation) = alt((
            map(
                separated_pair(alt((tag("*"), tag("+"))), multispace0, tag("old")),
                |(sign, _)| match sign {
                    "*" => Operation::Square,
                    "+" => Operation::Multiply(2),
                    _ => panic!("shouldn't happen"),
                },
            ),
            map(
                // "* 12" or "+ 220"
                separated_pair(
                    alt((tag("*"), tag("+"))),
                    multispace0,
                    map(u16, |u| u as usize),
                ),
                |(sign, num)| match sign {
                    "*" => Operation::Multiply(num),
                    "+" => Operation::Add(num),
                    _ => panic!("shouldn't happen"),
                },
            ),
        ))(s)?;

        let (s, _) = ignore_until_include_newline(s)?;
        Ok((s, operation))
    }

    #[cfg(test)]
    mod test {
        use crate::model::Operation;

        use super::{parse_items, parse_operation};

        #[test]
        fn test_parse_operation() -> eyre::Result<()> {
            let (_, operation) = parse_operation("  Operation: new = old * 233\n")?;
            assert_eq!(operation, Operation::Multiply(233));
            Ok(())
        }

        #[test]
        fn test_parse_items() -> eyre::Result<()> {
            let (_, items) = parse_items("  Starting items: 54, 65, 75, 74\n")?;
            assert_eq!(items, vec![54, 65, 75, 74]);
            Ok(())
        }
    }
}

mod model {
    use std::cell::RefCell;

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub enum Operation {
        Multiply(usize),
        Add(usize),
        Square,
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    pub struct TestCondition {
        pub divisible_by: usize,
        pub if_true: usize,
        pub if_false: usize,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Monkey {
        pub items: Vec<usize>,
        pub operation: Operation,
        pub test: TestCondition,
        pub count: usize,
    }

    impl Monkey {
        pub fn inspect(&self, item: usize) -> usize {
            let o = match self.operation {
                Operation::Multiply(n) => item * n,
                Operation::Add(n) => item + n,
                Operation::Square => item * item,
            };
            (o as f64 / 3.0).floor() as usize
        }

        pub fn inspect_2(&self, item: usize) -> usize {
            let o = match self.operation {
                Operation::Multiply(n) => item * n,
                Operation::Add(n) => item + n,
                Operation::Square => item * item,
            };
            o
        }

        pub fn determine_target(&self, item: usize) -> usize {
            match item % self.test.divisible_by {
                0 => self.test.if_true,
                _ => self.test.if_false,
            }
        }

        pub fn determine_target_2(&self, item: usize, coprime: usize) -> usize {
            match item % coprime {
                0 => self.test.if_true,
                _ => self.test.if_false,
            }
        }

        pub fn add_inspect_count(&mut self, how_much: usize) {
            self.count += how_much;
        }
    }

    #[derive(Debug)]
    pub struct Monkeys(pub Vec<RefCell<Monkey>>);
    impl Monkeys {
        fn push_to_monkey_list(&self, id: usize, item: usize) {
            self.0[id].borrow_mut().items.push(item);
        }

        pub fn run(&mut self) {
            for m in self.0.iter() {
                let mut m = m.borrow_mut();
                let l = m.items.len();
                m.add_inspect_count(l);

                for item in m.items.iter() {
                    let new_item = m.inspect(*item);
                    let target = m.determine_target(new_item);
                    self.push_to_monkey_list(target, new_item);
                }
                m.items.clear();
            }
        }

        pub fn run_2(&mut self) {
            let coprime: usize = self
                .0
                .iter()
                .map(|r| r.borrow().test.divisible_by)
                .fold(1, |a, x| a * x);

            for m in self.0.iter() {
                let mut m = m.borrow_mut();
                let l = m.items.len();
                m.add_inspect_count(l);

                for item in m.items.iter() {
                    let new_item = m.inspect_2(*item);
                    let target = m.determine_target(new_item);

                    // Since all divisors are coprime, we can simply keep the modulo
                    let normalized_item = new_item % coprime;
                    self.push_to_monkey_list(target, normalized_item);
                }
                m.items.clear();
            }
        }

        pub fn get_counts(&self) -> Vec<usize> {
            self.0.iter().map(|r| r.borrow().count).collect()
        }

        pub fn print_debug(&self, round: usize) {
            dbg!(round, self);
        }

        pub fn print_active(&self, round: usize) {
            dbg!(round, self.get_counts());
        }
    }
}

pub fn part_one(input: &str) -> eyre::Result<usize> {
    let mut input: &str = &mut input.to_owned();
    let mut mvec: Vec<Monkey> = Default::default();

    while input.len() > 0 {
        let (s, (_id, monkey)) = parser::parse_monkey(input).map_err(|e| {
            dbg!(e);
            eyre::eyre!("Failed to parse monkey")
        })?;
        mvec.push(monkey);
        if s.len() == 0 {
            break;
        } else {
            let (mut ss, _) = line_ending::<&str, nom::error::Error<&str>>(s).map_err(|e| {
                dbg!(e);
                eyre::eyre!("Failed to parse monkey")
            })?;
            input = &mut ss;
        }
    }

    let mut monkeys: Monkeys = Monkeys(mvec.into_iter().map(RefCell::new).collect());
    for i in 0..20 {
        monkeys.run();
        // monkeys.print_debug(i);
    }

    let mut counts = monkeys.get_counts();
    counts.sort_by(Ord::cmp);
    counts.reverse();
    Ok(counts[0] * counts[1])
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    let mut input: &str = &mut input.to_owned();
    let mut mvec: Vec<Monkey> = Default::default();

    while input.len() > 0 {
        let (s, (_id, monkey)) = parser::parse_monkey(input).map_err(|e| {
            dbg!(e);
            eyre::eyre!("Failed to parse monkey")
        })?;
        mvec.push(monkey);
        if s.len() == 0 {
            break;
        } else {
            let (mut ss, _) = line_ending::<&str, nom::error::Error<&str>>(s).map_err(|e| {
                dbg!(e);
                eyre::eyre!("Failed to parse monkey")
            })?;
            input = &mut ss;
        }
    }

    let mut monkeys: Monkeys = Monkeys(mvec.into_iter().map(RefCell::new).collect());
    for i in 0..10000 {
        monkeys.run_2();
        // monkeys.print_debug(i);
    }

    let mut counts = monkeys.get_counts();
    counts.sort_by(Ord::cmp);
    counts.reverse();
    Ok(counts[0] * counts[1])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input).unwrap(), 10605);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input).unwrap(), 2713310158);
    }
}
