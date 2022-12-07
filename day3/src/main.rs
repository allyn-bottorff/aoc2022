use std::collections::HashMap;
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

fn main() {
    rucksack_sorting("./day3.txt");
    rucksack_badge_search("./day3.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
