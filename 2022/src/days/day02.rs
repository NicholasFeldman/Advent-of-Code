use std::str::FromStr;

/// The Elves begin to set up camp on the beach.
/// To decide whose tent gets to be closest to the snack storage,
/// a giant Rock Paper Scissors tournament is already in progress.

/// Rock Paper Scissors is a game between two players.
/// Each game contains many rounds; in each round, the players each simultaneously
/// choose one of Rock, Paper, or Scissors using a hand shape.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Rock, Paper, Scissors
}

/// Then, a winner for that round is selected:
/// Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
/// If both players choose the same shape, the round instead ends in a draw.
#[derive(Clone, Copy, Debug)]
enum Outcome {
    Win, Lose, Draw
}

impl Shape {
    pub fn outcome_against(&self, shape2: Shape) -> Outcome {
        match (self, shape2) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Lose,
            (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }
}

/// Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input)
/// that they say will be sure to help you win.
/// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors.
/// The second column--" Suddenly, the Elf is called away to help with someone's tent.
/// The second column, you reason, must be what you should play in response:
/// X for Rock, Y for Paper, and Z for Scissors.
/// Winning every time would be suspicious, so the responses must have been carefully chosen.
impl FromStr for Shape {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(()),
        }
    }
}

/// The winner of the whole tournament is the player with the highest score.
/// Your total score is the sum of your scores for each round.
/// The score for a single round is the score for the shape you selected
/// (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round
/// (0 if you lost, 3 if the round was a draw, and 6 if you won).
impl Shape {
    pub fn get_points(&self) -> u32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Outcome {
    pub fn get_points(&self) -> u32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

fn score_for_round(shape: Shape, my_shape: Shape) -> u32 {
    my_shape.get_points() + my_shape.outcome_against(shape).get_points()
}

/// The Elf finishes helping with the tent and sneaks back over to you.
/// "Anyway, the second column says how the round needs to end:
/// X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
/// Good luck!"
impl FromStr for Outcome {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn shape_needed(shape: Shape, needed_outcome: Outcome) -> Shape {
    match (shape, needed_outcome) {
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
    }
}

pub fn part_one(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();

        let shape = Shape::from_str(split[0]).unwrap();
        let shape2 = Shape::from_str(split[1]).unwrap();

        score += score_for_round(shape, shape2);
    }

    score
}

pub fn part_two(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let split = line.split(" ").collect::<Vec<_>>();

        let shape = Shape::from_str(split[0]).unwrap();
        let needed_outcome = Outcome::from_str(split[1]).unwrap();

        score += score_for_round(shape, shape_needed(shape, needed_outcome));
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use crate::read_file;
        let input = read_file("examples", "day02.txt");
        assert_eq!(part_one(&input), 15);
    }

#[test]
fn test_part_two() {
        use crate::read_file;
        let input = read_file("examples", "day02.txt");
        assert_eq!(part_two(&input), 12);
    }
}