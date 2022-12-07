use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Directory {
    name: String,
    directories: Vec<Directory>,
    files: Vec<(i32, String)>,
}

enum LineType {
    CD(String),
    LS,
    File((i32, String)),
    Dir(String),
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_file(file: &str) {
    let lines = read_lines(file).unwrap();

    for line_result in lines {
        let line = line_result.unwrap().as_str();
    }
}

/// Interpret the line type and package relevent info.
fn parse_line(line: &str) -> LineType {
    let line_parts: Vec<&str> = line.split(' ').collect();
    if line_parts[0] == "$" && line_parts[1] == "cd" {
        return LineType::CD(line_parts[2].to_string());
    }
    if line_parts[0] == "$" && line_parts[1] == "ls" {
        return LineType::LS;
    }
    if line_parts[0] == "dir" {
        return LineType::Dir(line_parts[1].to_string());
    } else {
        let size: i32 = line_parts[0].parse::<i32>().unwrap();
        return LineType::File((size, line_parts[1].to_string()));
    }
}

fn main() {
    println!("Hello, world!");
}
