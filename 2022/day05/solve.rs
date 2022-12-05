use std::collections::HashMap;
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
    let cranes: Vec<i32> = inp
        .iter()
        .filter(|line| !line.contains("[") && !line.contains("move") && !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                // .collect::<Vec<&str>>()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .next()
        .unwrap();
    let cranes_count = cranes.iter().count();
    let initial: Vec<&str> = inp
        .clone()
        .into_iter()
        .filter(|line| line.contains("["))
        .rev()
        .collect();
    let mut f: HashMap<&i32, Vec<char>> = HashMap::new();
    for (idx, crane) in cranes.iter().enumerate() {
        let mut k = Vec::new();
        for crates in initial.iter() {
            let c = crates.chars().skip(1).skip(idx * 4).next().unwrap();
            if !c.is_whitespace() {
                k.push(c);
            }
        }
        f.insert(crane, k);
    }

    let instructions: Vec<(i32, i32, i32)> = inp
        .into_iter()
        .filter(|line| line.contains("move"))
        .map(|line| line.split_whitespace().collect())
        .map(|vec: Vec<&str>| {
            (
                vec[1].parse::<i32>().unwrap(),
                vec[3].parse::<i32>().unwrap(),
                vec[5].parse::<i32>().unwrap(),
            )
        })
        .collect();

    let mut cranes_1 = f.clone();
    for instruction in instructions.iter() {
        for _ in 0..instruction.0 {
            let removed: char = cranes_1
                .get_mut(&instruction.1)
                .map(|val| val.pop().unwrap())
                .unwrap();
            cranes_1
                .get_mut(&instruction.2)
                .map(|val| val.push(removed));
        }
    }

    let mut cranes_2 = f.clone();
    for instruction in instructions.iter() {
        let mut stack = Vec::new();
        for _ in 0..instruction.0 {
            let removed: char = cranes_2
                .get_mut(&instruction.1)
                .map(|val| val.pop().unwrap())
                .unwrap();
            stack.push(removed);
        }
        for x in stack.into_iter().rev() {
            cranes_2
                .get_mut(&instruction.2)
                .map(|val| val.push(x));
        }
    }

    let mut result = vec![' '; cranes_count];
    for (crane, stack) in &cranes_1 {
        result[**crane as usize - 1] = stack.clone().pop().unwrap();
    }
    res.part_1 = result.iter().collect();

    let mut result = vec![' '; cranes_count];
    for (crane, stack) in &cranes_2 {
        result[**crane as usize - 1] = stack.clone().pop().unwrap();
    }
    res.part_2 = result.iter().collect();
}
