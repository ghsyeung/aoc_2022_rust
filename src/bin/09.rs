use std::{
    collections::HashMap,
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Location(isize, isize);
impl Add for Location {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        Location(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self(0, 0)
    }
}

impl Location {
    fn all_directions(&self) -> impl Iterator<Item = Location> + '_ {
        (-1..=1)
            .cartesian_product(-1..=1)
            .map(|(dx, dy)| Location(dx, dy))
            .map(|d| *self + d)
    }

    pub fn is_touching(&self, other: &Self) -> bool {
        self.all_directions().any(|d| &d == other)
    }

    pub fn move_direction(&self, tail: &Self) -> TailMove {
        let Self(dx, dy) = *self - *tail;
        if self.is_touching(tail) {
            TailMove::NONE
        } else if dx == 0 {
            if dy < 0 {
                TailMove::D
            } else {
                TailMove::U
            }
        } else if dy == 0 {
            if dx < 0 {
                TailMove::L
            } else {
                TailMove::R
            }
        } else if dy < 0 {
            if dx < 0 {
                TailMove::DL
            } else {
                TailMove::DR
            }
        } else if dy > 0 {
            if dx < 0 {
                TailMove::UL
            } else {
                TailMove::UR
            }
        } else {
            panic!("Impossible to get here")
        }
    }
}

#[derive(Debug, Default)]
struct Visit(HashMap<Location, usize>);
impl Visit {
    pub fn add(&mut self, tail: Location) {
        let e = self.0.entry(tail);
        e.and_modify(|s| *s += 1).or_insert(1);
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl From<&Dir> for Location {
    fn from(d: &Dir) -> Self {
        match d {
            Dir::Up => Location(0, -1),
            Dir::Down => Location(0, 1),
            Dir::Left => Location(-1, 0),
            Dir::Right => Location(1, 0),
        }
    }
}

impl FromStr for Dir {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Dir::Right),
            "L" => Ok(Dir::Left),
            "U" => Ok(Dir::Up),
            "D" => Ok(Dir::Down),
            _ => Err(eyre::eyre!("Invalid command: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Move(Dir, usize);
impl FromStr for Move {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();
        let dir = Dir::from_str(it.next().ok_or(eyre::eyre!("Missing command"))?)?;
        let steps = it
            .next()
            .ok_or(eyre::eyre!("Missing steps"))?
            .parse::<usize>()?;
        Ok(Move(dir, steps))
    }
}

enum TailMove {
    U,
    D,
    L,
    R,
    UL,
    UR,
    DL,
    DR,
    NONE,
}

impl From<TailMove> for Location {
    fn from(m: TailMove) -> Self {
        match m {
            TailMove::U => Location(0, 1),
            TailMove::D => Location(0, -1),
            TailMove::L => Location(-1, 0),
            TailMove::R => Location(1, 0),
            TailMove::UL => Location(-1, 1),
            TailMove::UR => Location(1, 1),
            TailMove::DL => Location(-1, -1),
            TailMove::DR => Location(1, -1),
            TailMove::NONE => Location(0, 0),
        }
    }
}

fn determine_tail_move(head: &Location, tail: &Location) -> TailMove {
    head.move_direction(tail)
}

fn run_simulation(visit: &mut Visit, moves: &[Move]) -> eyre::Result<()> {
    let mut head = Location(0, 0);
    let mut tail = Location(0, 0);
    visit.add(tail);

    let mut c = 0;
    for mmove in moves {
        let Move(d, s) = mmove;
        for _i in 0..*s {
            head = head + d.into();
            let tailmove = determine_tail_move(&head, &tail);
            tail = tail + tailmove.into();
            visit.add(tail);
            c += 1;
            dbg!(c, head, tail);
        }
    }
    Ok(())
}

fn run_simulation_n(n: usize, visit: &mut Visit, moves: &[Move]) -> eyre::Result<()> {
    let mut knots = vec![Location::default(); n];
    visit.add(*knots.last().unwrap());

    for mmove in moves {
        let Move(d, s) = mmove;
        for _s in 0..*s {
            let head = knots.get(0).unwrap();
            knots[0] = *head + d.into();
            for i in 1..n {
                let head = knots.get(i - 1).unwrap();
                let tail = knots.get(i).unwrap();
                let tailmove = determine_tail_move(&head, &tail);
                knots[i] = *tail + tailmove.into();
                if i == n - 1 {
                    visit.add(knots[i]);
                }
            }
        }
    }
    Ok(())
}

pub fn part_one(input: &str) -> eyre::Result<usize> {
    let moves = input
        .lines()
        .map(Move::from_str)
        .collect::<eyre::Result<Vec<_>>>()?;

    let mut visit = Visit::default();
    run_simulation(&mut visit, &moves)?;
    Ok(visit.count())
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    let moves = input
        .lines()
        .map(Move::from_str)
        .collect::<eyre::Result<Vec<_>>>()?;

    let mut visit = Visit::default();
    run_simulation_n(10, &mut visit, &moves)?;
    Ok(visit.count())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input).unwrap(), 13);
    }
    */

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input).unwrap(), 36);
    }
}
