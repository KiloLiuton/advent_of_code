use std::fs;

const SCORE_ROCK: i32 = 1;
const SCORE_PAPER: i32 = 2;
const SCORE_SCISSOR: i32 = 3;

const SCORE_LOSS: i32 = 0;
const SCORE_TIE: i32 = 3;
const SCORE_WIN: i32 = 6;

struct Game {
    left: String,
    right: String,
}

impl Game {
    fn score(&self) -> i32 {
        match self.right.as_str() {
            "A" => {
                SCORE_ROCK
                    + match self.left.as_str() {
                        "A" => SCORE_TIE,
                        "B" => SCORE_LOSS,
                        "C" => SCORE_WIN,
                        _ => panic!("Hand not recognized!"),
                    }
            }
            "B" => {
                SCORE_PAPER
                    + match self.left.as_str() {
                        "A" => SCORE_WIN,
                        "B" => SCORE_TIE,
                        "C" => SCORE_LOSS,
                        _ => panic!("Hand not recognized!"),
                    }
            }
            "C" => {
                SCORE_SCISSOR
                    + match self.left.as_str() {
                        "A" => SCORE_LOSS,
                        "B" => SCORE_WIN,
                        "C" => SCORE_TIE,
                        _ => panic!("Hand not recognized!"),
                    }
            }
            _ => panic!("Hand not recognized!"),
        }
    }
}

fn decode_strategy(strat: &mut Vec<Game>) {
    for i in 0..strat.len() {
        let game = &mut strat[i];
        match game.right.as_str() {
            // loss
            "A" => {
                strat[i] = Game {
                    left: game.left.clone(),
                    right: match game.left.as_str() {
                        "A" => "C".to_string(),
                        "B" => "A".to_string(),
                        "C" => "B".to_string(),
                        _ => panic!("Hand not recognized!"),
                    },
                }
            }
            // draw
            "B" => {
                strat[i] = Game {
                    left: game.left.clone(),
                    right: game.left.clone(),
                }
            }
            // win
            "C" => {
                strat[i] = Game {
                    left: game.left.clone(),
                    right: match game.left.as_str() {
                        "A" => "B".to_string(),
                        "B" => "C".to_string(),
                        "C" => "A".to_string(),
                        _ => panic!("Hand not recognized!"),
                    },
                }
            }
            _ => panic!("Hand not recognized!"),
        }
    }
}

pub fn main() -> Result<(), String> {
    let strategy = fs::read_to_string("inputs/day2").expect("Can't read input file!");

    let mut strategy: Vec<Game> = strategy
        .split_terminator("\n")
        .map(|line| {
            let mut it = line.split(" ");
            let left = it.next().expect("Failed to parse left side!").to_string();
            let right = it.next().expect("Failed to parse right side!");
            let right = match right {
                "X" => "A".to_string(),
                "Y" => "B".to_string(),
                "Z" => "C".to_string(),
                _ => panic!("Failed to decode hand!"),
            };
            Game { left, right }
        })
        .collect();

    let mut score = 0;
    for game in strategy.iter() {
        score += game.score();
    }
    println!("Part 1:\nStrategy score is: {}", score);

    decode_strategy(&mut strategy);
    score = 0;
    for game in strategy.iter() {
        score += game.score();
    }
    println!("Part 2:\nDecoded strategy score is: {}", score);

    Ok(())
}
