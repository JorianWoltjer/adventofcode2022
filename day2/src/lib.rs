use strum::{EnumIter, IntoEnumIterator};
use std::cmp::Ordering;
use std::error::Error;
use std::str::FromStr;

#[derive(PartialEq, EnumIter, Clone)]
pub enum HandShape {
    ROCK,
    PAPER,
    SCISSORS,
}

impl PartialOrd for HandShape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            HandShape::ROCK => match other {
                HandShape::ROCK => Some(Ordering::Equal),
                HandShape::PAPER => Some(Ordering::Less),
                HandShape::SCISSORS => Some(Ordering::Greater),
            }
            HandShape::PAPER => match other {
                HandShape::ROCK => Some(Ordering::Greater),
                HandShape::PAPER => Some(Ordering::Equal),
                HandShape::SCISSORS => Some(Ordering::Less),
            }
            HandShape::SCISSORS =>  match other {
                HandShape::ROCK => Some(Ordering::Less),
                HandShape::PAPER => Some(Ordering::Greater),
                HandShape::SCISSORS => Some(Ordering::Equal),
            }
        }
    }
}

impl HandShape {
    pub fn points(&self) -> i64 {
        match self {
            HandShape::ROCK => 1,
            HandShape::PAPER => 2,
            HandShape::SCISSORS => 3,
        }
    }
}

impl FromStr for HandShape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(HandShape::ROCK),
            "B" | "Y" => Ok(HandShape::PAPER),
            "C" | "Z" => Ok(HandShape::SCISSORS),
            _ => Err(format!("{:?} is not in ABCXYZ", s))
        }
    }
}

pub struct Round {
    pub opponent_move: HandShape,
    pub player_move: HandShape,
}

impl Round {
    pub fn outcome(&self) -> Outcomes {
        if self.opponent_move == self.player_move {
            Outcomes::DRAW
        } else if self.opponent_move > self.player_move {
            Outcomes::LOSE
        } else if self.player_move > self.opponent_move {
            Outcomes::WIN
        } else {
            unreachable!();
        }
    }
}

impl FromStr for Round {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(" ");
        let opponent_move: HandShape = s.next().ok_or("No first move")?.parse()?;
        let player_move: HandShape = s.next().ok_or("No second move")?.parse()?;

        Ok(Self { opponent_move, player_move })
    }
}

#[derive(PartialEq)]
pub enum Outcomes {
    DRAW,
    WIN,
    LOSE,
}

impl Outcomes {
    pub fn get(&self) -> Outcome {
        match self {
            Outcomes::DRAW => Outcome { opponent_points: 3, player_points: 3},
            Outcomes::WIN => Outcome { opponent_points: 0, player_points: 6 },
            Outcomes::LOSE => Outcome { opponent_points: 6, player_points: 0 },
        }
    }
}

impl FromStr for Outcomes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcomes::LOSE),
            "Y" => Ok(Outcomes::DRAW),
            "Z" => Ok(Outcomes::WIN),
            _ => Err(format!("{:?} is not X, Y or Z", s))
        }
    }
}

#[derive(Debug)]
pub struct Outcome {
    pub opponent_points: i64,
    pub player_points: i64,
}

pub fn parse_line(s: &str) -> Result<(HandShape, Outcomes), Box<dyn Error>> {
    let mut s = s.split(" ");
    let opponent_move: HandShape = s.next().ok_or("No first move")?.parse()?;
    let desired_outcome: Outcomes = s.next().ok_or("No outcome")?.parse()?;

    Ok((opponent_move, desired_outcome))
}

pub fn find_move_for(opponent_move: HandShape, desired_outcome: Outcomes) -> Option<Round> {
    for possible_move in HandShape::iter() {
        let round = Round { opponent_move: opponent_move.clone(), player_move: possible_move };

        if round.outcome() == desired_outcome {
            return Some(round);
        }
    }

    None
}
