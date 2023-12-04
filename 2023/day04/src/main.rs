use std::collections::HashSet;
use std::env;
use std::io::Read;

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
    part_1: u32,
    part_2: u32,
}

#[derive(Debug)]
struct Card {
    id: u32,
    numbers_winning: HashSet<u32>,
    numbers_have: Vec<u32>,
}

impl TryFrom<&str> for Card {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (id, rest) = value
            .split_once(":")
            .ok_or(anyhow::anyhow!("Unexpected format: card"))?;
        let (_, id) = id
            .trim()
            .split_once(" ")
            .ok_or(anyhow::anyhow!("Unexpected format: id"))?;
        let id = id.trim();
        let (nums_win, nums_have) = rest
            .split_once("|")
            .ok_or(anyhow::anyhow!("Unexpected format: nums"))?;
        Ok(Self {
            id: id.parse()?,
            numbers_winning: nums_win
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("Unexpected format: nums_win"))
                .collect(),
            numbers_have: nums_have
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u32>().expect("Unexpected format: nums_win"))
                .collect(),
        })
    }
}

impl Card {
    fn value(&self) -> u32 {
        let mut wins = 0;
        for num_have in self.numbers_have.iter() {
            if self.numbers_winning.contains(&num_have) {
                wins += 1;
            }
        }
        let mut result = if wins > 0 { 1 } else { 0 };
        for _ in 1..wins {
            result *= 2;
        }
        result
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    for line in inp {
        let card: Card = line.try_into().expect("Parsing failed");
        res.part_1 += card.value();
    }
}
