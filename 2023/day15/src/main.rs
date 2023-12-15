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
    part_1: usize,
    part_2: usize,
}

fn hash(s: &str) -> usize {
    let mut current = 0;
    for c in s.chars() {
        if !c.is_ascii() {
            panic!("Not an ascii char.");
        }
        current += c as usize;
        current *= 17;
        current = current % 256;
    }

    current
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    for step in inp[0].split(",") {
        res.part_1 += hash(step);
    }
}
