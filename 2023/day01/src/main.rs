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
        part_2: String::from(""),
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: String,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut nums = Vec::new();
    for line in inp {
        let numbers: Vec<_> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        let num = 10 * numbers.first().expect("No numbers found on line")
            + numbers.last().expect("No numbers found on line");
        nums.push(num);
        // println!("{:?}", num)
    }
    res.part_1 = nums.iter().sum::<u32>().to_string();
}
