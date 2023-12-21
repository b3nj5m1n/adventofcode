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

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut walls = HashSet::new();
    let mut start = None;
    let height = inp.len() - 1;
    let width = inp[0].len() - 1;
    for (y, line) in inp.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let current = Point {
                x: x as i64,
                y: y as i64,
            };
            if c == '#' {
                walls.insert(current);
            }
            if c == 'S' {
                start = Some(current);
            }
        }
    }
    let start = start.expect("Failed to find starting point");
    let mut currents = HashSet::new();
    currents.insert(start);
    let steps = 6;
    let steps = 64;
    for _ in 0..steps {
        let to_consider = currents.clone();
        currents.drain();
        for point in to_consider {
            for neighbor in get_neighbours(&point, &walls, height, width) {
                currents.insert(neighbor);
            }
        }
    }
    dbg!(currents.len());
}

fn get_neighbours(
    current: &Point,
    walls: &HashSet<Point>,
    height: usize,
    width: usize,
) -> Vec<Point> {
    let mut result = Vec::new();
    for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let neighbor = Point {
            x: current.x + offset.0,
            y: current.y + offset.1,
        };
        if neighbor.x < 0
            || neighbor.y < 0
            || neighbor.x as usize > width
            || neighbor.y as usize > height
        {
            continue;
        }
        if walls.contains(&neighbor) {
            continue;
        }
        result.push(neighbor);
    }
    result
}
