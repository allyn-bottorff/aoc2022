use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone, Copy, Eq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(file: &str) -> Vec<Coord> {
    let mut head: Coord = Coord { x: 0, y: 0 };
    let mut tail: Coord = Coord { x: 0, y: 0 };
    let mut tail_locations: Vec<Coord> = Vec::new();
    let lines = read_lines(file).unwrap();
    for line_result in lines {
        let line = line_result.unwrap();
        let line_vec = line.split_once(' ').unwrap();
        let direction = line_vec.0.parse::<char>().unwrap();
        let val = line_vec.1.parse::<i32>().unwrap();

        let mut i: i32 = 0;
        while i < val {
            let prev_head = head.clone();
            match direction {
                'R' => head.x += 1,
                'L' => head.x += -1,
                'U' => head.y += 1,
                'D' => head.y += -1,
                _ => panic!(),
            }
            if !is_adjacent(&head, &tail) {
                tail = prev_head;
                tail_locations.push(tail);
            }
            i += 1;
        }
    }
    tail_locations
}

fn number_of_unique_locations(locations: Vec<Coord>) -> i32 {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut duplicate_count: i32 = 0;
    let location_max: usize = locations.len();
    while i < location_max {
        while j < location_max {
            if i != j && locations[i] == locations[j] {
                duplicate_count += 1;
            }
            j += 1;
        }
        i += 1;
    }
    location_max as i32 - duplicate_count
}

/// Determine if points are within one unit of each other, including diagonal
fn is_adjacent(p1: &Coord, p2: &Coord) -> bool {
    if p1.x - p2.x > 1 || p1.x - p2.x < -1 {
        return false;
    }

    if p1.y - p2.y > 1 || p1.y - p2.y < -1 {
        return false;
    }

    return true;
}

fn main() {
    let tail_locs = process_file("./day9.txt");
    let unique_tail_locs = number_of_unique_locations(tail_locs);

    println!("Unique tail locations: {}", unique_tail_locs);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn test_adjacent_diagonal() {
        let p1 = Coord { x: 5, y: 5 };
        let p2 = Coord { x: 6, y: 6 };
        assert_eq!(true, is_adjacent(&p1, &p2))
    }
    #[test]
    fn test_adjacent() {
        let p1 = Coord { x: 5, y: 5 };
        let p2 = Coord { x: 6, y: 5 };
        assert_eq!(true, is_adjacent(&p1, &p2))
    }
    #[test]
    fn test_not_adjacent() {
        let p1 = Coord { x: 5, y: 5 };
        let p2 = Coord { x: 7, y: 6 };
        assert_eq!(false, is_adjacent(&p1, &p2))
    }
    #[test]
    fn test_unique_locations() {
        let tail_locs = process_file("./tests/day9.txt");
        let tail_count = number_of_unique_locations(tail_locs);

        assert_eq!(tail_count, 13);
    }
}
