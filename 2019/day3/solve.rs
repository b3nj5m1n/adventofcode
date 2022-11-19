use std::env;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

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
        part_1: -1,
        part_2: -1,
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

struct Position {
    x: i32,
    y: i32,
}

fn manhattan_distance(pos: (i32, i32)) -> i32 {
    pos.0.abs() + pos.1.abs()
}

fn get_points(instructions_str: &str) -> HashMap<(i32, i32), i32> {
    let mut moves = HashMap::new();
    moves.insert('R', (1, 0));
    moves.insert('L', (-1, 0));
    moves.insert('U', (0, 1));
    moves.insert('D', (0, -1));
    let mut result = HashMap::new();
    let instructions = instructions_str.split(",");
    // Starting position
    let mut current_position: Position = Position { x: 0, y: 0 };
    let mut steps = 0;
    // Loop over instructions for the first path
    for instruction in instructions {
        if instruction == "" { continue }
        // An iterator over the chars in the instruction string, which looks like this: "R100"
        let mut iterator = instruction.chars();
        // Get the first char from this iterator, the direction, for example 'R'
        let direction = iterator.nth(0).unwrap();
        // The maths associated with a move in that direction, represented by what has to be added to the x and y component of the current position
        let m = moves[&direction];
        // How many times this move has to be repeated, for example 100 times
        let n: i32 = iterator.as_str().parse().unwrap();
        for _ in 0..n {
            current_position.x += m.0;
            current_position.y += m.1;
            steps += 1;
            // Insert the current position into the HashSet
            result.entry((current_position.x, current_position.y)).or_insert(steps);
        }
    }
    return result;
}

// Function to solve both parts
fn solve(inp: String, res: &mut Result) {
    // Create a vector with the lines of the input
    let paths = inp.split("\n").collect::<Vec<&str>>();
    // Vectors with all points you can land on for a path, as well as the number of steps it took to get there
    let points_1 = get_points(paths[0]);
    let points_2 = get_points(paths[1]);
    // Get the intersections beteween the two HashSets (The elements they both have in common)
    let points_1_set: HashSet<(i32, i32)> = points_1.keys().cloned().collect();
    let points_2_set: HashSet<(i32, i32)> = points_2.keys().cloned().collect();
    let intersection = points_1_set.intersection(&points_2_set);
    for point in intersection {
        // Calculate the manhattan distance and see if it's closer to the starting point
        let md = manhattan_distance(*point);
        if md < res.part_1 {
            res.part_1 = md;
        }
        else if res.part_1 == -1 {
            res.part_1 = md;
        }
        // Calculate the number of steps required for each path
        let sp = points_1[point] + points_2[point];
        if sp < res.part_2 {
            res.part_2 = sp;
        }
        else if res.part_2 == -1 {
            res.part_2 = sp;
        }
    }
}
