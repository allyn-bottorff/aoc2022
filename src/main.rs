use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max_calories_1: i32 = 0;
    let mut max_calories_2: i32 = 0;
    let mut max_calories_3: i32 = 0;

    let mut current_elf: i32 = 1;
    if let Ok(lines) = read_lines("./day1.txt") {
        let mut current_calories: i32 = 0;
        for line in lines {
            if let Ok(cal_str) = line {
                let cal = match cal_str.parse::<i32>() {
                    Ok(cal) => cal,
                    Err(_) => {
                        current_elf += 1;
                        if current_calories > max_calories_1 {
                            max_calories_3 = max_calories_2;
                            max_calories_2 = max_calories_1;
                            max_calories_1 = current_calories;
                        } else if current_calories > max_calories_2 {
                            max_calories_2 = current_calories;
                        } else if current_calories > max_calories_3 {
                            max_calories_3 = current_calories;
                        }
                        current_calories = 0;
                        continue;
                    }
                };

                current_calories += cal;
            }
        }
    }
    let top_3_sum = max_calories_1 + max_calories_2 + max_calories_3;
    println!("Hello, world!");
    println!("Max Calories: {}", max_calories_1);
    println!("From Elf: {}", current_elf);
    println!(
        "Top 3 calorie counts: {} {} {}",
        max_calories_1, max_calories_2, max_calories_3
    );
    println!("Sum of top 3: {}", top_3_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
