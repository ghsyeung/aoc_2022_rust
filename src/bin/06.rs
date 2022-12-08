use std::collections::HashSet;

use itertools::Itertools;

const DATAGRAM_WINDOW: usize = 4;
pub fn part_one(input: &str) -> color_eyre::Result<usize> {
    for (i, window) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(DATAGRAM_WINDOW)
        .enumerate()
    {
        let s: HashSet<&char> = window.iter().collect();
        if s.len() == window.len() {
            return Ok(i + DATAGRAM_WINDOW);
        }
    }
    Err(eyre::eyre!("Cannot find start of datagram"))
}

const MESSAGE_WINDOW: usize = 14;
pub fn part_two(input: &str) -> color_eyre::Result<usize> {
    for (i, window) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(MESSAGE_WINDOW)
        .enumerate()
    {
        let s: HashSet<&char> = window.iter().collect();
        if s.len() == window.len() {
            return Ok(i + MESSAGE_WINDOW);
        }
    }
    Err(eyre::eyre!("Cannot find start of message"))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input).unwrap(), 10);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input).unwrap(), 29);
    }
}
