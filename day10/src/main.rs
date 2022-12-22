use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(filename: &str) -> Vec<i32> {
    let lines = read_lines(filename);

    let mut clock: i32 = 1;
    let mut regx: i32 = 1;
    let mut signals: Vec<i32> = Vec::new();

    for line_result in lines.unwrap() {
        let line_string = line_result.unwrap();
        let line_vec: Vec<&str> = line_string.split_whitespace().collect();

        match line_vec[0] {
            "noop" => {
                if check_for_important_clocks(&clock, &regx) {
                    signals.push(clock * regx);
                }
                draw_pixel(&clock, &regx);
                clock += 1;
                continue;
            }
            "addx" => {
                if check_for_important_clocks(&clock, &regx) {
                    signals.push(clock * regx);
                }
                draw_pixel(&clock, &regx);
                let add_val = line_vec[1].parse::<i32>().unwrap();
                clock += 1;
                if check_for_important_clocks(&clock, &regx) {
                    signals.push(clock * regx);
                }
                draw_pixel(&clock, &regx);
                clock += 1;
                regx += add_val;
                continue;
            }
            _ => panic!(),
        }
    }

    signals
}

fn draw_pixel(clock: &i32, regx: &i32) {
    if (*clock % 40) - 1 == *regx - 1
        || (*clock % 40) - 1 == *regx
        || (*clock % 40) - 1 == *regx + 1
    {
        print!("#");
    } else {
        print!(".");
    }

    match clock {
        40 => println!(""),
        80 => println!(""),
        120 => println!(""),
        160 => println!(""),
        200 => println!(""),
        240 => println!(""),
        _ => {}
    }
}

fn check_for_important_clocks(clock: &i32, _regx: &i32) -> bool {
    // match clock {
    //     20 => println!("Clock: {}, X: {}", clock, regx),
    //     60 => println!("Clock: {}, X: {}", clock, regx),
    //     100 => println!("Clock: {}, X: {}", clock, regx),
    //     140 => println!("Clock: {}, X: {}", clock, regx),
    //     180 => println!("Clock: {}, X: {}", clock, regx),
    //     220 => println!("Clock: {}, X: {}", clock, regx),
    //     _ => return false,
    // }
    match clock {
        20 => return true,
        60 => return true,
        100 => return true,
        140 => return true,
        180 => return true,
        220 => return true,
        _ => return false,
    }
}

fn main() {
    let test_signals = process_file("./tests/day10.txt");
    println!("");

    let signals = process_file("./day10.txt");

    let sum: i32 = signals.iter().sum();

    println!("sum of signal strengths: {}", sum);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_test() {}
    #[test]
    fn test_part1() {
        let signals = process_file("./tests/day10.txt");
        let sum: i32 = signals.iter().sum();

        assert_eq!(sum, 13140)
    }
}
