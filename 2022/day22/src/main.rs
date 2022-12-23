// Spent _way_ too long not realizing I forgot about the indicies starting at 1 for my max_x and
// max_y calculation.
use std::collections::{HashMap, VecDeque};
use std::env;
use std::io::Read;

use nom::bytes::complete::tag;
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

#[derive(Debug)]
enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    if let Ok((input, x)) = nom::character::complete::u32::<&str, nom::error::Error<&str>>(input) {
        Ok((input, Instruction::Move(x)))
    } else if let Ok((input, _)) = tag::<&str, &str, nom::error::Error<&str>>("L")(input) {
        Ok((input, Instruction::TurnLeft))
    } else if let Ok((input, _)) = tag::<&str, &str, nom::error::Error<&str>>("R")(input) {
        Ok((input, Instruction::TurnRight))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        )))
    }
}

fn turn(facing_direction: u8, left: bool) -> u8 {
    if left {
        (facing_direction as i8 - 1).rem_euclid(4) as u8
    } else {
        (facing_direction + 1).rem_euclid(4)
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut instructions_str = inp.clone().into_iter().last().unwrap();
    let mut instructions = VecDeque::new();
    let mut current = None;
    let mut max_x = 0;
    let mut max_y = 0;
    while let Ok((rest, instruction)) = parse_instruction(instructions_str) {
        instructions_str = rest;
        instructions.push_back(instruction);
    }
    let mut grid = HashMap::new();
    for (y, line) in inp.clone().into_iter().enumerate() {
        if y == inp.len() - 1 {
            break;
        }
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                grid.insert((x + 1, y + 1), true);
                if current.is_none() {
                    current = Some((x + 1, y + 1));
                }
            } else if c == '#' {
                grid.insert((x + 1, y + 1), false);
            }
            if x + 1 > max_x {
                max_x = x + 1;
            }
        }
        if y + 1 > max_y {
            max_y = y + 1;
        }
    }
    let mut facing_direction = 0;
    let mut path = vec![current.expect("Couldn't find starting tile")];
    while instructions.len() > 0 {
        match instructions.pop_front().unwrap() {
            Instruction::TurnLeft => facing_direction = turn(facing_direction, true),
            Instruction::TurnRight => facing_direction = turn(facing_direction, false),
            Instruction::Move(count) => {
                'outer: for _ in 0..count {
                    let current = path.iter().last().unwrap().clone();
                    // Right
                    if facing_direction == 0 {
                        if let Some(walkable) = grid.get(&(current.0 + 1, current.1)) {
                            if !walkable {
                                break 'outer;
                            }
                            path.push((current.0 + 1, current.1));
                        } else {
                            'inner: for x in 1..current.0 {
                                if let Some(walkable) = grid.get(&(x, current.1)) {
                                    if !walkable {
                                        break 'outer;
                                    } else {
                                        path.push((x, current.1));
                                        break 'inner;
                                    }
                                }
                            }
                        }
                    }
                    // Down
                    if facing_direction == 1 {
                        if let Some(walkable) = grid.get(&(current.0, current.1 + 1)) {
                            if !walkable {
                                break 'outer;
                            }
                            path.push((current.0, current.1 + 1));
                        } else {
                            'inner: for y in 1..current.1 {
                                if let Some(walkable) = grid.get(&(current.0, y)) {
                                    if !walkable {
                                        break 'outer;
                                    } else {
                                        path.push((current.0, y));
                                        break 'inner;
                                    }
                                }
                            }
                        }
                    }
                    // Left
                    if facing_direction == 2 {
                        if let Some(walkable) = grid.get(&(current.0 - 1, current.1)) {
                            if !walkable {
                                break 'outer;
                            }
                            path.push((current.0 - 1, current.1));
                        } else {
                            'inner: for x in ((current.0 + 1)..=max_x).into_iter().rev() {
                                if let Some(walkable) = grid.get(&(x, current.1)) {
                                    if !walkable {
                                        break 'outer;
                                    } else {
                                        path.push((x, current.1));
                                        break 'inner;
                                    }
                                }
                            }
                        }
                    }
                    // Up
                    if facing_direction == 3 {
                        if let Some(walkable) = grid.get(&(current.0, current.1 - 1)) {
                            if !walkable {
                                break 'outer;
                            }
                            path.push((current.0, current.1 - 1));
                        } else {
                            'inner: for y in ((current.1 + 1)..=max_y).into_iter().rev() {
                                if let Some(walkable) = grid.get(&(current.0, y)) {
                                    if !walkable {
                                        break 'outer;
                                    } else {
                                        path.push((current.0, y));
                                        break 'inner;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let current = path.iter().last().unwrap().clone();
    res.part_1 = (current.1 * 1000 + 4 * current.0 + facing_direction as usize).to_string();

    /* dbg!(turn(0, true));  // Facing right, turn left,  now facing up    -> 3
    dbg!(turn(0, false)); // Facing right, turn right, now facing down  -> 1
    dbg!(turn(1, true));  // Facing down,  turn left,  now facing right -> 0
    dbg!(turn(1, false)); // Facing down,  turn right, now facing left  -> 2
    dbg!(turn(2, true));  // Facing left,  turn left,  now facing down  -> 1
    dbg!(turn(2, false)); // Facing left,  turn right, now facing up    -> 3
    dbg!(turn(3, true));  // Facing up,    turn left,  now facing left  -> 2
    dbg!(turn(3, false)); // Facing up,    turn right, now facing right -> 0 */
}
