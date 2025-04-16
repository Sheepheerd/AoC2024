use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() {
    let (rules, updates) = read_lines("input");
    println!("{:?}", rules);
    println!("");
    println!("{:?}", updates);

    let mut num_safe = 0;
    let mut safe_updates: Vec<Vec<i32>> = Vec::new();
    for update in updates {
        if check_valid_update(&rules, update.clone()) {
            //println!("{:?}", update);
            safe_updates.push(update);
            num_safe += 1;
        }
    }

    let mut sum_of_middles = 0;
    for safe_update in safe_updates {
        let middle = (safe_update.len() / 2);
        println!("{:?}", safe_update);
        println!("{}", safe_update[middle]);
        sum_of_middles += safe_update[middle];
    }
    println!("{}", sum_of_middles);
}

fn check_valid_update(rules: &Vec<(i32, i32)>, update: Vec<i32>) -> bool {
    let mut working_update: Vec<i32> = Vec::new();
    let mut is_valid: bool = true;
    let mut position: usize = 0;
    for value in update.clone() {
        if working_update.is_empty() {
            working_update.push(value);
            continue;
        }
        for pair in rules {
            let before = pair.0;
            let after = pair.1;
            if before != value && position == update.len() - 1 {
                //println!("{} {}", before, after);
                if !working_update.contains(&value) {
                    working_update.push(value);
                }
                position += 1;
            } else if before != value {
                position += 1;
                continue;
            } else if !working_update.contains(&after) {
                position += 1;
                if !working_update.contains(&value) {
                    working_update.push(value);
                }
            } else {
                position += 1;
                is_valid = false;
            }
        }
        position = 0;
    }
    println!("{:?}", working_update);
    is_valid
}

fn read_lines(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut touples: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let mut get_rules: bool = true;

    let file = File::open(filename).expect("bruh");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.expect("bruh");
        // if is blank line, break
        if line.trim().is_empty() {
            get_rules = false;
            continue;
        }
        if get_rules {
            // Get first num
            let num = line.split("|").next().unwrap();

            // Get second num
            let num2 = line.split("|").last().unwrap();

            let touple = (num.parse::<i32>().unwrap(), num2.parse::<i32>().unwrap());
            touples.push(touple);
        } else if !get_rules {
            let mut update: Vec<i32> = Vec::new();
            for num in line.split(",") {
                update.push(num.parse::<i32>().unwrap());
            }
            updates.push(update);
        }
    }
    return (touples, updates);
}
