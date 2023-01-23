use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
    items: Vec<i32>,
    true_dest: i32,
    false_dest: i32,
    operation: fn(i32, i32) -> i32,
    op_const: i32,
    test: fn(i32) -> bool,
}
impl Monkey {
    fn new() -> Self {
        let operation = add_two;
        let test = |x: i32| -> bool { x % 10 == 0 };
        Self {
            true_dest: 0,
            false_dest: 0,
            operation: operation,
            op_const: 0,
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

fn add_two(old: i32, c: i32) -> i32 {
    old + c
}

fn mult_two(old: i32, c: i32) -> i32 {
    old * c
}

fn sub_two(old: i32, c: i32) -> i32 {
    old - c
}

fn process_file(filename: &str) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines = read_lines(filename);


    let mut starting_items: Vec<i32> = Vec::new();
    let mut operation: fn(i32, i32) -> i32  = add_two;
    let mut test: fn(i32) -> bool = |x: i32| -> bool {true};
    let mut true_dest: usize = 0;
    let mut false_dest: usize = 0;
let mut op_const: i32 = 0;
    for line_result in lines.unwrap() {
        let line_string = line_result.unwrap();
        let line_vec: Vec<&str> = line_string.split_whitespace().collect();
        println!("{:?}", line_vec);
        if line_vec.len() == 0 {
            continue;
        }

        match line_vec[0] {
            "Monkey" => continue,
            "Starting" => {
                for item in line_vec {
                    let num_result = item.parse::<i32>();
                    match num_result {
                        Ok(x) => {
                            starting_items.push(x);
                        }
                        Err(_) => {}
                    }
                }
            }
            "Operation:" =>  {
                let constant_result = line_vec[5].parse::<i32>();
                match constant_result {
                    Ok(c) => {
                        match line_vec[4] {
                            "+" => {
                    operation = &add_two;
                    

}
                            "-" => operation = &sub_two,
                            "*" => operation = &mult_two,
                        },
                    Err(_) => {
            match line_vec[4] {
                            "+" => operation = |x: i32| -> i32 {x + x},
                            "-" => operation = |x: i32| -> i32 {x - x},
                            "*" => operation = |x: i32| -> i32 {x * x},

            }
        }

                    }
                }
                match line_vec[4] {
                    "+"
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
