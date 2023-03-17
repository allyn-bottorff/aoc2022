use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Monkey {
    items: Vec<i32>,
    true_dest: i32,
    false_dest: i32,
    operation: Vec<String>,
    test_div: i32,
}
impl Monkey {
    fn new() -> Self {
        Self {
            true_dest: 0,
            false_dest: 0,
            operation: vec![String::from("")],
            items: vec![0],
            test_div: 1,
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



fn process_file(filename: &str) -> Vec<Monkey>{
    let mut monkeys: Vec<Monkey> = Vec::new();

    let lines = read_lines(filename).unwrap();

    for line_result in lines {
        let line_string = line_result.unwrap();

        let line_vec: Vec<&str> = line_string.split_whitespace().collect();
        if line_vec.len() == 0 {
            continue;
        }

        match line_vec[0]{
            "Monkey" => {
                monkeys.push(Monkey::new());
                continue;
            }
            "Starting" => { 

                //get ready for some messy functional chaos
                monkeys.last_mut().unwrap().items = line_vec[2..].into_iter().map(|x| {
                    if x.ends_with(",") {
                       x[..x.len()-1].parse::<i32>().unwrap() 
                    } else {
                        x.parse::<i32>().unwrap()
                    }

                }).collect();
                continue;
            }
            "Operation:" => {
                monkeys.last_mut().unwrap().operation = line_vec[1..].into_iter().map(|x| String::from(*x)).collect(); //verbose magic functional nonsense :)

            }
            "Test" => {
                monkeys.last_mut().unwrap().test_div = line_vec.last().unwrap().parse().unwrap();

            }
            "If" => {
                match line_vec[1] {
                    "true:" => {
                        monkeys.last_mut().unwrap().true_dest = line_vec.last().unwrap().parse().unwrap();
                        continue;
                    }
                    "false:" =>  {
                        monkeys.last_mut().unwrap().false_dest = line_vec.last().unwrap().parse().unwrap();
                        continue;
                    }
                    _ => {
                        continue;
                    }
                }
            }
            _ => { continue; }
        }
    }

    monkeys
}


fn parse_op(item: i32, op_string: Vec<String>) -> i32 {

    let left = match op_string[2].as_str() {
        "old" => item,
        _ => 0,
    };

    let right = match op_string[4].as_str() {
        "old" => item,
        _ => op_string[4].parse().unwrap(),
    };

    let new_val = match op_string[3].as_str() {
        "+" => left + right,
        "*" => left * right,
        _ => panic!(),
    };


    new_val



}

// fn process_file(filename: &str) {
//     let mut monkeys: Vec<Monkey> = Vec::new();
//     let lines = read_lines(filename);
//
//
//     let mut starting_items: Vec<i32> = Vec::new();
//     let mut operation: fn(i32, i32) -> i32  = add_two;
//     let mut test: fn(i32) -> bool = |x: i32| -> bool {true};
//     let mut true_dest: usize = 0;
//     let mut false_dest: usize = 0;
//     let mut op_const: i32 = 0;
//     for line_result in lines.unwrap() {
//         let line_string = line_result.unwrap();
//         let line_vec: Vec<&str> = line_string.split_whitespace().collect();
//         println!("{:?}", line_vec);
//         if line_vec.len() == 0 {
//             continue;
//         }
//
//         match line_vec[0] {
//             "Monkey" => continue,
//             "Starting" => {
//                 for item in line_vec {
//                     let num_result = item.parse::<i32>();
//                     match num_result {
//                         Ok(x) => {
//                             starting_items.push(x);
//                         }
//                         Err(_) => {}
//                     }
//                 }
//             }
//             "Operation:" =>  {
//             match line_vec[4] {
//                             // "+" => operation = |x: i32, x: i32| -> i32 {x + x},
//                             // "-" => operation = |x: i32| -> i32 {x - x},
//                             // "*" => operation = |x: i32| -> i32 {x * x},
//                             "+" => operation = add_two,
//                             "-" => operation = |x: i32| -> i32 {x - x},
//                             "*" => operation = |x: i32| -> i32 {x * x},
//
//             }
//         }
//
//                     }
//                 }
//                 }
//             
            

fn main() {
    println!("Hello, world!");
    let monkeys = process_file("./tests/day11.txt");

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey: {}", i);
        println!("  Starting items: {:?}", monkey.items);
        println!("  Operation: {:?}", monkey.operation);
        println!("  Test divisor: {}", monkey.test_div); 
        println!("    True dest: {}", monkey.true_dest);
        println!("    False dest: {}", monkey.false_dest);
    }
    
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn test_add_single_op() {
        let op_string = vec![String::from("new"), String::from("="), String::from("old"), String::from("+"), String::from("4")];
        let initial: i32 = 10;
        let expected: i32 = 14; 

        let result = parse_op(initial, op_string);


        assert_eq!(expected, result)
    }
    #[test]
    fn test_mult_self_op() {
        let op_string = vec![String::from("new"), String::from("="), String::from("old"), String::from("*"), String::from("old")];
        let initial: i32 = 10;
        let expected: i32 = 100; 

        let result = parse_op(initial, op_string);


        assert_eq!(expected, result)
    }
    #[test]
    fn test_mult_other_op() {
        let op_string = vec![String::from("new"), String::from("="), String::from("old"), String::from("*"), String::from("4")];
        let initial: i32 = 10;
        let expected: i32 = 40; 

        let result = parse_op(initial, op_string);


        assert_eq!(expected, result)
    }
}
