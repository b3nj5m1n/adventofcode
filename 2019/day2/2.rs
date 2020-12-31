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
    part_1: i64,
    part_2: i64,
}

fn run_program<'a>(mut program: Vec<i64>) -> Vec<i64> {
    let mut instruction_pointer = 0;
    while program[instruction_pointer] != 99 {
        let num_of_values = 4;
        let mut params = Vec::new();
        for i in 0..num_of_values {
            params.push(program[instruction_pointer+i+1] as usize);
        }
        if program[instruction_pointer] == 1 {
            // Add the numbers together
            program[params[2]] = program[params[0]] + program[params[1]];
        }
        else {
            // Multiply the numbers together
            program[params[2]] = program[params[0]] * program[params[1]];
        }
        instruction_pointer += num_of_values;
    }
    return program;
}

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    let mut program = Vec::new();
    // Parse the input into a list of intergers
    for line in inp.split(",") {
        program.push(line.trim().parse::<i64>().unwrap());
    }
    // "To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2."
    program[1] = 12;
    program[2] = 2;
    res.part_1 = run_program(program.clone())[0];
    // Loop over all possible values for positions 1 & 2
    for i in 0..100 {
        for j in 0..100 {
            program[1] = i;
            program[2] = j;
            if run_program(program.clone())[0] == 19690720 {
                res.part_2 = 100 * i + j;
            }
        }
    }
}
