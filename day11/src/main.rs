use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
    items: Vec<i32>,
    true_dest: i32,
    false_dest: i32,
    operation: fn(i32) -> i32,
    test: fn(i32) -> bool,
}
impl Monkey {
    fn new() -> Self {
        let operation = |x: i32| -> i32 { x + 3 };
        let test = |x: i32| -> bool { x % 10 == 0 };
        Self {
            true_dest: 0,
            false_dest: 0,
            operation: operation,
            items: vec![0],
            test: test,
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(filename: &str) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines = read_lines(filename);

    for line_result in lines.unwrap() {
        let line_string = line_result.unwrap();
        let line_vec: Vec<&str> = line_string.split_whitespace().collect();
        println!("{:?}", line_vec);
        if line_vec.len() == 0 {
            continue;
        }

        match line_vec[0] {
            "Monkey" => monkeys.push(Monkey::new()),
            "Starting" => {
                for item in line_vec {
                    let num_result = item.parse::<i32>();
                    match num_result {
                        Ok(x) => {
                            let monkey = &mut *monkeys.last().unwrap();
                            monkey.items.push(x);
                        }
                        Err(_) => {}
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    process_file("./tests/day11.txt");
}

#[cfg(test)]
mod test {
    #[test]
    fn test_the_tests() {}
}
