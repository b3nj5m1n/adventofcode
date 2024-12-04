use std::env;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::digit1;
use nom::combinator::{map_res, not, peek};
use nom::sequence::delimited;
use nom::{IResult, Parser};

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
enum Instruction {
    Mul { a: u64, b: u64 },
    Enable,
    Disable,
}

fn parse_instruction_mul_content(input: &str) -> IResult<&str, (u64, u64)> {
    let (input, a) = map_res(digit1, u64::from_str)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = map_res(digit1, u64::from_str)(input)?;

    Ok((input, (a, b)))
}

fn try_parse_instruction_mul(input: &str) -> IResult<&str, Option<Instruction>> {
    let (input, _) = take_until("mul")(input)?;
    let (input, _) = tag("mul")(input)?;
    let (input, instruction) =
        match peek(delimited(tag("("), parse_instruction_mul_content, tag(")")))(input) {
            Ok((input, (a, b))) => (input, Some(Instruction::Mul { a, b })),
            Err(_) => (input, None),
        };
    Ok((input, instruction))
}

fn try_parse_instruction_conditional(input: &str) -> IResult<&str, Option<Instruction>> {
    let (input, _) = take_until("do")(input)?;
    let (input, t) = alt((tag("don't"), tag("do")))(input)?;
    let (input, _) = tag("()")(input)?;
    Ok((
        input,
        match t {
            "don't" => Some(Instruction::Disable),
            "do" => Some(Instruction::Enable),
            _ => unreachable!(),
        },
    ))
}

fn try_parse_instruction(input: &str) -> IResult<&str, Option<Instruction>> {
    let index_mul = input.find("mul");
    let index_do = input.find("do");
    match (index_mul, index_do) {
        (None, None) => Ok((input, None)),
        (None, Some(_)) => try_parse_instruction_conditional(input),
        (Some(_), None) => try_parse_instruction_mul(input),
        (Some(m), Some(d)) => {
            let result_m = try_parse_instruction_mul(input);
            let result_d = try_parse_instruction_conditional(input);
            match (result_m, result_d) {
                (Ok((input_m, i_m)), Ok((input_d, i_d))) => if m.lt(&d) {
                    Ok((input_m, i_m))
                } else {
                    Ok((input_d, i_d))
                },
                (Ok((input, i)), Err(_)) => Ok((input, i)),
                (Err(_), Ok((input, i))) => Ok((input, i)),
                (Err(_), Err(e)) => Err(e),
            }
        }
    }
}

fn count(instructions: Vec<Instruction>) -> (u64, u64) {
    let mut enabled = true;
    let mut tally_1 = 0;
    let mut tally_2 = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Enable => { enabled = true; },
            Instruction::Disable => { enabled = false },
            Instruction::Mul { a, b } => {
                let prod = a * b;
                tally_1 += prod;
                if enabled {
                    tally_2 += prod;
                }
            },
        }
    }
    (tally_1, tally_2)
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut input = inp.concat();
    let mut instructions = Vec::new();
    while input.contains("mul") {
        let (new_input, instruction) = match try_parse_instruction(input.as_str()) {
            Ok((new_input, instruction)) => (new_input, instruction),
            Err(_) => break,
        };
        if let Some(instruction) = instruction {
            instructions.push(instruction);
        }
        input = new_input.to_string();
    }
    (res.part_1, res.part_2) = count(instructions);
}
