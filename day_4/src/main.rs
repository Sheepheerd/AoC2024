use std::fs::read_to_string;
fn main() {
    // Part A
    //let s = read_lines("input");
    //
    //let num_xmas = detect_xmas(s);
    //println!("Number of XMAS found: {}", num_xmas);

    // Part B

    let s = read_lines("input");

    let num_xmas = detect_x_mas(s);
    println!("Number of XMAS found: {}", num_xmas);
}

fn detect_x_mas(s: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            // if value is == 'M' then check if s[i+1][j+1] == "A   " and s[i+2][j+2] == "S   "
            if s[i][j] == 'M' {
                // Both M's are on left
                if j < s[j].len() - 2
                    && i < s.len() - 2
                    && s[i + 1][j + 1] == 'A'
                    && s[i + 2][j + 2] == 'S'
                    && s[i][j + 2] == 'S'
                    && s[i + 2][j] == 'M'
                {
                    count += 1;
                }

                // Both M's are on the top
                if j < s[j].len() - 2
                    && i < s.len() - 2
                    && s[i + 1][j + 1] == 'A'
                    && s[i + 2][j + 2] == 'S'
                    && s[i][j + 2] == 'M'
                    && s[i + 2][j] == 'S'
                {
                    count += 1;
                }
            }
            if s[i][j] == 'S' {
                // Both S's are on top
                if j < s[j].len() - 2
                    && i < s.len() - 2
                    && s[i + 1][j + 1] == 'A'
                    && s[i + 2][j + 2] == 'M'
                    && s[i][j + 2] == 'S'
                    && s[i + 2][j] == 'M'
                {
                    count += 1;
                }

                // Both S's are on the left side
                if j < s[j].len() - 2
                    && i < s.len() - 2
                    && s[i + 1][j + 1] == 'A'
                    && s[i + 2][j + 2] == 'M'
                    && s[i][j + 2] == 'M'
                    && s[i + 2][j] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn detect_xmas(s: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..s.len() {
        for j in 0..s[i].len() {
            // if value is == 'X' then check all 8 directions for 'X' 'M' 'A' 'S'
            if s[i][j] == 'X' {
                //for s in &s {
                //    println!("{}", s.iter().collect::<String>());
                //}
                //println!("{} {}", i, j);
                //println!("{}", s[i][j]);

                // check north
                if i > 2 && s[i - 1][j] == 'M' && s[i - 2][j] == 'A' && s[i - 3][j] == 'S' {
                    count += 1;
                }
                // check south
                if i < s.len() - 3 && s[i + 1][j] == 'M' && s[i + 2][j] == 'A' && s[i + 3][j] == 'S'
                {
                    count += 1;
                }
                // check east
                if j < s[j].len() - 3
                    && s[i][j + 1] == 'M'
                    && s[i][j + 2] == 'A'
                    && s[i][j + 3] == 'S'
                {
                    count += 1;
                }
                // check west
                if j > 2 && s[i][j - 1] == 'M' && s[i][j - 2] == 'A' && s[i][j - 3] == 'S' {
                    count += 1;
                }
                // check south east
                if i < s.len() - 3
                    && j < s[j].len() - 3
                    && s[i + 1][j + 1] == 'M'
                    && s[i + 2][j + 2] == 'A'
                    && s[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }
                // check south west
                if i < s.len() - 3
                    && j > 2
                    && s[i + 1][j - 1] == 'M'
                    && s[i + 2][j - 2] == 'A'
                    && s[i + 3][j - 3] == 'S'
                {
                    count += 1;
                }
                // check north east
                if i > 2
                    && j < s[j].len() - 3
                    && s[i - 1][j + 1] == 'M'
                    && s[i - 2][j + 2] == 'A'
                    && s[i - 3][j + 3] == 'S'
                {
                    count += 1;
                }
                // check north west
                if i > 2
                    && j > 2
                    && s[i - 1][j - 1] == 'M'
                    && s[i - 2][j - 2] == 'A'
                    && s[i - 3][j - 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }
    return count;
}

fn read_lines(filename: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(take_string(line.to_string()));
    }

    result
}

fn take_string(input: String) -> Vec<char> {
    // Convert input to vector of characters
    let vec: Vec<char> = input.trim().chars().collect();

    // Return vector
    return vec;
}
