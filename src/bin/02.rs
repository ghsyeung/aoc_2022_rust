#[derive(PartialEq, Eq, Copy, Clone)]
enum PlayType {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

const fn cheat(opponent: PlayType, outcome: Outcome) -> PlayType {
    match (opponent, outcome) {
        (PlayType::Rock, Outcome::Win) => PlayType::Paper,
        (PlayType::Rock, Outcome::Draw) => PlayType::Rock,
        (PlayType::Rock, Outcome::Lose) => PlayType::Scissors,
        (PlayType::Paper, Outcome::Win) => PlayType::Scissors,
        (PlayType::Paper, Outcome::Draw) => PlayType::Paper,
        (PlayType::Paper, Outcome::Lose) => PlayType::Rock,
        (PlayType::Scissors, Outcome::Win) => PlayType::Rock,
        (PlayType::Scissors, Outcome::Draw) => PlayType::Scissors,
        (PlayType::Scissors, Outcome::Lose) => PlayType::Paper,
    }
}

const fn play(opponent: PlayType, mine: PlayType) -> Outcome {
    match (opponent, mine) {
        (PlayType::Rock, PlayType::Paper) => Outcome::Win,
        (PlayType::Rock, PlayType::Scissors) => Outcome::Lose,
        (PlayType::Paper, PlayType::Scissors) => Outcome::Win,
        (PlayType::Paper, PlayType::Rock) => Outcome::Lose,
        (PlayType::Scissors, PlayType::Paper) => Outcome::Lose,
        (PlayType::Scissors, PlayType::Rock) => Outcome::Win,
        (PlayType::Rock, PlayType::Rock) => Outcome::Draw,
        (PlayType::Paper, PlayType::Paper) => Outcome::Draw,
        (PlayType::Scissors, PlayType::Scissors) => Outcome::Draw,
    }
}

const fn score_of_play(p: PlayType) -> u32 {
    match p {
        PlayType::Rock => 1,
        PlayType::Paper => 2,
        PlayType::Scissors => 3,
    }
}

const fn score_of_outcome(p: Outcome) -> u32 {
    match p {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    }
}

const fn score_of_round(opponent: PlayType, mine: PlayType) -> u32 {
    score_of_play(mine) + score_of_outcome(play(opponent, mine))
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

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            let mut it = line.split_whitespace().into_iter();
            let opponent = parse_opponent(it.next().unwrap());
            let mine = parse_mine(it.next().unwrap());
            total_score += score_of_round(opponent, mine);
        }
    }
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score: u32 = 0;
    for line in input.split("\n") {
        if !line.trim().is_empty() {
            let mut it = line.split_whitespace().into_iter();
            let opponent = parse_opponent(it.next().unwrap());
            let outcome = parse_outcome(it.next().unwrap());
            let mine = cheat(opponent, outcome);
            total_score += score_of_round(opponent, mine);
        }
    }
    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
