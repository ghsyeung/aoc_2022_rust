use model::Packet;

use crate::logic::is_right_order;

mod parser {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{digit1, line_ending},
        combinator::{map, map_res},
        multi::separated_list0,
        sequence::tuple,
        IResult,
    };

    use crate::model::Packet;

    pub fn parse_num(s: &str) -> IResult<&str, Packet> {
        map_res(digit1, |s: &str| s.parse::<usize>().map(|u| Packet::Num(u)))(s)
    }

    pub fn parse_items(s: &str) -> IResult<&str, Vec<Packet>> {
        separated_list0(tag(","), alt((parse_num, parse_list)))(s)
    }

    pub fn parse_list(s: &str) -> IResult<&str, Packet> {
        map(tuple((tag("["), parse_items, tag("]"))), |(_, items, _)| {
            Packet::List(items)
        })(s)
    }

    pub fn parse_pairs(s: &str) -> IResult<&str, (Packet, Packet)> {
        map(
            tuple((
                parse_list,
                line_ending,
                parse_list,
                line_ending,
                line_ending,
            )),
            |(m, _, n, _, _)| (m, n),
        )(s)
    }
}

mod model {
    use std::str::FromStr;

    use crate::{
        logic::{self, Outcome},
        parser,
    };

    #[derive(Debug, Clone)]
    pub enum Packet {
        Num(usize),
        List(Vec<Packet>),
    }
    impl FromStr for Packet {
        type Err = eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            parser::parse_list(s)
                .map(|(_, p)| p)
                .map_err(|e| eyre::eyre!(e.to_string()))
        }
    }

    impl PartialEq for Packet {
        fn eq(&self, other: &Self) -> bool {
            match logic::comp((&self, &other)) {
                Outcome::Continue => true,
                _ => false,
            }
        }
    }
    impl Eq for Packet {}

    impl PartialOrd for Packet {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(match logic::is_right_order((self, other)) {
                true => std::cmp::Ordering::Less,
                false => std::cmp::Ordering::Greater,
            })
        }
    }
    impl Ord for Packet {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match logic::comp((self, other)) {
                Outcome::Continue => std::cmp::Ordering::Equal,
                Outcome::Right => std::cmp::Ordering::Less,
                Outcome::Wrong => std::cmp::Ordering::Greater,
            }
        }
    }
}

mod logic {
    use crate::model::Packet;

    pub enum Outcome {
        Continue,
        Right,
        Wrong,
    }

    pub fn comp((l, r): (&Packet, &Packet)) -> Outcome {
        match (l, r) {
            (Packet::Num(s), Packet::Num(t)) => {
                if s < t {
                    Outcome::Right
                } else if s == t {
                    Outcome::Continue
                } else {
                    Outcome::Wrong
                }
            }
            (s @ Packet::Num(_), t @ Packet::List(_)) => comp((&Packet::List(vec![s.clone()]), t)),
            (s @ Packet::List(_), t @ Packet::Num(_)) => comp((s, &Packet::List(vec![t.clone()]))),
            (Packet::List(s), Packet::List(t)) => {
                let mut s = s.into_iter();
                let mut t = t.into_iter();
                loop {
                    let i = s.next();
                    let j = t.next();
                    match (i, j) {
                        (None, None) => return Outcome::Continue,
                        (None, Some(_)) => return Outcome::Right,
                        (Some(_), None) => return Outcome::Wrong,
                        (Some(s), Some(t)) => {
                            let r = comp((s, t));
                            match r {
                                Outcome::Continue => {}
                                a @ _ => return a,
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn is_right_order((l, r): (&Packet, &Packet)) -> bool {
        match comp((l, r)) {
            Outcome::Continue => false,
            Outcome::Right => true,
            Outcome::Wrong => false,
        }
    }
}

pub fn part_one(input: &str) -> eyre::Result<usize> {
    let mut input = input;
    let mut pairs: Vec<(Packet, Packet)> = Default::default();
    loop {
        let (s, pair) = parser::parse_pairs(input).map_err(|e| eyre::eyre!(e.to_string()))?;
        pairs.push(pair);
        input = s;
        if input.is_empty() {
            break;
        }
    }

    Ok(pairs
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| is_right_order((l, r)))
        .map(|(i, _)| i + 1)
        .sum())
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    let mut packets = input
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            parser::parse_list(s)
                .map_err(|e| eyre::eyre!(e.to_string()))
                .map(|(_, p)| p)
        })
        .collect::<eyre::Result<Vec<Packet>>>()?;

    let divisor = vec![
        parser::parse_list("[[2]]").map(|(_, p)| p).unwrap(),
        parser::parse_list("[[6]]").map(|(_, p)| p).unwrap(),
    ];

    packets.append(&mut divisor.clone());
    packets.sort();
    let mut s = 0;
    let mut t = 0;
    for (i, p) in packets.iter().enumerate() {
        if &divisor[0] == p {
            s = i + 1;
        } else if &divisor[1] == p {
            t = i + 1;
        }
        // println!("{}: {:?}", &i, &p);
    }
    Ok(s * t)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input).unwrap(), 13);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input).unwrap(), 140);
    }
}
