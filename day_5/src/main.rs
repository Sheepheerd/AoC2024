use std::fs::File;
use std::io::{self, prelude::*, BufReader};
fn main() {
    let (rules, updates) = read_lines("input");
    println!("{:?}", rules);
    println!("");
    println!("{:?}", updates);
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
