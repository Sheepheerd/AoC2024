use std::fs::read_to_string;
fn main() {
    // Part A
    let result = read_lines("input");
    //let mut safe = 0;
    //for i in result {
    //    println!("{}", i);
    //    let value: Vec<i32> = i
    //        .split_whitespace()
    //        .map(|a| a.parse::<i32>().unwrap())
    //        .collect();
    //
    //    if is_safe(value) {
    //        safe += 1;
    //    }
    //}
    //println!("{}", safe);

    /////////////////////////
    // Part B
    let mut safe: i32 = 0;
    let mut was_safe: bool = false;

    // 2 3 4 5 = i
    for i in result {
        let value: Vec<i32> = i
            .split_whitespace()
            .map(|a| a.parse::<i32>().unwrap())
            .collect();
        // value.len() = 4
        for j in 0..value.len() {
            let mut temp_value = value.clone();
            if !was_safe {
                if j != temp_value.len() {
                    temp_value.remove(j);
                    // 3 4 5
                }

                if is_safe(temp_value.clone()) {
                    was_safe = true;
                }
            }
        }

        if was_safe {
            safe += 1;
        }
        was_safe = false;
    }
    println!("{}", safe);
}

fn is_safe(value: Vec<i32>) -> bool {
    let mut pointer_a = 0;
    let mut pointer_b = 1;

    let mut increasing: bool = false;
    let mut difference_a_b = value[pointer_a] - value[pointer_b];

    if difference_a_b.abs() > 3 || difference_a_b.abs() == 0 {
        return false;
    }

    if value[pointer_a] < value[pointer_b] {
        increasing = true;
    } else if value[pointer_a] > value[pointer_b] {
        increasing = false;
    }

    pointer_a += 1;
    pointer_b += 1;
    while pointer_b < value.len() {
        difference_a_b = value[pointer_a] - value[pointer_b];
        if difference_a_b.abs() > 3 || difference_a_b.abs() == 0 {
            return false;
        }
        if value[pointer_a] == value[pointer_b] {
            return false;
        } else if increasing == true && value[pointer_a] > value[pointer_b] {
            return false;
        } else if increasing == false && value[pointer_a] < value[pointer_b] {
            return false;
        }
        pointer_a += 1;
        pointer_b += 1;
    }
    true
}
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
