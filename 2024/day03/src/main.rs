use std::env;
use std::io::Read;

use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::digit1;
use nom::combinator::{map_res, peek};
use nom::sequence::delimited;
use nom::IResult;

use std::str::FromStr;

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

#[derive(Debug, PartialEq)]
struct InstructionMul {
    a: u64,
    b: u64,
}

fn parse_instruction_mul_content(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, a) = map_res(digit1, u64::from_str)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = map_res(digit1, u64::from_str)(input)?;

    Ok((input, (a, b)))
}

fn try_parse_instruction_mul(input: &str) -> IResult<&str, Option<InstructionMul>> {
    let (input, _) = take_until("mul")(input)?;
    let (input, _) = tag("mul")(input)?;
    let (input, instruction) =
        match peek(delimited(tag("("), parse_instruction_mul_content, tag(")")))(input) {
            Ok((input, (a, b))) => (input, Some(InstructionMul { a, b })),
            Err(_) => (input, None),
        };
    Ok((input, instruction))
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut input = inp.concat();
    let mut instructions = Vec::new();
    while input.contains("mul") {
        let (new_input, instruction) = match try_parse_instruction_mul(input.as_str()) {
            Ok((new_input, instruction)) => (new_input, instruction),
            Err(_) => break,
        };
        if let Some(instruction) = instruction {
            instructions.push(instruction);
        }
        input = new_input.to_string();
    }
    /* for line in inp {
        println!("{}", line)
    } */
    res.part_1 = instructions.iter().map(|i| i.a * i.b).sum();
}
