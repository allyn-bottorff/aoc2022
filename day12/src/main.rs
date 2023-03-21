use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

// 83 -> 'S' (starting point)
// 69 -> 'E' (ending point)

fn read_file(path: &str) -> Lines<BufReader<File>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let lines = reader.lines();

    lines
}

fn make_topo(lines: Lines<BufReader<File>>) -> Vec<Vec<u8>> {
    let mut topo: Vec<Vec<u8>> = Vec::new();

    for line_r in lines {
        let line = line_r.unwrap();

        let heights = line.chars().map(|c| c as u8).collect::<Vec<u8>>();
        topo.push(heights);
    }

    topo
}

fn main() {
    println!("Hello, world!");

    let lines = read_file("tests/day12.txt");

    let topo = make_topo(lines);

    for line in topo {
        println!("{:?}", line);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_the_tests() {}
}
