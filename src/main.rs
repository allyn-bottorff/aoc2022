use std::collections::HashMap;
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

struct CraneMoves {
    quantity: Vec<usize>,
    source: Vec<usize>,
    destination: Vec<usize>,
}

/// Read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Get top calories.
/// Implements day1 part 1 and part 2 solutions
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

/// Play the rock paper scissors game using day 2 part 1 rules.
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

/// Play the rock paper scissors game using day 2 part 2 rules.
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

/// Pick a winning choice based on the opponent's choice
fn rps_pick_win(c1: &RPSChoice) -> RPSChoice {
    let win = match c1 {
        RPSChoice::Rock => RPSChoice::Paper,
        RPSChoice::Paper => RPSChoice::Scissors,
        RPSChoice::Scissors => RPSChoice::Rock,
    };
    win
}

/// Pick a losing choice based on the opponent's choice
fn rps_pick_lose(c1: &RPSChoice) -> RPSChoice {
    let lose = match c1 {
        RPSChoice::Rock => RPSChoice::Scissors,
        RPSChoice::Paper => RPSChoice::Rock,
        RPSChoice::Scissors => RPSChoice::Paper,
    };
    lose
}

/// Calculate the score of a Rock Paper Scissors game.
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

/// Find the misplaced item and return of the sum of the priorities
fn rucksack_sorting(files: &str) -> i32 {
    let priorities = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let mut pri_sum: i32 = 0;
    if let Ok(lines) = read_lines(files) {
        'outer: for line in lines {
            if let Ok(bag_str) = line {
                let bag_size = bag_str.as_str().chars().count();
                let half = bag_size / 2;
                let first_half = &bag_str.as_str()[0..half];
                let second_half = &bag_str.as_str()[half..];
                for c1 in first_half.chars() {
                    for c2 in second_half.chars() {
                        if c1 == c2 {
                            // println!("match found {}", c1);
                            pri_sum += priorities.get(&c1).unwrap();
                            continue 'outer;
                        }
                    }
                }
                println!("match not found in {}", bag_str);
            }
        }
    }

    println!("Sum of the priorities of misplaced items: {}", pri_sum);
    return pri_sum;
}

fn rucksack_badge_search(files: &str) -> i32 {
    let priorities = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52),
    ]);

    let mut pri_sum: i32 = 0;
    let mut line_strings: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(files) {
        for line in lines {
            let line_string = line.unwrap();
            line_strings.push(line_string);
        }
    }
    let mut i: usize = 0;
    for line in &line_strings[..] {
        if i % 3 == 2 {
            'outer: for c1 in line.as_str().clone().chars() {
                for c2 in line_strings[i - 1].as_str().clone().chars() {
                    for c3 in line_strings[i - 2].as_str().clone().chars() {
                        if c1 == c2 && c2 == c3 {
                            pri_sum += priorities.get(&c1).unwrap();
                            break 'outer;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    println!("Sum of the priorities of misplaced items: {}", pri_sum);
    return pri_sum;
}

fn find_complete_overlaps(file: &str) -> i32 {
    let mut overlaps: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            let line_str = line.unwrap();
            let (left, right) = line_str.as_str().split_once(',').unwrap();
            let (s1, e1) = left.split_once('-').unwrap();
            let (s2, e2) = right.split_once('-').unwrap();
            let s1_int = s1.parse::<i32>().unwrap();
            let e1_int = e1.parse::<i32>().unwrap();
            let s2_int = s2.parse::<i32>().unwrap();
            let e2_int = e2.parse::<i32>().unwrap();

            if s1_int <= s2_int && e1_int >= e2_int {
                overlaps += 1;
            } else if s1_int >= s2_int && e1_int <= e2_int {
                overlaps += 1;
            }
        }
    }

    println!("Overlapping job pairs: {}", overlaps);
    return overlaps;
}

fn find_partial_overlaps(file: &str) -> i32 {
    let mut overlaps: i32 = 0;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            let line_str = line.unwrap();
            let (left, right) = line_str.as_str().split_once(',').unwrap();
            let (s1, e1) = left.split_once('-').unwrap();
            let (s2, e2) = right.split_once('-').unwrap();
            let s1_int = s1.parse::<i32>().unwrap();
            let e1_int = e1.parse::<i32>().unwrap();
            let s2_int = s2.parse::<i32>().unwrap();
            let e2_int = e2.parse::<i32>().unwrap();

            if s1_int <= s2_int && e1_int >= s2_int {
                overlaps += 1;
                continue;
            }
            if s1_int >= s2_int && s1_int <= e2_int {
                overlaps += 1;
                continue;
            }
        }
    }

    println!("Overlapping job pairs: {}", overlaps);
    return overlaps;
}

fn read_crane_data(file: &str) -> (Vec<String>, Vec<String>) {
    let mut state_lines: Vec<String> = Vec::new();
    let mut proc_lines: Vec<String> = Vec::new();
    let mut end_of_state: bool = false;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            let line_string = line.unwrap();
            if line_string == String::from("") {
                end_of_state = true;
                continue;
            }
            if !end_of_state {
                state_lines.push(line_string);
            } else {
                proc_lines.push(line_string);
            }
        }
    }

    // println!("state: {:?}", state_lines);
    // println!("procedure: {:?}", proc_lines);

    (state_lines, proc_lines)
}

fn parse_crane_state(state_lines: Vec<String>) -> Vec<Vec<char>> {
    let last_string = state_lines.last().unwrap();
    let last_str = last_string.as_str();
    let trimmed = last_str.trim();
    let last_char = String::from(trimmed).pop().unwrap();
    let num_of_piles: usize = last_char.to_digit(10).unwrap().try_into().unwrap();

    let piles: Vec<Vec<char>> = Vec::with_capacity(num_of_piles);

    piles
}

fn parse_crane_proc(proc_lines: Vec<String>) -> CraneMoves {
    let program = CraneMoves {
        quantity: Vec::new(),
        source: Vec::new(),
        destination: Vec::new(),
    };

    program
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let selection = &args[1];

        match selection.as_str() {
            "d1" => _ = get_top_calories("./data/day1.txt"),
            "d2" => _ = rock_paper_scissors("./data/day2.txt"),
            "d2-2" => _ = rock_paper_scissors2("./data/day2.txt"),
            "d3" => _ = rucksack_sorting("./data/day3.txt"),
            "d3-2" => _ = rucksack_badge_search("./data/day3.txt"),
            "d4" => _ = find_complete_overlaps("./data/day4.txt"),
            "d4-2" => _ = find_partial_overlaps("./data/day4.txt"),
            "d5" => _ = read_crane_data("./data/day5.txt"),
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
        let score = rock_paper_scissors("./tests/day2.txt");

        assert_eq!(score, 15)
    }
    #[test]
    fn test_day2_2() {
        let score = rock_paper_scissors2("./tests/day2.txt");

        assert_eq!(score, 12)
    }

    #[test]
    fn test_day3_1() {
        let priority_sum = rucksack_sorting("./tests/day3.txt");

        assert_eq!(priority_sum, 157)
    }
    #[test]
    fn test_day3_2() {
        let priority_sum = rucksack_badge_search("./tests/day3.txt");

        assert_eq!(priority_sum, 70)
    }
    #[test]
    fn test_day4_1() {
        let overlaps = find_complete_overlaps("./tests/day4.txt");
        assert_eq!(overlaps, 2)
    }
    #[test]
    fn test_day4_2() {
        let overlaps = find_partial_overlaps("./tests/day4.txt");
        assert_eq!(overlaps, 4)
    }
    #[test]
    fn test_day5_parse() {
        let (conf, proc) = read_crane_data("./tests/day5.txt");
        assert_eq!(conf.len(), 4);
        assert_eq!(proc.len(), 4);
    }
}
