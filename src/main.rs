use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone)]
enum RPSChoice {
    Rock,
    Paper,
    Scissors,
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_top_calories(file: &str) {
    let mut current_calories: i32 = 0;
    let mut max_cal_1: i32 = 0;
    let mut max_cal_2: i32 = 0;
    let mut max_cal_3: i32 = 0;

    // let mut current_elf: i32 = 1;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(cal_str) = line {
                let cal = match cal_str.parse::<i32>() {
                    Ok(cal) => cal,
                    Err(_) => {
                        // current_elf += 1;
                        if current_calories > max_cal_1 {
                            max_cal_3 = max_cal_2;
                            max_cal_2 = max_cal_1;
                            max_cal_1 = current_calories;
                        } else if current_calories > max_cal_2 {
                            max_cal_2 = current_calories;
                        } else if current_calories > max_cal_3 {
                            max_cal_3 = current_calories;
                        }
                        current_calories = 0;
                        continue;
                    }
                };

                current_calories += cal;
            }
        }
    }

    let top_3_sum = max_cal_1 + max_cal_2 + max_cal_3;
    println!("Max Calories: {}", max_cal_1);
    println!(
        "Top 3 calorie counts: {} {} {}",
        max_cal_1, max_cal_2, max_cal_3
    );
    println!("Sum of top 3: {}", top_3_sum);
}

fn rock_paper_scissors(file: &str) -> i32 {
    let mut score: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_string) = line {
                let opp_char = line_string.as_str().chars().nth(0).unwrap();
                let you_char = line_string.as_str().chars().nth(2).unwrap();

                let opp_choice = match opp_char {
                    'A' => RPSChoice::Rock,
                    'B' => RPSChoice::Paper,
                    'C' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                let your_choice = match you_char {
                    'X' => RPSChoice::Rock,
                    'Y' => RPSChoice::Paper,
                    'Z' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                score += score_rps(opp_choice, your_choice);
            }
        }
    }

    println!("Your score: {}", score);
    score
}

fn rock_paper_scissors2(file: &str) -> i32 {
    let mut score: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(line_string) = line {
                // Using unwrap() here because if this fails, it' better to
                // crash.
                let opp_char = line_string.as_str().chars().nth(0).unwrap();
                let you_char = line_string.as_str().chars().nth(2).unwrap();

                let opp_choice = match opp_char {
                    'A' => RPSChoice::Rock,
                    'B' => RPSChoice::Paper,
                    'C' => RPSChoice::Scissors,
                    _ => panic!("Failed to parse move"),
                };
                let your_choice = match you_char {
                    'X' => rps_pick_lose(&opp_choice),
                    'Y' => opp_choice,
                    'Z' => rps_pick_win(&opp_choice),
                    _ => panic!("Failed to parse move"),
                };
                score += score_rps(opp_choice, your_choice);
            }
        }
    }

    println!("Your score: {}", score);
    score
}
fn rps_pick_win(c1: &RPSChoice) -> RPSChoice {
    let win = match c1 {
        RPSChoice::Rock => RPSChoice::Paper,
        RPSChoice::Paper => RPSChoice::Scissors,
        RPSChoice::Scissors => RPSChoice::Rock,
    };
    win
}

fn rps_pick_lose(c1: &RPSChoice) -> RPSChoice {
    let lose = match c1 {
        RPSChoice::Rock => RPSChoice::Scissors,
        RPSChoice::Paper => RPSChoice::Rock,
        RPSChoice::Scissors => RPSChoice::Paper,
    };
    lose
}

fn score_rps(c1: RPSChoice, c2: RPSChoice) -> i32 {
    let score = match (c1, c2) {
        (RPSChoice::Rock, RPSChoice::Rock) => 1 + 3,
        (RPSChoice::Rock, RPSChoice::Paper) => 2 + 6,
        (RPSChoice::Rock, RPSChoice::Scissors) => 3 + 0,

        (RPSChoice::Paper, RPSChoice::Rock) => 1 + 0,
        (RPSChoice::Paper, RPSChoice::Paper) => 2 + 3,
        (RPSChoice::Paper, RPSChoice::Scissors) => 3 + 6,

        (RPSChoice::Scissors, RPSChoice::Rock) => 1 + 6,
        (RPSChoice::Scissors, RPSChoice::Paper) => 2 + 0,
        (RPSChoice::Scissors, RPSChoice::Scissors) => 3 + 3,
    };

    score
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let selection = &args[1];

        match selection.as_str() {
            "d1" => _ = get_top_calories("./day1.txt"),
            "d2" => _ = rock_paper_scissors("./day2.txt"),
            "d2-2" => _ = rock_paper_scissors2("./day2.txt"),
            _ => println!("Unrecognized command."),
        }
    } else {
        println!("Pick a day to run. e.g. \"d1\"");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_1() {
        let score = rock_paper_scissors("./day2test.txt");

        assert_eq!(score, 15)
    }
    #[test]
    fn test_day2_2() {
        let score = rock_paper_scissors2("./day2test.txt");

        assert_eq!(score, 12)
    }
}
