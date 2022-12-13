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

/// Represent trees as (u8, bool) tuples. The bool represents visibility.
///
/// Grid layout:
/// 0 ─────→+
/// │ ttttt
/// │ ttttt
/// ↓
/// +
fn process_file(file: &str) -> Vec<Vec<(u8, bool)>> {
    let lines = read_lines(file).unwrap();
    let mut grid: Vec<Vec<(u8, bool)>> = Vec::new();
    for line_result in lines {
        let line = line_result.unwrap();
        let line_str_chars = line.as_str().chars();

        let mut grid_line: Vec<(u8, bool)> = Vec::new();
        for c in line_str_chars {
            grid_line.push((c.to_digit(10).unwrap() as u8, false));
        }
        grid.push(grid_line);
    }

    grid
}

fn find_visible(grid: &mut Vec<Vec<(u8, bool)>>) {
    //process side views
    let mut row_count = 0;
    let row_max = grid.len();
    while row_count < row_max {
        //process from left to right
        let mut max_height: u8 = 0;
        let mut col_count = 0;

        loop {
            if col_count == grid[row_count].len() {
                break;
            }
            if grid[row_count][col_count].0 <= max_height && col_count != 0 {
                //tree is not visible
            } else {
                //tree is visible
                grid[row_count][col_count].1 = true;
                max_height = grid[row_count][col_count].0;
            }
            col_count += 1;
        }
        max_height = 0;
        col_count = grid[row_count].len() - 1;
        loop {
            if grid[row_count][col_count].0 <= max_height && col_count != grid[row_count].len() - 1
            {
                //tree is not visible
            } else {
                //tree is visible
                grid[row_count][col_count].1 = true;
                max_height = grid[row_count][col_count].0;
            }
            if col_count == 0 {
                break;
            }
            col_count -= 1;
        }
        row_count += 1;
    }

    // process top/bottom views
    let mut col_count = 0;
    let col_max = grid[0].len();
    let row_max = grid.len();

    while col_count < col_max {
        //top to bottom
        let mut row_count = 0;
        let mut max_height: u8 = 0;
        loop {
            if grid[row_count][col_count].0 <= max_height && row_count != 0 {
                //tree is not visible
            } else {
                //tree is visible
                grid[row_count][col_count].1 = true;
                max_height = grid[row_count][col_count].0;
            }
            if row_count == row_max - 1 {
                break;
            }
            row_count += 1;
        }

        //bottom to top
        row_count = row_max - 1;
        max_height = 0;
        loop {
            if grid[row_count][col_count].0 <= max_height && row_count != row_max - 1 {
                //tree is not visible
            } else {
                //tree is visible
                grid[row_count][col_count].1 = true;
                max_height = grid[row_count][col_count].0;
            }
            if row_count == 0 {
                break;
            }
            row_count -= 1;
        }
        col_count += 1;
    }
}

fn get_scenic_scores(grid: &Vec<Vec<(u8, bool)>>) -> Vec<i32> {
    let mut row_count: usize = 0;
    let row_max: usize = grid.len();
    let col_max: usize = grid[0].len();
    let mut scores: Vec<i32> = Vec::new();

    while row_count < row_max {
        let mut col_count: usize = 0;
        while col_count < col_max {
            let tree_height = grid[row_count][col_count].0;

            let mut right_count: i32 = 0;
            for height in grid[row_count][col_count..col_max - 1].into_iter() {
                if height.0 < tree_height {
                    right_count += 1;
                } else {
                    right_count += 1;
                    break;
                }
            }

            let mut left_count: i32 = 0;
            for height in grid[row_count][0..col_count].into_iter().rev() {
                if height.0 < tree_height {
                    left_count += 1;
                } else {
                    left_count += 1;
                    break;
                }
            }

            let mut down_count: i32 = 0;
            for row in grid[row_count..row_max - 1].into_iter() {
                for height in row[col_count] {
                    if height.0 < tree_height {
                        down_count += 1;
                    } else {
                        down_count += 1;
                        break;
                    }
                }
            }

            let mut up_count: i32 = 0;
            for height in grid[0..row_count][col_count].into_iter().rev() {
                if height.0 < tree_height {
                    up_count += 1;
                } else {
                    up_count += 1;
                    break;
                }
            }
            col_count += 1;
            scores.push(right_count * left_count * up_count * down_count);
        }
        row_count += 1;
    }
    scores
}

fn count_visible(grid: &Vec<Vec<(u8, bool)>>) -> i32 {
    let mut tree_count: i32 = 0;
    for row in grid {
        for col in row {
            if col.1 {
                tree_count += 1;
            }
        }
    }
    tree_count
}

fn main() {
    println!("Hello, world!");

    let mut grid = process_file("./day8.txt");
    find_visible(&mut grid);
    let tree_count = count_visible(&grid);
    println!("Visible trees: {}", tree_count);

    // println!("{:?}", grid);
    // for line in grid {
    //     println!("{:?}", line);
    // }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn test_tree_count() {
        let mut grid = process_file("./tests/day8.txt");
        find_visible(&mut grid);
        let tree_count = count_visible(&grid);

        assert_eq!(21, tree_count)
    }
}
