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
        part_1: String::from(""),
        part_2: 0,
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: i32,
}

const PATTERN: [&'static i32; 4] = [&0, &1, &0, &-1];

fn truncate_i32(x: i32) -> i32 {
    x.to_string().chars().last().unwrap().to_digit(10).unwrap() as i32
}

fn get_pattern(i: usize) -> impl Iterator<Item = i32> {
    let mut result = Vec::with_capacity(i * 4);
    result = PATTERN.iter().flat_map(|x| std::iter::repeat(**x).take(i)).collect();
    result.into_iter().cycle().skip(1)
}

fn phase(signal: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(signal.capacity());

    for i in 0..signal.len() {
        let mut pattern = get_pattern(i+1);
        result.push(truncate_i32(
            signal
                .iter()
                .enumerate()
                .map(|(i, x)| x * pattern.next().unwrap())
                .sum::<i32>(),
        ));
    }

    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let input_signal: Vec<i32> = inp[0]
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();
    let mut last_phase = input_signal;
    for _ in 0..100 {
        last_phase = phase(last_phase);
    }
    res.part_1 = last_phase.into_iter().map(|x| std::char::from_digit(x as u32, 10).unwrap()).take(8).collect();
}
