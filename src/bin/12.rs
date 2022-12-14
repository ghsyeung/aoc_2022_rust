use std::str::FromStr;

use model::{HillMap, Walk, XY};

mod model {
    use std::{
        collections::{BinaryHeap, HashMap},
        fmt::Display,
        str::FromStr,
    };

    #[derive(Debug, Hash, Clone, Copy, Default, Eq, PartialEq)]
    pub struct XY(usize, usize);
    impl Display for XY {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({},{})", self.0, self.1)
        }
    }

    #[derive(Clone)]
    pub struct Contour(Vec<Vec<isize>>);
    impl Contour {
        pub fn at(&self, i: usize, j: usize) -> Option<isize> {
            self.0.get(j).and_then(|v| v.get(i)).map(|u| *u)
        }
    }

    fn char_to_isize(c: char) -> isize {
        match c {
            'S' => 0,
            'E' => 25,
            _ => (c as u8 - 'a' as u8) as isize,
        }
    }

    impl FromStr for HillMap {
        type Err = eyre::Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let lines: Vec<&str> = s.lines().collect();

            let mut start: XY = Default::default();
            let mut lowest: Vec<XY> = Default::default();
            let mut end: XY = Default::default();
            let mut contour: Vec<Vec<isize>> = Default::default();
            for (j, line) in lines.iter().enumerate() {
                let mut row: Vec<isize> = Default::default();
                for (i, c) in line.chars().enumerate() {
                    if c == 'S' {
                        start = XY(i, j);
                    } else if c == 'E' {
                        end = XY(i, j);
                    }
                    if c == 'S' || c == 'a' {
                        lowest.push(XY(i, j));
                    }
                    row.push(char_to_isize(c));
                }
                contour.push(row);
            }

            let hillmap = HillMap {
                x: contour[0].len(),
                y: contour.len(),
                lowest,
                contour: Contour(contour),
                start,
                end,
            };
            Ok(hillmap)
        }
    }

    #[derive(Clone)]
    pub struct HillMap {
        x: usize,
        y: usize,
        pub lowest: Vec<XY>,
        contour: Contour,
        pub start: XY,
        pub end: XY,
    }
    impl HillMap {
        pub fn movable_neighbours(&self, &XY(i, j): &XY) -> Vec<(XY, isize)> {
            let me = self
                .contour
                .at(i, j)
                .expect(&format!("Cannot find XY({}, {})", i, j));

            [
                (i as isize - 1, j as isize),
                (i as isize, j as isize - 1),
                (i as isize + 1, j as isize),
                (i as isize, j as isize + 1),
            ]
            .into_iter()
            // make sure neighbour is in grid
            .filter(|(i, j)| *i >= 0 && *j >= 0 && *i < self.x as isize && *j < self.y as isize)
            .map(|(i, j)| {
                let i = i as usize;
                let j = j as usize;
                // dbg!(i, j);
                (XY(i, j), self.contour.at(i, j).unwrap())
            })
            // make sure neighbour is not too high
            .filter(|(_xy, height)| height - me <= 1)
            .collect()
        }
    }

    #[derive(Eq, PartialEq)]
    pub struct Visitable(XY, usize);
    impl PartialOrd for Visitable {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.1.partial_cmp(&self.1)
        }
    }
    impl Ord for Visitable {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.1.cmp(&self.1)
        }
    }

    pub struct Walk {
        pub visited: HashMap<XY, usize>,
        visit_order: BinaryHeap<Visitable>,
        pub hillmap: HillMap,
    }

    impl Walk {
        pub fn new(hillmap: HillMap) -> Self {
            Self {
                visited: Default::default(),
                visit_order: BinaryHeap::new(),
                hillmap,
            }
        }

        pub fn walk(&mut self) {
            self.visit_order.push(Visitable(self.hillmap.start, 0));
            loop {
                let current = self.visit_order.pop();
                if current.is_none() {
                    break;
                }
                let Visitable(xy, steps) = current.unwrap();
                // println!("Visiting {} at Step {}", xy, steps);

                // self.visited.insert(xy, steps);

                if xy == self.hillmap.end {
                    break;
                }

                let neighbours = self.hillmap.movable_neighbours(&xy);
                let neighbours = neighbours
                    .iter()
                    // ignore XY that we have visited
                    .filter(|(xy, _)| !self.visited.contains_key(xy))
                    .collect::<Vec<_>>();
                neighbours.iter().for_each(|(xy, _)| {
                    self.visit_order.push(Visitable(*xy, steps + 1));
                    self.visited.insert(*xy, steps + 1);
                });
            }
        }
    }
}

pub fn part_one(input: &str) -> eyre::Result<usize> {
    let hillmap = HillMap::from_str(input)?;
    let mut walk = Walk::new(hillmap);
    walk.walk();
    let how_much = walk.visited.get(&walk.hillmap.end);
    Ok(*how_much.unwrap())
}

pub fn part_two(input: &str) -> eyre::Result<usize> {
    let hillmap = HillMap::from_str(input)?;

    let mut min_steps: Option<(XY, usize)> = None;

    for xy in &hillmap.lowest {
        let mut hillmap = hillmap.clone();
        hillmap.start = *xy;

        let mut walk = Walk::new(hillmap);
        walk.walk();
        let how_much = walk.visited.get(&walk.hillmap.end);
        if let Some(h) = how_much {
            match min_steps {
                Some((_ij, m)) => {
                    if *h < m {
                        min_steps = Some((*xy, *h))
                    }
                }
                None => min_steps = Some((*xy, *h)),
            }
        }
    }

    Ok(min_steps.unwrap().1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input).unwrap(), 31);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input).unwrap(), 29);
    }
}
