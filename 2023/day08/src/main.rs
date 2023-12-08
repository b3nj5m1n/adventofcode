use std::collections::HashMap;
use std::io::Read;
use std::{env, iter};

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
    part_1: u64,
    part_2: u64,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let instructions = inp[0];

    let mut map = HashMap::new();
    for line in inp[1..].iter() {
        let (id, lr) = line.split_once(" = ").expect("Parsing failed");
        let lr = lr
            .strip_suffix(")")
            .expect("Parsing failed")
            .strip_prefix("(")
            .expect("Parsing failed");
        let (left, right) = lr.split_once(", ").expect("Parsing failed");
        map.insert(id, (left, right));
    }

    // Part 1
    let mut current = "AAA";
    let mut i = 0;
    for instruction in iter::repeat_with(|| instructions.chars()).flatten() {
        if current == "ZZZ" {
            break;
        }
        i = i + 1;
        match instruction {
            'R' => {
                current = map
                    .get(current)
                    .expect("Didn't find current element in map")
                    .1
            }
            'L' => {
                current = map
                    .get(current)
                    .expect("Didn't find current element in map")
                    .0
            }
            _ => unreachable!(),
        }
    }
    res.part_1 = i;

    // Part 2
    // Get all elements starting with 'A'
    let currents = map
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|k| *k)
        .collect::<Vec<_>>();
    // Find Cycles for each element (How long does it take it to end up on "Z")
    let ends_up_on_z = currents.clone().into_iter().map(|current| {
        let mut current = current;
        let mut i = 0;
        for instruction in iter::repeat_with(|| instructions.chars()).flatten() {
            if current.ends_with("Z") {
                break;
            }
            i = i + 1;
            match instruction {
                'R' => {
                    current = map
                        .get(current)
                        .expect("Didn't find current element in map")
                        .1
                }
                'L' => {
                    current = map
                        .get(current)
                        .expect("Didn't find current element in map")
                        .0
                }
                _ => unreachable!(),
            }
        }
        i
    }).collect::<Vec<_>>();
    // Find the lowest common multiple of all cycles
    res.part_2 = ends_up_on_z.into_iter().fold(1, |a,b| num::integer::lcm(a,b));
    // How did I come up with this at 6 am?
}
