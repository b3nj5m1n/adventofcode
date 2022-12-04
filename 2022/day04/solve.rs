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
    let inp: Vec<&str> = inp.split("\n").filter(|line| !line.is_empty()).collect();

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
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut pairs = Vec::new();
    for line in inp {
        let y = line
            .split(",")
            .map(|s| {
                s.split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        pairs.push((
            std::ops::Range {
                start: y[0][0],
                end: y[0][1] + 1,
            },
            std::ops::Range {
                start: y[1][0],
                end: y[1][1] + 1,
            },
        ));
    }
    for pair in pairs.clone() {
        let mut fully_contains_a = true;
        let mut some_contains_a = false;
        for x in pair.0.clone() {
            if !pair.1.contains(&x) {
                fully_contains_a = false;
            } else {
                some_contains_a = true;
            }
        }
        let mut fully_contains_b = true;
        let mut some_contains_b = false;
        for x in pair.1.clone() {
            if !pair.0.contains(&x) {
                fully_contains_b = false;
            } else {
                some_contains_b = true;
            }
        }
        if fully_contains_a || fully_contains_b {
            res.part_1 += 1;
        }
        if some_contains_a || some_contains_b {
            res.part_2 += 1;
        }
    }
}
