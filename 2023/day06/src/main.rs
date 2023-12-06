use std::env;
use std::io::Read;
use std::str::FromStr;

use anyhow;

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

struct Race {
    time: usize,
    dist: usize,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let times = inp[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let dists = inp[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut races = Vec::new();
    for (time, dist) in times.zip(dists) {
        races.push(Race { time, dist })
    }
    let mut all_possibilities: Vec<usize> = Vec::new();
    for race in races {
        let mut possibilities = 0;
        for ms in 1..race.time {
            let speed = 1 * ms;
            let dist = speed * (race.time - ms);
            if dist > race.dist {
                possibilities += 1;
            }
        }
        all_possibilities.push(possibilities);
    }
    res.part_1 = all_possibilities.iter().fold(1, |a, b| a * b);

    // Part 2
    let the_time: usize = inp[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let the_dist: usize = inp[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();

    let mut possibilities = Vec::new();
    for ms in 1..the_time {
        let speed = 1 * ms;
        let dist = speed * (the_time - ms);
        if dist > the_dist {
            possibilities.push(ms);
        }
    }
    res.part_2 = possibilities.len();
}
