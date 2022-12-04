use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

struct RuckSack(Compartment, Compartment);

impl FromStr for RuckSack {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 1 {
            Err(eyre::eyre!("Invalid RuckSack length: {}", s.len()))
        } else {
            let (first, second) = s.split_at(s.len() / 2);

            let c1 = Compartment::from_str(first)?;
            let c2 = Compartment::from_str(second)?;
            Ok(RuckSack(c1, c2))
        }
    }
}

impl RuckSack {
    pub fn all_item_types(&self) -> HashSet<char> {
        let Self(first, second) = self;
        first.0.union(&second.0).map(ToOwned::to_owned).collect()
    }
    pub fn common(&self) -> Vec<char> {
        let Self(first, second) = self;
        first
            .0
            .intersection(&second.0)
            .map(ToOwned::to_owned)
            .collect()
    }
}

fn priority(c: &char) -> color_eyre::Result<u32> {
    match c {
        'A'..='Z' => Ok(u32::try_from(*c)? - u32::try_from('A')? + 27),
        'a'..='z' => Ok(u32::try_from(*c)? - u32::try_from('a')? + 1),
        _ => Err(eyre::eyre!("Invalid character: {}", c)),
    }
}

struct Compartment(HashSet<char>);

impl FromStr for Compartment {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h: HashSet<char> = HashSet::from_iter(s.chars());
        Ok(Compartment(h))
    }
}

pub fn part_one(input: &str) -> color_eyre::Result<u32> {
    let mut total = 0;
    for line in input.lines() {
        let rucksack = RuckSack::from_str(line)?;
        let v: u32 = rucksack
            .common()
            .iter()
            .map(priority)
            .collect::<color_eyre::Result<Vec<u32>>>()?
            .iter()
            .sum();
        total += v;
    }
    Ok(total)
}

pub fn part_two(input: &str) -> color_eyre::Result<u32> {
    let mut total = 0;
    let mut it = input.lines().peekable();
    loop {
        if it.peek().is_none() {
            break;
        }

        let s1 = RuckSack::from_str(it.next().ok_or(eyre::eyre!("Missing expected line"))?)?;
        let s2 = RuckSack::from_str(it.next().ok_or(eyre::eyre!("Missing expected line"))?)?;
        let s3 = RuckSack::from_str(it.next().ok_or(eyre::eyre!("Missing expected line"))?)?;

        let common: HashSet<char> = s3
            .all_item_types()
            .intersection(
                &s1.all_item_types()
                    .intersection(&s2.all_item_types())
                    .map(ToOwned::to_owned)
                    .collect(),
            )
            .map(ToOwned::to_owned)
            .collect();

        if common.len() != 1 {
            Err(eyre::eyre!(
                "Fail invariant: More than 1 common type ingroup"
            ))?
        }
        total += priority(
            common
                .iter()
                .next()
                .ok_or(eyre::eyre!("Fail invariant: No common type in group"))?,
        )?;
    }

    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input).unwrap(), 157);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input).unwrap(), 70);
    }
}
