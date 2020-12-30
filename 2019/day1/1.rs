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

fn calc_fuel(mass: i32) -> (i32, i32) {
    let fuel_initial = (mass / 3 as i32) - 2;
    let mut fuel_recursive = fuel_initial;
    let mut mass = fuel_recursive;
    loop {
        let fuel = (mass / 3 as i32) - 2;
        if fuel <= 0 {
            break;
        }
        fuel_recursive += fuel;
        mass = fuel;
    }
    return (fuel_initial, fuel_recursive);
}

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    for line in inp.split("\n") {
        if line == "" { continue }
        let mass: i32 = line.parse().unwrap();
        let fuel = calc_fuel(mass);
        res.part_1 += fuel.0;
        res.part_2 += fuel.1;
    }
}
