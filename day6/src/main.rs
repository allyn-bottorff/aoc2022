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

fn find_packet_start(file: &str) -> usize {
    7
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_packet_start_1() {
        let packet_start = find_packet_start("./tests/day6-1.txt");
        assert_eq!(packet_start, 7)
    }
}
