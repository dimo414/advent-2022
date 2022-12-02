use std::str::FromStr;

use anyhow::{Result, Error, anyhow};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Choice { Rock, Paper, Scissors }

impl Choice {
    fn score(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl FromStr for Choice {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(anyhow!("Invalid choice: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Outcome { Lose, Draw, Win }

impl FromStr for Outcome {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(anyhow!("Invalid outcome: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    response: Choice,
    outcome: Outcome,
}

impl Round {
    fn score_choice(&self) -> i32 {
        score_round(self.opponent, self.response)
    }

    fn resolve_outcome(&self) -> Choice {
        match self.outcome {
            Outcome::Lose => match self.opponent {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            Outcome::Draw => self.opponent,
            Outcome::Win => match self.opponent {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
        }
    }

    fn score_outcome(&self) -> i32 {
        let resolved = self.resolve_outcome();
        score_round(self.opponent, resolved)
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        debug_assert_eq!(s.len(), 3);
        debug_assert_eq!(&s[1..2], " ");
        let opponent: Choice = s[..1].parse()?;
        let response: Choice = s[2..].parse()?;
        let outcome: Outcome = s[2..].parse()?;
        Ok(Round{ opponent, response, outcome })
    }
}

fn main() -> Result<()> {
    let input = parse_input(include_str!("input.txt"))?;
    let scores: Vec<_> = input.iter().map(|r| r.score_choice()).collect();
    println!("Playing as choices: {}", scores.iter().sum::<i32>());
    let scores: Vec<_> = input.iter().map(|r| r.score_outcome()).collect();
    println!("Playing as outcomes: {}", scores.iter().sum::<i32>());

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Round>> {
    input.lines().map(|r| r.parse::<Round>()).collect()
}

fn score_round(a: Choice, b: Choice) -> i32 {
    b.score() + match (a, b) {
        // win
        (Choice::Rock, Choice::Paper) | (Choice::Paper, Choice::Scissors) | (Choice::Scissors, Choice::Rock) => 6,
        (aa, bb) if aa == bb => 3,  // tie
        _ => 0,  // lose
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let example = parse_input(include_str!("example.txt")).unwrap();
        assert_eq!(example.iter().map(|r| r.score_choice()).sum::<i32>(), 15);
        assert_eq!(example.iter().map(|r| r.score_outcome()).sum::<i32>(), 12);
    }

    parameterized_test::create!{ score, (a, b, score), {
        assert_eq!(score_round(a, b), score);
    } }
    score! {
        win: (Choice::Rock, Choice::Paper, 8),
        loss: (Choice::Paper, Choice::Rock, 1),
        draw: (Choice::Scissors, Choice::Scissors, 6),
    }
}
