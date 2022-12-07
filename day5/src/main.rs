use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

    let mut piles: Vec<Vec<char>> = vec![vec![' ']; num_of_piles];

    // Start with the line above the stack numbers.
    let mut i: usize = state_lines.len() - 2;
    loop {
        let j_len: usize = state_lines[i].len();
        let mut j: usize = 0;
        let mut col: usize = 0;
        while j < j_len {
            if j % 4 == 1 {
                let line = &state_lines[i];
                let line_str = line.as_str().clone();
                let char = line_str.chars().nth(j).unwrap();
                if !char.is_ascii_alphabetic() {
                    j += 1;
                    col += 1;
                    continue;
                }

                if piles[col][0] == ' ' {
                    piles[col][0] = char;
                } else {
                    piles[col].push(char);
                }
                col += 1;
            }
            j += 1;
        }
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }

    piles
}

fn parse_crane_proc(proc_lines: Vec<String>) -> CraneMoves {
    let mut program = CraneMoves {
        quantity: Vec::new(),
        source: Vec::new(),
        destination: Vec::new(),
    };

    for line in proc_lines {
        let words = line.split(" ");
        for (i, word) in words.enumerate() {
            if i == 1 {
                let q = word.parse::<usize>().unwrap();
                program.quantity.push(q);
            } else if i == 3 {
                let v = word.parse::<usize>().unwrap();
                program.source.push(v)
            } else if i == 5 {
                let v = word.parse::<usize>().unwrap();
                program.destination.push(v)
            }
        }
    }

    program
}

fn run_crane(proc: CraneMoves, mut state: Vec<Vec<char>>) -> Vec<char> {
    let mut i: usize = 0;
    while i < proc.quantity.len() {
        let mut j: usize = 0;
        while j < proc.quantity[i] {
            let container = state[proc.source[i] - 1].pop().unwrap();
            state[proc.destination[i] - 1].push(container);

            j += 1;
        }
        i += 1;
    }

    let mut results: Vec<char> = Vec::new();
    for col in &state {
        results.push(*col.last().unwrap());
    }

    println!("Tops of the stacks: {:?}", results);
    results
}

fn run_crane_2(proc: CraneMoves, mut state: Vec<Vec<char>>) -> Vec<char> {
    let mut i: usize = 0;
    while i < proc.quantity.len() {
        let len = state[proc.source[i] - 1].len();
        let mut containers = state[proc.source[i] - 1].split_off(len - proc.quantity[i]);
        state[proc.destination[i] - 1].append(&mut containers);

        i += 1;
    }

    let mut results: Vec<char> = Vec::new();
    for col in &state {
        results.push(*col.last().unwrap());
    }

    println!("Tops of the stacks: {:?}", results);
    results
}
fn main() {
    let (state_lines, proc_lines) = read_crane_data("./day5.txt");
    let i_state = parse_crane_state(state_lines);
    let proc = parse_crane_proc(proc_lines);
    run_crane(proc, i_state);

    let (state_lines_2, proc_lines_2) = read_crane_data("./day5.txt");
    let i_state_2 = parse_crane_state(state_lines_2);
    let proc_2 = parse_crane_proc(proc_lines_2);
    let _ = run_crane_2(proc_2, i_state_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_read() {
        let (conf, proc) = read_crane_data("./tests/day5.txt");
        assert_eq!(conf.len(), 4);
        assert_eq!(proc.len(), 4);
    }
    #[test]
    fn test_day5_crane_state() {
        let (state, _) = read_crane_data("./tests/day5.txt");
        let actual_state: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let parsed_state: Vec<Vec<char>> = parse_crane_state(state);

        assert_eq!(actual_state, parsed_state)
    }

    #[test]
    fn test_day5_crane_proc() {
        let (_, proc_lines) = read_crane_data("./tests/day5.txt");
        let actual_proc = CraneMoves {
            quantity: vec![1, 3, 2, 1],
            source: vec![2, 1, 2, 1],
            destination: vec![1, 3, 1, 2],
        };

        let parsed_proc = parse_crane_proc(proc_lines);
        assert_eq!(actual_proc.quantity, parsed_proc.quantity);
        assert_eq!(actual_proc.source, parsed_proc.source);
        assert_eq!(actual_proc.destination, parsed_proc.destination);
    }

    #[test]
    fn test_day5_run_crane() {
        let (state_lines, proc_lines) = read_crane_data("./tests/day5.txt");
        let i_state = parse_crane_state(state_lines);
        let proc = parse_crane_proc(proc_lines);

        let final_state = run_crane(proc, i_state);
        let expected: Vec<char> = vec!['C', 'M', 'Z'];

        assert_eq!(final_state, expected)
    }
    #[test]
    fn test_day5_2_run_crane() {
        let (state_lines, proc_lines) = read_crane_data("./tests/day5.txt");
        let i_state = parse_crane_state(state_lines);
        let proc = parse_crane_proc(proc_lines);

        let final_state = run_crane_2(proc, i_state);
        let expected: Vec<char> = vec!['M', 'C', 'D'];

        assert_eq!(final_state, expected)
    }
}
