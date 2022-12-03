use itertools::Itertools;

pub fn part_one(input: &str) -> color_eyre::Result<u32> {
    let mut max: Option<u32> = None;
    let iter = input.split('\n');
    let mut current: Option<u32> = None;

    for i in iter.into_iter() {
        if i.is_empty() {
            // update max
            if let Some(c) = current {
                max = if let Some(m) = max {
                    Some(if c > m { c } else { m })
                } else {
                    Some(c)
                };
                current = None;
            }
        } else {
            current = if let Some(c) = current {
                Some(c + i.parse::<u32>().unwrap())
            } else {
                Some(i.parse::<u32>().unwrap())
            }
        }
    }
    match max {
        Some(s) => Ok(s),
        None => Err(eyre::eyre!("invalid")),
    }
}

pub fn part_two(input: &str) -> color_eyre::Result<u32> {
    let top_3 = input
        .split('\n')
        // translate into [Some(1), Some(2), None, Some(3), Some(4), ...]
        .map(|s| s.parse::<u32>().ok())
        // group them by is_some
        // [ (true, [Some(1), Some(2)]), (false, [None]), (true, [Some(3), Some(4)]), ...]
        .group_by(|s| s.is_some())
        .into_iter()
        // filter out None groups
        // [ (true, [Some(1), Some(2)]), (true, [Some(3), Some(4)]), ...]
        .filter(|(is_some, _group)| *is_some)
        // map to take only _.1 while unwrapping and summing up
        .map(|(_, group)| {
            group
                .into_iter()
                // [ [1, 2], [3, 4], ...]
                .map(Option::unwrap)
                .sum()
        })
        .sorted_by(|a: &u32, b| Ord::cmp(b, a))
        .take(3);

    Ok(top_3.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input).unwrap(), 24000);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input).unwrap(), 45000);
    }
}
