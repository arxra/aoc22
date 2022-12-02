#[derive(Debug, Clone)]
enum Outcome {
    Draw,
    Loss,
    Win,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::Draw => 3,
            Outcome::Loss => 0,
            Outcome::Win => 6,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Move {
    Rock,
    Paper,
    Siccor,
}

impl Move {
    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Siccor => 3,
            _ => unreachable!("not supported playoption"),
        }
    }
    fn desired_move(c1: &Move, c2: char) -> Self {
        match (c1, c2) {
            (Self::Rock, 'Y') | (Self::Paper, 'X') | (Self::Siccor, 'Z') => Self::Rock,
            (Self::Rock, 'Z') | (Self::Paper, 'Y') | (Self::Siccor, 'X') => Self::Paper,
            (Self::Rock, 'X') | (Self::Paper, 'Z') | (Self::Siccor, 'Y') => Self::Siccor,
            _ => unimplemented!(),
        }
    }
}
impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Siccor,
            _ => unreachable!("not supported playoption"),
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    p1: Move,
    p2: Move,
}

impl Game {
    fn score(&self) -> usize {
        self.player_winner().score() + self.p2.value()
    }

    fn player_winner(&self) -> Outcome {
        match (&self.p1, &self.p2) {
            (a, b) if a == b => Outcome::Draw,
            (Move::Rock, Move::Paper)
            | (Move::Paper, Move::Siccor)
            | (Move::Siccor, Move::Rock) => Outcome::Win,
            (Move::Rock, Move::Siccor)
            | (Move::Paper, Move::Rock)
            | (Move::Siccor, Move::Paper) => Outcome::Loss,
            _ => unimplemented!(),
        }
    }
}

fn solve_p1(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|f| {
            let mut chars = f.chars();
            let p1 = chars.next().unwrap().into();
            let p2 = chars.nth(1).unwrap().into();
            Game { p1, p2 }
        })
        .map(|f| f.score())
        .sum()
}

fn solve_p2(file_name: &str) -> usize {
    std::fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|f| {
            let mut chars = f.chars();
            let p1 = chars.next().unwrap().into();
            let p2 = Move::desired_move(&p1, chars.nth(1).unwrap());
            Game { p1, p2 }
        })
        .map(|f| f.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1p1_exp() {
        let expected = 15;
        let result = solve_p1("small/02.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn d1p2_exp() {
        let expected = 12;
        let result = solve_p2("small/02.txt");
        assert_eq!(result, expected);
    }
    #[test]
    fn d1p1() {
        let result = solve_p1("input/02.txt");
        println!("d02p1: {result}");
    }
    #[test]
    fn d1p2() {
        let result = solve_p2("input/02.txt");
        println!("d02p2: {result}");
    }
}
