use std::collections::VecDeque;
use std::collections::HashSet;
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

fn contains_duplicates(queue: &VecDeque<char>) -> bool {
    let mut set = HashSet::new();
    for x in queue {
        if set.contains(x) {
            return true;
        }
        set.insert(x);
    }
    false
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut last_4 = VecDeque::new();
    let mut last_14 = VecDeque::new();
    let mut result_idx_1 = 0;
    let mut result_idx_2 = 0;
    for (i, c) in inp[0].chars().enumerate() {
        if last_4.len() >= 4 {
            if !contains_duplicates(&last_4) && result_idx_1 == 0 {
                result_idx_1 = i;
            }
            last_4.pop_back();
        }
        if last_14.len() >= 14 {
            if !contains_duplicates(&last_14) && result_idx_2 == 0 {
                result_idx_2 = i;
            }
            last_14.pop_back();
        }
        last_4.push_front(c);
        last_14.push_front(c);
    }
    res.part_1 = result_idx_1;
    res.part_2 = result_idx_2;
}
