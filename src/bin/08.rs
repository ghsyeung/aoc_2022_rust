use std::str::FromStr;

struct Grid {
    data: Vec<u32>,
    size: usize,
}

impl Grid {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn at(&self, x: usize, y: usize) -> u32 {
        self.data[x + y * self.size]
    }

    pub fn up(&self, x: usize, y: usize) -> impl Iterator<Item = u32> + '_ {
        (0..y).rev().map(move |v| self.at(x, v))
    }

    pub fn down(&self, x: usize, y: usize) -> impl Iterator<Item = u32> + '_ {
        ((y + 1)..self.size).map(move |v| self.at(x, v))
    }

    pub fn left(&self, x: usize, y: usize) -> impl Iterator<Item = u32> + '_ {
        (0..x).rev().map(move |u| self.at(u, y))
    }

    pub fn right(&self, x: usize, y: usize) -> impl Iterator<Item = u32> + '_ {
        ((x + 1)..self.size).map(move |u| self.at(u, y))
    }

    pub fn view_score(&self, x: usize, y: usize) -> usize {
        let me = self.at(x, y);
        let mut total_score = 1;
        for trees in [
            self.up(x, y).collect::<Vec<_>>(),
            self.down(x, y).collect::<Vec<_>>(),
            self.left(x, y).collect::<Vec<_>>(),
            self.right(x, y).collect::<Vec<_>>(),
        ] {
            let mut count = 0;
            for tree in trees.iter() {
                count += 1;
                if *tree >= me {
                    break;
                }
            }
            total_score *= usize::max(count, 1);
        }
        total_score
    }

    pub fn visible(&self, x: usize, y: usize) -> bool {
        let me = self.at(x, y);
        self.up(x, y).all(|h| h < me)
            || self.down(x, y).all(|h| h < me)
            || self.left(x, y).all(|h| h < me)
            || self.right(x, y).all(|h| h < me)
    }
}

impl FromStr for Grid {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_line, _rest) = s
            .split_once('\n')
            .ok_or(eyre::eyre!("Invalid grid with no lines"))?;
        let data: Vec<u32> = s
            .chars()
            .filter(char::is_ascii_digit)
            .map(|s| s.to_digit(10).ok_or(eyre::eyre!("Invalid digit")))
            .collect::<eyre::Result<Vec<_>>>()?;
        let size = first_line.len();
        Ok(Grid { data, size })
    }
}

pub fn part_one(input: &str) -> color_eyre::Result<usize> {
    let grid = Grid::from_str(input)?;
    let mut counter = 0;
    for i in 0..grid.size() {
        for j in 0..grid.size() {
            if grid.visible(i, j) {
                counter += 1;
            }
        }
    }
    Ok(counter)
}

pub fn part_two(input: &str) -> color_eyre::Result<usize> {
    let grid = Grid::from_str(input)?;
    let mut max: (Option<(usize, usize)>, usize) = (None, 0);
    for i in 0..grid.size() {
        for j in 0..grid.size() {
            let score = grid.view_score(i, j);
            max = if score > max.1 {
                (Some((i, j)), score)
            } else {
                max
            };
        }
    }
    dbg!(max);
    Ok(max.1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input).unwrap(), 21);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input).unwrap(), 16);
    }
}
