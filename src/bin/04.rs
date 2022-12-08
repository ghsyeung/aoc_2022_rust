use std::str::FromStr;

struct Range(usize, usize);

impl Range {
    fn is_completely_subset(&self, other: &Range) -> bool {
        let Self(l, r) = self;
        let Self(u, v) = other;

        l >= u && r <= v
    }

    fn is_overlap(&self, other: &Range) -> bool {
        let Self(l, r) = self;
        let Self(u, v) = other;

        !(r < u || l > v)
    }
}

impl FromStr for Range {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("-");
        let l = parts
            .next()
            .ok_or(eyre::eyre!("Invalid Range: missing left"))?
            .parse()?;
        let r = parts
            .next()
            .ok_or(eyre::eyre!("Invalid Range: missing right"))?
            .parse()?;
        Ok(Range(l, r))
    }
}

struct Pair(Range, Range);
impl FromStr for Pair {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(",");
        let l = Range::from_str(
            parts
                .next()
                .ok_or(eyre::eyre!("Invalid Group: missing first elf"))?,
        )?;
        let r = Range::from_str(
            parts
                .next()
                .ok_or(eyre::eyre!("Invalid Group: missing second elf"))?,
        )?;

        Ok(Pair(l, r))
    }
}

pub fn part_one(input: &str) -> color_eyre::Result<usize> {
    let mut counter = 0;
    for line in input.lines() {
        let pair = Pair::from_str(line)?;
        let Pair(f, s) = pair;
        if f.is_completely_subset(&s) || s.is_completely_subset(&f) {
            counter += 1;
        }
    }
    Ok(counter)
}

pub fn part_two(input: &str) -> color_eyre::Result<usize> {
    let mut counter = 0;
    for line in input.lines() {
        let pair = Pair::from_str(line)?;
        let Pair(f, s) = pair;
        if f.is_overlap(&s) {
            counter += 1;
        }
    }
    Ok(counter)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input).unwrap(), 2);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input).unwrap(), 4);
    }
}
