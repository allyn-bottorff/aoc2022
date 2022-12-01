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

fn get_top_calories(file: &str) -> (i32, i32, i32) {
    let mut current_calories: i32 = 0;
    let mut max_calories_1: i32 = 0;
    let mut max_calories_2: i32 = 0;
    let mut max_calories_3: i32 = 0;

    // let mut current_elf: i32 = 1;
    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(cal_str) = line {
                let cal = match cal_str.parse::<i32>() {
                    Ok(cal) => cal,
                    Err(_) => {
                        // current_elf += 1;
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

    (max_calories_1, max_calories_2, max_calories_3)
}

fn main() {
    let (max_cal_1, max_cal_2, max_cal_3) = get_top_calories("./day1.txt");
    let top_3_sum = max_cal_1 + max_cal_2 + max_cal_3;
    println!("Max Calories: {}", max_cal_1);
    println!(
        "Top 3 calorie counts: {} {} {}",
        max_cal_1, max_cal_2, max_cal_3
    );
    println!("Sum of top 3: {}", top_3_sum);
}
