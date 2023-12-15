use std::collections::{HashMap, VecDeque};
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

fn hash(s: &str) -> usize {
    let mut current = 0;
    for c in s.chars() {
        if !c.is_ascii() {
            panic!("Not an ascii char.");
        }
        current += c as usize;
        current *= 17;
        current = current % 256;
    }

    current
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut map: Vec<Vec<_>> = vec![Vec::new(); 256]; // Vec::with_capacity(256);
    for step in inp[0].split(",") {
        res.part_1 += hash(step);
        if step.contains("-") {
            let id = step.split_once("-").unwrap().0;
            let the_box = hash(id);
            // println!("{step}: {the_box}");
            if let Some((idx_in_box, _)) = map[the_box]
                .iter()
                .enumerate()
                .find(|(idx, (box_id, _))| box_id == &id)
            {
                map[the_box].remove(idx_in_box);
            }
        } else if step.contains("=") {
            let (id, value) = step.split_once("=").unwrap();
            let the_box = hash(id);
            // println!("{step}: {the_box}");
            let idx_in_box = map[the_box]
                .iter()
                .enumerate()
                .find(|(idx, (box_id, _))| box_id == &id);
            match idx_in_box {
                Some((idx, _)) => {
                    // map[the_box].remove(idx);
                    map[the_box][idx] = (id, value);
                }
                None => {
                    map[the_box].push((id, value));
                }
            }
        } else {
            unreachable!()
        }
    }
    for i in 0..256 {
        if !map[i].is_empty() {
            // dbg!(i, &map[i]);
            for (j, lens) in map[i].iter().enumerate() {
                let focal_length = lens.1.parse::<usize>().expect("Couldn't parse as number");
                // println!("{}: (box {i}) * {j} * {focal_length}", lens.0);
                res.part_2 += (1 + i) * (1 + j) * focal_length;
            }
        }
    }
}

// Rank 802 🦀
