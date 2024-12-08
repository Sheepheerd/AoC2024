use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let s = read_lines("input");

    // Part A

    //let mut correct_lines: Vec<Vec<String>> = Vec::new();
    //for line in &s {
    //    let new_word = str_strip_numbers(line);
    //    correct_lines.push(new_word);
    //}
    //let mut stripped_mul_lines: Vec<Vec<String>> = Vec::new();
    //for group in &correct_lines {
    //    for word in group {
    //        let new_word = str_strip_mul(word);
    //        stripped_mul_lines.push(new_word);
    //    }
    //}
    //for word in &stripped_mul_lines {
    //    println!("{:?}", word);
    //}
    //let mut product_sum = 0;
    //
    //for word in stripped_mul_lines {
    //    product_sum += word[0].parse::<i32>().unwrap() * word[1].parse::<i32>().unwrap();
    //}
    //println!("{:?}", product_sum);

    ///////////////////////
    // Part B

    let mut correct_lines: Vec<Vec<String>> = Vec::new();
    for line in &s {
        let new_word = str_strip_numbers_b(line);
        correct_lines.push(new_word);
    }
    let mut stripped_mul_lines: Vec<Vec<String>> = Vec::new();
    for group in &correct_lines {
        for word in group {
            let new_word = str_strip_mul_b(word);
            stripped_mul_lines.push(new_word);
        }
    }
    for word in &stripped_mul_lines {
        println!("{:?}", word);
    }
    let mut product_sum = 0;
    let mut can_multiply = true;
    for word in stripped_mul_lines {
        println!("{:?}", word);
        if word[0] == "do()" {
            can_multiply = true;
        } else if word[0] == "don't()" {
            can_multiply = false;
        } else {
            if can_multiply == true {
                product_sum += word[0].parse::<i32>().unwrap() * word[1].parse::<i32>().unwrap();
            }
        }
    }
    println!("{:?}", product_sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn str_strip_numbers_b(s: &str) -> Vec<String> {
    let RE: Regex = Regex::new(r"mul\(\s*\d+\s*,\s*\d+\s*\)|do\(\)|don't\(\)").unwrap();
    // iterate over all matches
    RE.find_iter(s)
        .filter_map(|str| str.as_str().parse().ok())
        .collect()
}

fn str_strip_mul_b(s: &str) -> Vec<String> {
    let RE: Regex = Regex::new(r"\d+|do\(\)|don't\(\)").unwrap();
    // iterate over all matches
    RE.find_iter(s)
        .filter_map(|str| str.as_str().parse().ok())
        .collect()
}

//fn str_strip_numbers(s: &str) -> Vec<String> {
//    let RE: Regex = Regex::new(r"mul\(\s*\d+\s*,\s*\d+\s*\)").unwrap();
//    // iterate over all matches
//    RE.find_iter(s)
//        .filter_map(|str| str.as_str().parse().ok())
//        .collect()
//}

//fn str_strip_mul(s: &str) -> Vec<String> {
//    let RE: Regex = Regex::new(r"\d+").unwrap();
//    // iterate over all matches
//    RE.find_iter(s)
//        .filter_map(|str| str.as_str().parse().ok())
//        .collect()
//}
