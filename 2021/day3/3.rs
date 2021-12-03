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

// Convert vector containing single digits of a binary number to decimal
fn binary_to_decimal(vector: &Vec<u8>) -> usize {
    let mut acc = 0;
    let mut pow = 1;
    for x in vector.iter().rev() {
        acc += pow * usize::from(*x);
        pow *= 2;
    }
    acc
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let width = inp[0].len();
    let height = inp.len();
    let flattened_array: Vec<usize> = inp
        .iter()
        .flat_map(|num| num.chars())
        .filter_map(|c| match c {
            '0' => Some(0),
            '1' => Some(1),
            _ => None,
        })
        .collect();
    let mut rate_gamma: Vec<u8> = Vec::new();
    for i in 0..width {
        let row_ones: usize = usize::from(
            flattened_array
                .iter()
                .enumerate()
                .filter(|&(j, _)| j % width == i)
                .map(|(_, e)| e)
                .sum::<usize>(),
        );
        let row_zeroes = height - usize::from(row_ones);
        if row_ones > row_zeroes {
            rate_gamma.push(1);
        } else {
            rate_gamma.push(0);
        }
    }
    let rate_epsilon: Vec<u8> = rate_gamma
        .iter()
        .map(|bit| if bit == &0 { 1 } else { 0 })
        .collect();
    res.part_1 = binary_to_decimal(&rate_gamma) * binary_to_decimal(&rate_epsilon);
}
