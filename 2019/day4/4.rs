use std::env;
use std::io::Read;

// Function to output the solutions to both parts
fn output(result: &Result) {
    println!("Part 1: {}", &result.part_1);
    println!("Part 2: {}", &result.part_2);
}

fn main() {
    // Vector of the command line arguments
    let args: Vec<String> = env::args().collect();

    // Read in the input
    let mut file_handle = std::fs::File::open(&args[1]).unwrap();
    let mut inp = String::new();
    file_handle.read_to_string(&mut inp).unwrap();

    // Struct storing the resulting values
    let mut result: Result = Result {
        part_1: 0,
        part_2: 0,
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: i32,
    part_2: i32,
}

fn is_valid(password: i32) -> (bool, bool) {
    let password_arr: Vec<i64> = password
        .to_string()
        .chars()
        .map(|c| i64::from(c.to_digit(10).unwrap()))
        .collect();
    // It is a six-digit number.
    if password_arr.len() != 6 {
        return (false, false);
    }
    let mut valid_part_1 = false;
    let mut valid_part_2 = false;
    for i in 1..password_arr.len() {
        // Two adjacent digits are the same (like 22 in 122345).
        if password_arr[i - 1] == password_arr[i] {
            valid_part_1 = true;
            // the two adjacent matching digits are not part of a larger group of matching digits.
            let prev = if i >= 2 { password_arr[i - 2] } else { -1 };
            let next = if i + 1 < password_arr.len() {
                password_arr[i + 1]
            } else {
                -1
            };
            if prev != password_arr[i] && next != password_arr[i] {
                valid_part_2 = true
            }
        }
        // Going from left to right, the digits never decrease; they only ever increase or stay the same (like 111123 or 135679).
        if password_arr[i] < password_arr[i - 1] {
            return (false, false);
        }
    }
    return (valid_part_1, valid_part_2);
}

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    let range: Vec<&str> = inp.split("-").collect();
    let start: i32 = range[0].trim().parse::<i32>().unwrap();
    let end: i32 = range[1].trim().parse::<i32>().unwrap();
    for i in start..end {
        let validity = is_valid(i);
        if validity.0 {
            res.part_1 += 1;
        }
        if validity.1 {
            res.part_2 += 1;
        }
    }
}
