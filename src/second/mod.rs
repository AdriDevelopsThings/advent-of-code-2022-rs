enum Winning {
    Win,
    Loose,
    Draw
}

impl Winning {
    fn new(input: &str) -> Self { // only for part two
        match input {
            "X" => Self::Loose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("invalid winning type")
        }
    }

    fn score(&self) -> u8 {
        match self {
            Self::Loose => 0,
            Self::Draw => 3,
            Self::Win => 6
        }
    }
}

#[derive(PartialEq)]
enum Choose {
    Rock,
    Paper,
    Scissors
}

impl Choose {
    fn new(input: &str) -> Self {
        match input {
            "A" => Choose::Rock,
            "B" => Choose::Paper,
            "C" => Choose::Scissors,
            "X" => Choose::Rock,
            "Y" => Choose::Paper,
            "Z" => Choose::Scissors,
            _ => panic!("Invalid choose type") 
        }
    }

    fn generate_by_win(other: &Self, win: &Winning) -> Self {
        match other {
            Self::Rock => match win {
                Winning::Win => Self::Paper,
                Winning::Loose => Self::Scissors,
                Winning::Draw => Self::Rock,
            },
            Self::Paper => match win {
                Winning::Win => Self::Scissors,
                Winning::Loose => Self::Rock,
                Winning::Draw => Self::Paper,
            },
            Self::Scissors => match win {
                Winning::Win => Self::Rock,
                Winning::Loose => Self::Paper,
                Winning::Draw => Self::Scissors,
            },
        }
    }

    fn score(&self) -> u8 {
        match self {
            Choose::Rock => 1,
            Choose::Paper => 2,
            Choose::Scissors => 3,
        }
    }

    fn win(&self, other: &Self) -> Winning {
        match self {
            Self::Rock => match other {
                Self::Paper => Winning::Loose,
                Self::Scissors => Winning::Win,
                Self::Rock => Winning::Draw
            },
            Self::Paper => match other {
                Self::Rock => Winning::Win,
                Self::Scissors => Winning::Loose,
                Self::Paper => Winning::Draw
            },
            Self::Scissors => match other {
                Self::Rock => Winning::Loose,
                Self::Paper => Winning::Win,
                Self::Scissors => Winning::Draw
            }
        }
    }
}

struct RoundV1 {
    opponent_choose: Choose,
    your_choose: Choose
}

impl RoundV1 {
    fn new(input: &str) -> Self {
        let splitted: Vec<&str> = input.split(' ').collect();
        RoundV1 { opponent_choose: Choose::new(splitted[0]), your_choose: Choose::new(splitted[1]) }
    }

    fn score(&self) -> u16 {
        u16::from(self.your_choose.score()) + u16::from(self.your_choose.win(&self.opponent_choose).score())
    }
}

pub fn solve_first(input: String) -> String {
    input.lines().map(RoundV1::new).collect::<Vec<RoundV1>>().into_iter().map(|round| round.score()).sum::<u16>().to_string()

}

struct RoundV2 {
    opponent_choose: Choose,
    winning: Winning
}

impl RoundV2 {
    fn new(input: &str) -> Self {
        let splitted: Vec<&str> = input.split(' ').collect();
        RoundV2 { opponent_choose: Choose::new(splitted[0]), winning: Winning::new(splitted[1]) }
    }

    fn score(&self) -> u16 {
        u16::from(Choose::generate_by_win(&self.opponent_choose, &self.winning).score()) + u16::from(self.winning.score())

    }
}

pub fn solve_second(input: String) -> String {
    input.lines().map(RoundV2::new).collect::<Vec<RoundV2>>().into_iter().map(|round| round.score()).sum::<u16>().to_string()
}