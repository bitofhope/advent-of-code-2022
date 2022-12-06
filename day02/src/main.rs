use std::env;
use std::io::BufRead;

const WIN: u64 = 6;
const TIE: u64 = 3;
const LOSS: u64 = 0;
const ROCK_SCORE: u64 = 1;
const PAPER_SCORE: u64 = 2;
const SCISSORS_SCORE: u64 = 3;

const OPPONENT_ROCK: &str = "A";
const OPPONENT_PAPER: &str = "B";
const OPPONENT_SCISSORS: &str = "C";

const YOU_ROCK: &str = "X";
const YOU_PAPER: &str = "Y";
const YOU_SCISSORS: &str = "Z";

const STRAT_LOSE: &str = "X";
const STRAT_DRAW: &str = "Y";
const STRAT_WIN: &str = "Z";

fn rps_match_1(opponent: &str, you: &str) -> u64 {
    match (opponent, you) {
        (OPPONENT_ROCK, YOU_ROCK) => TIE + ROCK_SCORE,
        (OPPONENT_ROCK, YOU_PAPER) => WIN + PAPER_SCORE,
        (OPPONENT_ROCK, YOU_SCISSORS) => LOSS + SCISSORS_SCORE,
        (OPPONENT_PAPER, YOU_ROCK) => LOSS + ROCK_SCORE,
        (OPPONENT_PAPER, YOU_PAPER) => TIE + PAPER_SCORE,
        (OPPONENT_PAPER, YOU_SCISSORS) => WIN + SCISSORS_SCORE,
        (OPPONENT_SCISSORS, YOU_ROCK) => WIN + ROCK_SCORE,
        (OPPONENT_SCISSORS, YOU_PAPER) => LOSS + PAPER_SCORE,
        (OPPONENT_SCISSORS, YOU_SCISSORS) => TIE + SCISSORS_SCORE,
        _ => panic!("Invalid throws: {:?}", (opponent, you)),
    }
}

fn rps_match_2(opponent: &str, strategy: &str) -> u64 {
    match (opponent, strategy) {
        (OPPONENT_ROCK, STRAT_LOSE) => LOSS + SCISSORS_SCORE,
        (OPPONENT_ROCK, STRAT_DRAW) => TIE + ROCK_SCORE,
        (OPPONENT_ROCK, STRAT_WIN) => WIN + PAPER_SCORE,
        (OPPONENT_PAPER, STRAT_LOSE) => LOSS + ROCK_SCORE,
        (OPPONENT_PAPER, STRAT_DRAW) => TIE + PAPER_SCORE,
        (OPPONENT_PAPER, STRAT_WIN) => WIN + SCISSORS_SCORE,
        (OPPONENT_SCISSORS, STRAT_LOSE) => LOSS + PAPER_SCORE,
        (OPPONENT_SCISSORS, STRAT_DRAW) => TIE + SCISSORS_SCORE,
        (OPPONENT_SCISSORS, STRAT_WIN) => WIN + ROCK_SCORE,
        _ => panic!("Invalid throws: {:?}", (opponent, strategy)),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let file = std::fs::File::open(file_path).unwrap();
    let lines = std::io::BufReader::new(file).lines();

    let mut matches = Vec::new();

    for line in lines {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let a = Box::from(words.next().unwrap());
        let b = Box::from(words.next().unwrap());
        matches.push((a, b));
    }
    part_1(matches.clone());
    part_2(matches);
}

fn part_1(matches: Vec<(Box<str>, Box<str>)>) {
    let mut total = 0u64;
    for (opp, you) in matches.iter() {
        total += rps_match_1(opp, you);
    }
    println!("{total}");
}

fn part_2(matches: Vec<(Box<str>, Box<str>)>) {
    let mut total = 0u64;
    for (opp, strat) in matches.iter() {
        total += rps_match_2(opp, strat);
    }
    println!("{total}");
}
