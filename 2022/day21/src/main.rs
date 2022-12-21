// I tested positive for COVID yesterday and I'm feeling absolutely horrible, I can barely focus so
// I might stick to just doing part 1 until I'm feeling better.
use std::collections::HashMap;
use std::env;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, multispace0};
use nom::error::ParseError;
use nom::sequence::{delimited, terminated};
use nom::IResult;

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
struct MonkeyYell<'a> {
    name: &'a str,
    number: i64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MonkeyResult<'a> {
    name: &'a str,
    operation: Operation,
    number_a: Option<i64>,
    number_b: Option<i64>,
    monkey_a: Option<&'a str>,
    monkey_b: Option<&'a str>,
}

fn parse_monkey_yell(input: &str) -> IResult<&str, MonkeyYell> {
    let (input, name) = terminated(alpha1, tag(":"))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, number) = nom::character::complete::i64(input)?;

    Ok((input, MonkeyYell { name, number }))
}

fn parse_monkey_result(input: &str) -> IResult<&str, MonkeyResult> {
    let (input, name) = terminated(alpha1, tag(":"))(input)?;
    let (input, _) = multispace0(input)?;

    let (input, number_a) = if let Ok((new_input, number)) =
        nom::character::complete::i64::<&str, nom::error::Error<&str>>(input)
    {
        (new_input, Some(number))
    } else {
        (input, None)
    };
    let (input, monkey_a) = if number_a.is_none() {
        let (new_input, monkey_name) = alpha1(input)?;
        (new_input, Some(monkey_name))
    } else {
        (input, None)
    };

    let (input, operation_str) = delimited(
        multispace0,
        alt((tag("+"), tag("-"), tag("*"), tag("/"))),
        multispace0,
    )(input)?;
    let operation = match operation_str {
        "+" => Operation::Add,
        "-" => Operation::Sub,
        "*" => Operation::Mul,
        "/" => Operation::Div,
        _ => panic!("Unkown operation"),
    };

    let (input, number_b) = if let Ok((new_input, number)) =
        nom::character::complete::i64::<&str, nom::error::Error<&str>>(input)
    {
        (new_input, Some(number))
    } else {
        (input, None)
    };
    let (input, monkey_b) = if number_a.is_none() {
        let (new_input, monkey_name) = alpha1(input)?;
        (new_input, Some(monkey_name))
    } else {
        (input, None)
    };

    Ok((
        input,
        MonkeyResult {
            name,
            number_a,
            number_b,
            monkey_a,
            monkey_b,
            operation,
        },
    ))
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut monkeys_result = Vec::new();
    let mut monkeys_yell = HashMap::new();
    for line in inp {
        if let Ok((input, monkey)) = parse_monkey_result(line) {
            monkeys_result.push(monkey);
        }
        if let Ok((input, monkey)) = parse_monkey_yell(line) {
            monkeys_yell.insert(monkey.name, monkey.number);
        }
    }
    /* println!("Result length: {}", monkeys_result.len());
    println!("Yell length: {}", monkeys_yell.len()); */
    while monkeys_result.len() > 0 {
        let mut to_remove = Vec::new();
        for monkey in monkeys_result.iter() {
            if let Some(m_1) = monkey.monkey_a {
                if let Some(m_2) = monkey.monkey_b {
                    if monkeys_yell.contains_key(m_1) && monkeys_yell.contains_key(m_2) {
                        let result = match monkey.operation {
                            Operation::Add => {
                                monkeys_yell.get(m_1).unwrap() + monkeys_yell.get(m_2).unwrap()
                            }
                            Operation::Sub => {
                                monkeys_yell.get(m_1).unwrap() - monkeys_yell.get(m_2).unwrap()
                            }
                            Operation::Mul => {
                                monkeys_yell.get(m_1).unwrap() * monkeys_yell.get(m_2).unwrap()
                            }
                            Operation::Div => {
                                monkeys_yell.get(m_1).unwrap() / monkeys_yell.get(m_2).unwrap()
                            }
                        };
                        monkeys_yell.insert(monkey.name, result);
                        to_remove.push(monkey.clone());
                    }
                }
            }
        }
        for monkey in to_remove.into_iter() {
            monkeys_result.remove(monkeys_result.iter().position(|&m| m == monkey).unwrap());
        }
    }
    /* println!("Result length: {}", monkeys_result.len());
    println!("Yell length: {}", monkeys_yell.len()); */
    res.part_1 = monkeys_yell.get("root").unwrap().to_string();
}
