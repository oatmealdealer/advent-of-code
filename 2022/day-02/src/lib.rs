use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum Throw {
    Rock,
    Paper,
    Scissors,
}

impl Throw {
    pub const fn base_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl From<&str> for Throw {
    fn from(letter: &str) -> Self {
        match letter {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => unreachable!()
        }
    }
}

impl Ord for Throw {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        }
    }
}

impl PartialOrd for Throw {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Match(Throw, Throw);

pub enum Outcome {
    Win,
    Loss,
    Draw
}

impl Outcome {
    pub fn throw_for(&self, throw: &Throw) -> Throw {
        match self {
            Self::Win => match throw {
                Throw::Rock => Throw::Paper,
                Throw::Paper => Throw::Scissors,
                Throw::Scissors => Throw::Rock

            },
            Self::Loss => match throw {
                Throw::Rock => Throw::Scissors,
                Throw::Paper => Throw::Rock,
                Throw::Scissors => Throw::Paper

            },
            Self::Draw => *throw
        }
    }
}

impl From<&str> for Outcome {
    fn from(letter: &str) -> Self {
        match letter {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!()
        }
    }
}

impl Match {
    pub fn score(&self) -> u32 {
        self.1.base_score()
            + match self.1.cmp(&self.0) {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            }
    }
}

pub fn process_part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut letters = line.split(" ");
            let parsed_match = Match(letters.next().unwrap().into(), letters.next().unwrap().into());
            parsed_match.score()
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut letters = line.split(" ");
            let opponent_throw = Throw::from(letters.next().unwrap());
            let desired_throw = Outcome::from(letters.next().unwrap()).throw_for(&opponent_throw);
            let parsed_match = Match(opponent_throw, desired_throw);
            parsed_match.score()
        })
        .sum::<u32>()
        .to_string()
}
