use std::collections::HashMap;
use std::env;
use std::io::Read;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

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
struct Claim {
    id: u32,
    offset_left: u32,
    offset_top: u32,
    width: u32,
    height: u32,
}

fn parse_claim(input: &str) -> IResult<&str, Claim> {
    let (input, _) = tag("#")(input)?;
    let (input, id) = map_res(digit1, str::parse)(input)?;
    let (input, _) = separated_pair(multispace0, tag("@"), multispace0)(input)?;
    let (input, (offset_left, offset_top)) = separated_pair(
        map_res(digit1, str::parse),
        tag(","),
        map_res(digit1, str::parse),
    )(input)?;
    let (input, _) = separated_pair(multispace0, tag(":"), multispace0)(input)?;
    let (input, (width, height)) = separated_pair(
        map_res(digit1, str::parse),
        tag("x"),
        map_res(digit1, str::parse),
    )(input)?;
    Ok((
        input,
        Claim {
            id,
            offset_left,
            offset_top,
            width,
            height,
        },
    ))
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid = HashMap::new();
    for line in inp {
        let (_, claim) = parse_claim(line).expect("Parsing failed");
        for x in claim.offset_left..(claim.offset_left + claim.width) {
            for y in claim.offset_top..(claim.offset_top + claim.height) {
                grid.entry((x, y))
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }
    for count in grid.values() {
        if *count >= 2 {
            res.part_1 += 1;
        }
    }
}
