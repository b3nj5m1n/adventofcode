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
    let inp: Vec<&str> = inp.split("\n").collect();

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
    part_1: i64,
    part_2: i64,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut current = 0;
    let mut elves = Vec::new();
    elves.push(0);
    for line in inp {
        if line == "" {
            current += 1;
            elves.push(0);
        } else {
            elves[current] = elves[current] + line.parse::<i64>().unwrap();
        }
    }
    elves.sort();
    let mut sorted = elves.iter().rev();
    res.part_1 = *sorted.next().unwrap();
    res.part_2 = sorted.take(2).sum::<i64>() + res.part_1;
}
