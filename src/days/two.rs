pub fn solution_easy(input: &str) -> i32 {
    input
        .lines()
        .map(|l| chars_to_easy(l.chars().filter(|c| !c.is_whitespace()).collect()))
        .sum()
}

pub fn solution_hard(input: &str) -> i32 {
    input
        .lines()
        .map(|l| chars_to_hard(l.chars().filter(|c| !c.is_whitespace()).collect()))
        .sum()
}

pub fn chars_to_easy(line: Vec<char>) -> i32 {
    let player = char_to_move(line[1]).unwrap();
    let other = char_to_move(line[0]).unwrap();
    player.score() + player.play(other).score()
}

pub fn chars_to_hard(line: Vec<char>) -> i32 {
    let result = char_to_result(line[1]).unwrap();
    let other = char_to_move(line[0]).unwrap();
    let move_to_play = other.move_for_result(&result);
    move_to_play.score() + move_to_play.play(other).score()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

fn char_to_move(c: char) -> Option<Move> {
    match c {
        'A' | 'X' => Some(Move::Rock),
        'B' | 'Y' => Some(Move::Paper),
        'C' | 'Z' => Some(Move::Scissor),
        _ => None,
    }
}

impl Move {
    fn next(&self) -> Self {
        use Move::*;
        match *self {
            Rock => Paper,
            Paper => Scissor,
            Scissor => Rock,
        }
    }

    fn previous(&self) -> Self {
        use Move::*;
        match *self {
            Rock => Scissor,
            Paper => Rock,
            Scissor => Paper,
        }
    }

    fn score(&self) -> i32 {
        use Move::*;
        match *self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }

    fn play(&self, other: Move) -> Result {
        if *self == other {
            Result::Draw
        } else if self.next() == other {
            Result::Loss
        } else {
            Result::Win
        }
    }

    fn move_for_result(&self, result: &Result) -> Move {
        match result {
            Result::Loss => self.previous(),
            Result::Draw => *self,
            Result::Win => self.next(),
        }
    }
}

fn char_to_result(c: char) -> Option<Result> {
    match c {
        'X' => Some(Result::Loss),
        'Y' => Some(Result::Draw),
        'Z' => Some(Result::Win),
        _ => None,
    }
}

enum Result {
    Loss,
    Draw,
    Win,
}

impl Result {
    fn score(&self) -> i32 {
        use self::Result::*;
        match *self {
            Loss => 0,
            Draw => 3,
            Win => 6,
        }
    }
}
