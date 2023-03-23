use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


struct Monkey {
    items: Vec<i64>,
    true_dest: usize,
    false_dest: usize,
    operation: Vec<String>,
    test_div: i64,
    items_inspected: i32,
}
impl Monkey {
    fn new() -> Self {
        Self {
            true_dest: 0,
            false_dest: 0,
            operation: vec![String::from("")],
            items: vec![0],
            test_div: 1,
            items_inspected: 0,
        }
    }

    // fn print(&self, idx: usize) {
    //     println!("Monkey {}:", idx);
    //     println!("  Items: {:?}", self.items);
    //     println!("  True dest: {}", self.true_dest);
    //     println!("  False dest: {}", self.false_dest);
    //     println!("  Operation: {:?}", self.operation);
    //     println!("  Test division: {}", self.test_div);
    //     println!("  Items inspected: {}", self.items_inspected);
    // }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_file(filename: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let lines = read_lines(filename).unwrap();

    for line_result in lines {
        let line_string = line_result.unwrap();

        let line_vec: Vec<&str> = line_string.split_whitespace().collect();
        if line_vec.len() == 0 {
            continue;
        }

        match line_vec[0] {
            "Monkey" => {
                monkeys.push(Monkey::new());
                continue;
            }
            "Starting" => {
                //get ready for some messy functional chaos
                monkeys.last_mut().unwrap().items = line_vec[2..]
                    .into_iter()
                    .map(|x| {
                        if x.ends_with(",") {
                            x[..x.len() - 1].parse::<i64>().unwrap()
                        } else {
                            x.parse::<i64>().unwrap()
                        }
                    })
                    .collect();
                continue;
            }
            "Operation:" => {
                monkeys.last_mut().unwrap().operation = line_vec[1..]
                    .into_iter()
                    .map(|x| String::from(*x))
                    .collect(); //verbose magic functional nonsense :)
            }
            "Test:" => {
                monkeys.last_mut().unwrap().test_div = line_vec.last().unwrap().parse().unwrap();
            }
            "If" => match line_vec[1] {
                "true:" => {
                    monkeys.last_mut().unwrap().true_dest =
                        line_vec.last().unwrap().parse().unwrap();
                    continue;
                }
                "false:" => {
                    monkeys.last_mut().unwrap().false_dest =
                        line_vec.last().unwrap().parse().unwrap();
                    continue;
                }
                _ => {
                    continue;
                }
            },
            _ => {
                continue;
            }
        }
    }

    monkeys
}

fn parse_op(item: i64, op_string: &Vec<String>) -> i64 {
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
fn parse_op_pt2(item_tuple: &mut (i64, usize), op_string: &Vec<String>) {
    

    let left = match op_string[2].as_str() {
        "old" => item_tuple.0,
        _ =>  0,
    };

    let right = match op_string[4].as_str() {
        "old" => item_tuple.0,
        _ => op_string[4].parse().unwrap(),
    };

    item_tuple.0 = match op_string[3].as_str() {
        "+" => left + right,
        "*" => left * right,
        _ => panic!(),
    };

}

fn process_monkeys(monkeys: &mut Vec<Monkey>, rounds: u32) {
    for _round in 0..rounds {
        for m_idx in 0..monkeys.len() {
            let mut true_temp_items = Vec::new();
            let mut false_temp_items = Vec::new();
            for i_idx in 0..monkeys[m_idx].items.len() {
                monkeys[m_idx].items_inspected += 1;
                let mut worry = parse_op(monkeys[m_idx].items[i_idx], &monkeys[m_idx].operation);
                worry = worry / 3;
                if worry % monkeys[m_idx].test_div == 0 {
                    true_temp_items.push(worry);
                } else {
                    false_temp_items.push(worry);
                }
            }
            let true_dest = monkeys[m_idx].true_dest;
            let false_dest = monkeys[m_idx].false_dest;
            monkeys[true_dest].items.extend(true_temp_items);
            monkeys[false_dest].items.extend(false_temp_items);
            monkeys[m_idx].items.clear();
        }
    }
}

fn process_monkeys_pt2(monkeys: &mut Vec<Monkey>, rounds: u32) {
    let mut items: Vec<(i64, usize)> = Vec::new(); //store the items in their own vector and
                                                      //reference monkeys
    let mut supermod = 1;
    for m_idx in 0..monkeys.len() {
        for i_idx in 0..monkeys[m_idx].items.len() {
            items.push((i64::from(monkeys[m_idx].items[i_idx]), m_idx));
        }
        supermod = supermod * monkeys[m_idx].test_div;
    }

    for _round in 0..rounds {
        for m_idx in 0..monkeys.len() {
            for i in 0..items.len() {
                if items[i].1 == m_idx{

                    items[i].0 = items[i].0 % supermod;
 
                    parse_op_pt2(&mut items[i], &monkeys[m_idx].operation);
                    if &items[i].0 % monkeys[m_idx].test_div == 0 {
                        items[i].1 = monkeys[m_idx].true_dest;
                    } else {
                        items[i].1 = monkeys[m_idx].false_dest;
                    }
                    monkeys[m_idx].items_inspected += 1;
                }
            }
            
        }
    }
}

fn get_monkey_business(monkeys: &Vec<Monkey>) -> i64 {
    let mut inspected: Vec<i32> = Vec::new();

    for m in monkeys {
        inspected.push(m.items_inspected);
    }

    inspected.sort();
    let first = inspected.pop().unwrap();
    let second = inspected.pop().unwrap();

    i64::from(first) * i64::from(second)
}

fn main() {
    let mut monkeys = process_file("./day11.txt");

    // process_monkeys(&mut monkeys, 10000);
    process_monkeys_pt2(&mut monkeys, 10000);

    let monkey_business = get_monkey_business(&monkeys);

    println!("monkey business: {}", monkey_business);
}

// TESTS

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_the_tests() {}

    #[test]
    fn test_add_single_op() {
        let op_string = vec![
            String::from("new"),
            String::from("="),
            String::from("old"),
            String::from("+"),
            String::from("4"),
        ];
        let initial: i64 = 10;
        let expected: i64 = 14;

        let result = parse_op(initial, &op_string);

        assert_eq!(expected, result)
    }
    #[test]
    fn test_mult_self_op() {
        let op_string = vec![
            String::from("new"),
            String::from("="),
            String::from("old"),
            String::from("*"),
            String::from("old"),
        ];
        let initial: i64 = 10;
        let expected: i64 = 100;

        let result = parse_op(initial, &op_string);

        assert_eq!(expected, result)
    }
    #[test]
    fn test_mult_other_op() {
        let op_string = vec![
            String::from("new"),
            String::from("="),
            String::from("old"),
            String::from("*"),
            String::from("4"),
        ];
        let initial: i64 = 10;
        let expected: i64 = 40;

        let result = parse_op(initial, &op_string);

        assert_eq!(expected, result)
    }

    #[test]
    fn test_one_round() {
        let mut monkeys = process_file("./tests/day11.txt");

        // println!("Monkey initial state:");
        // for (i,m) in monkeys.iter().enumerate() {
        //     m.print(i)
        // }
        process_monkeys(&mut monkeys, 1);
        let mut test_monkey_items: Vec<Vec<i64>> = Vec::new();

        test_monkey_items.push(vec![20, 23, 27, 26]);
        test_monkey_items.push(vec![2080, 25, 167, 207, 401, 1046]);
        test_monkey_items.push(Vec::new());
        test_monkey_items.push(Vec::new());

        for (i, m) in monkeys.iter().enumerate() {
            assert_eq!(m.items, test_monkey_items[i])
        }
    }
    #[test]
    fn test_one_round_pt2() {
        let mut monkeys = process_file("./tests/day11.txt");

        // println!("Monkey initial state:");
        // for (i,m) in monkeys.iter().enumerate() {
        //     m.print(i)
        // }
        process_monkeys_pt2(&mut monkeys, 1);

        assert_eq!(monkeys[0].items_inspected, 2);
        assert_eq!(monkeys[1].items_inspected, 4);
        assert_eq!(monkeys[2].items_inspected, 3);
        assert_eq!(monkeys[3].items_inspected, 6);
    }

    #[test]
    fn test_20_round_pt2() {
        let mut monkeys = process_file("./tests/day11.txt");

        // println!("Monkey initial state:");
        // for (i,m) in monkeys.iter().enumerate() {
        //     m.print(i)
        // }
        process_monkeys_pt2(&mut monkeys, 20);

        assert_eq!(monkeys[0].items_inspected, 99);
        assert_eq!(monkeys[1].items_inspected, 97);
        assert_eq!(monkeys[2].items_inspected, 8);
        assert_eq!(monkeys[3].items_inspected, 103);
    }

    #[test]
    fn test_overall_monkey_business() {
        let mut monkeys = process_file("./tests/day11.txt");
        process_monkeys(&mut monkeys, 20);
        let monkey_business = get_monkey_business(&monkeys);

        assert_eq!(monkey_business, 10605);
    }
}
