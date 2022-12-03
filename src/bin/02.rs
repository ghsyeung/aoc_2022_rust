use std::str::FromStr;

use eyre::eyre;

#[derive(PartialEq, Eq, Copy, Clone)]
enum PlayType {
    Rock,
    Paper,
    Scissors,
}

fn parse_opponent(s: &str) -> PlayType {
    match s {
        "A" => PlayType::Rock,
        "B" => PlayType::Paper,
        "C" => PlayType::Scissors,
        _ => panic!("Invalid opponent play type"),
    }
}

fn parse_mine(s: &str) -> PlayType {
    match s {
        "X" => PlayType::Rock,
        "Y" => PlayType::Paper,
        "Z" => PlayType::Scissors,
        _ => panic!("Invalid mine play type"),
    }
}

fn parse_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Invalid outcome type"),
    }
}

impl PlayType {
    pub fn win_against(p: &Self) -> Self {
        match p {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn play(&self, opponent: &Self) -> Outcome {
        if Self::win_against(opponent) == *self {
            Outcome::Win
        } else if Self::win_against(self) == *opponent {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            PlayType::Rock => 1,
            PlayType::Paper => 2,
            PlayType::Scissors => 3,
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    pub fn cheat(&self, opponent: &PlayType) -> PlayType {
        let win = PlayType::win_against(opponent);
        let lose = PlayType::win_against(&win);
        match self {
            Self::Win => win,
            Self::Draw => *opponent,
            Self::Lose => lose,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

struct CheatRound {
    opponent: PlayType,
    outcome: Outcome,
}

impl CheatRound {
    pub fn get_round(&self) -> Round {
        let mine = self.outcome.cheat(&self.opponent);

        Round {
            mine,
            opponent: self.opponent,
        }
    }
}

impl FromStr for CheatRound {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let opponent = iter
            .next()
            .map(parse_opponent)
            .ok_or(eyre!("Opponent play expected"))?;
        let outcome = iter
            .next()
            .map(parse_outcome)
            .ok_or(eyre!("Outcome expected"))?;
        Ok(CheatRound { outcome, opponent })
    }
}

struct Round {
    mine: PlayType,
    opponent: PlayType,
}

impl Round {
    pub fn score(&self) -> u32 {
        self.mine.score() + self.mine.play(&self.opponent).score()
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        let opponent = iter
            .next()
            .map(parse_opponent)
            .ok_or(eyre!("Opponent play expected"))?;
        let mine = iter
            .next()
            .map(parse_mine)
            .ok_or(eyre!("Mine play expected"))?;
        Ok(Round { mine, opponent })
    }
}

pub fn part_one(input: &str) -> color_eyre::Result<u32> {
    let mut total_score: u32 = 0;
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            let round: Round = Round::from_str(line)?;
            total_score += round.score();
        }
    }
    Ok(total_score)
}

pub fn part_two(input: &str) -> color_eyre::Result<u32> {
    let mut total_score: u32 = 0;
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            let cheat_round: CheatRound = CheatRound::from_str(line)?;
            let round = cheat_round.get_round();
            total_score += round.score();
        }
    }
    Ok(total_score)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input).unwrap(), 15);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input).unwrap(), 12);
    }
}
