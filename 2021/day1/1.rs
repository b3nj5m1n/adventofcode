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

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    let lines: Vec<i32> = inp
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|number| {
            return number.parse::<i32>().unwrap();
        })
        .collect();
    for i in 1..lines.len() {
        if lines[i - 1] < lines[i] {
            res.part_1 += 1;
        }
        if i > 2 && i < lines.len() {
            let a: i32 = lines
                .iter()
                .enumerate()
                .filter(|&(j, _)| j >= (i - 3) && j < i)
                .map(|(_, e)| e)
                .sum();
            let b: i32 = lines
                .iter()
                .enumerate()
                .filter(|&(j, _)| j > (i - 3) && j <= i)
                .map(|(_, e)| e)
                .sum();
            if b > a {
                res.part_2 += 1;
            }
        }
    }
}
