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
    part_2: i32,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut fish: Vec<Fish> = inp[0].split(",").map(|t| Fish { timer: t.parse::<u8>().unwrap() }).collect();
    let iterations = 80;
    for i in 0..iterations {
        fish = day(fish);
    }
    res.part_1 = fish.len();
}

fn day(fish: Vec<Fish>) -> Vec<Fish> {
    let mut result = Vec::new();
    for f in fish {
        if f.timer == 0 {
            result.push(Fish { timer: 6 });
            result.push(Fish { timer: 8 });
        } else {
            result.push(Fish { timer: f.timer - 1 });
        }
    }
    result
}

#[derive(Debug)]
struct Fish {
    timer: u8
}
