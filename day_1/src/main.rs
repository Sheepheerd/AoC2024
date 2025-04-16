use std::fs;
fn main() {
    // Part A
    let message: String = fs::read_to_string("input").unwrap();
    let split_message: Vec<i32> = message
        .split_whitespace()
        .map(|a| a.parse::<i32>().unwrap())
        .collect();

    let mut first_half: Vec<i32> = Vec::new();
    let mut second_half: Vec<i32> = Vec::new();

    let mut counter = 0;
    for elem in split_message {
        if counter % 2 == 0 {
            first_half.push(elem);
        } else {
            second_half.push(elem);
        }
        counter += 1;
    }
    let mut difference_vec: Vec<i32> = Vec::new();
    first_half.sort();
    second_half.sort();
    for i in 0..first_half.len() {
        let difference_value = first_half[i] - second_half[i];
        difference_vec.push(difference_value.abs());
    }

    let mut sum: i32 = 0;
    for elem in difference_vec {
        sum = sum + elem;
    }

    println!("{}", sum);

    ///////////////////

    // Part B
    let mut occurance_vec: Vec<i32> = Vec::new();
    for i in first_half {
        let mut occurance = 0;
        for j in &second_half {
            if i == *j {
                occurance += 1;
            }
        }
        occurance_vec.push(i * occurance);
    }

    let mut occurance_sum: i32 = 0;
    for i in occurance_vec {
        occurance_sum += i;
    }
    println!("{}", occurance_sum);
}
