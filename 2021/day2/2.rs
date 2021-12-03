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

#[derive(Debug)]
enum Direction {
    Forward,
    Up,
    Down,
}

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    let input: Vec<(Direction, i32)> = inp
        .split("\n")
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut split = line.split(" ");
            let direction = split.next().unwrap();
            let units = split.next().unwrap().parse::<i32>().unwrap();
            match direction {
                "forward" => return Some((Direction::Forward, units)),
                "down" => return Some((Direction::Down, units)),
                "up" => return Some((Direction::Up, units)),
                _ => return None,
            }
        })
        .collect();
    let mut position_horizontal = 0;
    let mut aim = 0;
    let mut position_depth_1 = 0;
    let mut position_depth_2 = 0;
    for command in input {
        match command.0 {
            Direction::Forward => {
                position_horizontal += command.1;
                position_depth_2 += aim * command.1
            }
            Direction::Up => {
                position_depth_1 -= command.1;
                aim -= command.1
            }
            Direction::Down => {
                position_depth_1 += command.1;
                aim += command.1
            }
        }
    }
    res.part_1 = position_depth_1 * position_horizontal;
    res.part_2 = position_depth_2 * position_horizontal;
}
