#[derive(Debug, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("{value}"),
        }
    }
}

impl Move {
    pub fn get_counter_move_for(&self, outcome: &Outcome) -> Self {
        match outcome {
            &Outcome::Win => match self {
                &Self::Rock => Self::Paper,
                &Self::Paper => Self::Scissors,
                &Self::Scissors => Self::Rock,
            },
            &Outcome::Draw => self.clone(),
            &Outcome::Lose => match self {
                &Self::Rock => Self::Scissors,
                &Self::Paper => Self::Rock,
                &Self::Scissors => Self::Paper,
            },
        }
    }

    pub fn into_score(&self) -> usize {
        match self {
            &Self::Rock => 1,
            &Self::Paper => 2,
            &Self::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl From<char> for Outcome {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("{value}"),
        }
    }
}

impl Outcome {
    pub fn into_score(&self) -> usize {
        match self {
            &Self::Win => 6,
            &Self::Draw => 3,
            &Self::Lose => 0,
        }
    }
}

fn main(input: &str) -> Option<usize> {
    let mut score = 0;
    for mut line in input.lines().map(|line| line.split_whitespace()) {
        let (enemy_move, expected_outcome): (Move, Outcome) = (
            line.next()?.parse::<char>().ok()?.into(),
            line.next()?.parse::<char>().ok()?.into(),
        );
        let player_move = enemy_move.get_counter_move_for(&expected_outcome);

        score += player_move.into_score() + expected_outcome.into_score();
    }

    Some(score)
}

#[cfg(test)]
mod test {
    use super::main;

    #[test]
    pub fn two() {
        assert_eq!(Some(11998), main(std::include_str!("./two.txt")));
    }
}
