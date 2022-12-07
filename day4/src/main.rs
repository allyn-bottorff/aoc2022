use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
fn main() {
    find_complete_overlaps("./day4.txt");
    find_partial_overlaps("./day4.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
