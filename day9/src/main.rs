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
    tail_locations.push(tail);
    let lines = read_lines(file).unwrap();
    for line_result in lines {
        let line = line_result.unwrap();
        let line_vec = line.split_once(' ').unwrap();
        let direction = line_vec.0.parse::<char>().unwrap();
        let val = line_vec.1.parse::<i32>().unwrap();

        let mut i: i32 = 1;
        while i <= val {
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

fn process_file_pt2(file: &str) -> Vec<Coord> {
    let mut rope: Vec<Coord> = Vec::new();
    let mut i: usize = 0;

    while i < 10 {
        rope.push(Coord { x: 0, y: 0 });
        i += 1;
    }

    let mut tail_locations: Vec<Coord> = Vec::new();
    tail_locations.push(rope[0]);
    let lines = read_lines(file).unwrap();
    for line_result in lines {
        let line = line_result.unwrap();
        let line_vec = line.split_once(' ').unwrap();
        let direction = line_vec.0.parse::<char>().unwrap();
        let val = line_vec.1.parse::<i32>().unwrap();

        let mut i: i32 = 1;
        while i <= val {
            match direction {
                'R' => rope[0].x += 1,
                'L' => rope[0].x += -1,
                'U' => rope[0].y += 1,
                'D' => rope[0].y += -1,
                _ => panic!(),
            }

            let mut k: usize = 0;
            while k < rope.len() - 1 {
                if !is_adjacent(&rope[k], &rope[k + 1]) {
                    if rope[k].x == rope[k + 1].x {
                        // same x, move vertically
                        if rope[k].y > rope[k + 1].y {
                            rope[k + 1].y += 1;
                        } else {
                            rope[k + 1].y -= 1;
                        }
                    } else if rope[k].y == rope[k + 1].y {
                        // same y, move horizontall
                        if rope[k].x > rope[k + 1].x {
                            rope[k + 1].x += 1;
                        } else {
                            rope[k + 1].x -= 1;
                        }
                    } else {
                        // diagonal moves
                        if rope[k].x > rope[k + 1].x {
                            rope[k + 1].x += 1;
                        } else {
                            rope[k + 1].x -= 1;
                        }
                        if rope[k].y > rope[k + 1].y {
                            rope[k + 1].y += 1;
                        } else {
                            rope[k + 1].y -= 1;
                        }
                    }
                    if k + 1 == rope.len() - 1 {
                        tail_locations.push(rope[k + 1]);
                    }
                }

                k += 1;
            }
            i += 1;
        }
    }
    tail_locations
}

fn number_of_unique_locations(locations: &Vec<Coord>) -> i32 {
    let mut unique_coords: Vec<Coord> = Vec::new();

    'outer: for coord in locations {
        let mut i: usize = 0;
        while i < unique_coords.len() {
            if *coord == unique_coords[i] {
                continue 'outer;
            }
            i += 1;
        }
        unique_coords.push(*coord);
    }

    unique_coords.len() as i32
}

/// Determine if points are within one unit of each other, including diagonal
fn is_adjacent(p1: &Coord, p2: &Coord) -> bool {
    if p1.x - p2.x > 1 || p2.x - p1.x > 1 {
        return false;
    }

    if p1.y - p2.y > 1 || p2.y - p1.y > 1 {
        return false;
    }

    return true;
}

fn main() {
    let tail_locs = process_file("./day9.txt");
    let tail_locs_pt2 = process_file_pt2("./day9.txt");
    let unique_tail_locs = number_of_unique_locations(&tail_locs);
    let unique_tail_locs_pt2 = number_of_unique_locations(&tail_locs_pt2);

    println!("Unique tail locations part 1: {}", unique_tail_locs);
    println!("Unique tail locations part 2: {}", unique_tail_locs_pt2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn test_adjacent_diagonal() {
        let p1 = Coord { x: 6, y: 6 };
        let p2 = Coord { x: 5, y: 5 };
        assert_eq!(true, is_adjacent(&p1, &p2))
    }
    #[test]
    fn test_adjacent() {
        let p1 = Coord { x: 6, y: 5 };
        let p2 = Coord { x: 5, y: 5 };
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
        let tail_count = number_of_unique_locations(&tail_locs);

        assert_eq!(tail_count, 13);
    }
    #[test]
    fn test_unique_locations2() {
        let tail_locs = process_file_pt2("./tests/day9-2.txt");
        let tail_count = number_of_unique_locations(&tail_locs);
        println!("day 2 tail locations: {:?}", tail_locs);

        assert_eq!(tail_count, 36);
    }
}
