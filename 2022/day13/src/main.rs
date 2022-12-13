use std::env;
use std::io::Read;

use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, one_of};
use nom::combinator::{map_res, recognize};
use nom::error::dbg_dmp;
use nom::multi::{many1, separated_list0};
use nom::sequence::delimited;
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
        part_1: 0,
        part_2: 1,
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

fn number(input: &str) -> IResult<&str, Element> {
    let (input, number) = map_res(recognize(many1(one_of("0123456789"))), |out: &str| {
        u32::from_str_radix(&out, 10)
    })(input)?;
    Ok((input, Element::Int(number)))
}

fn list(input: &str) -> IResult<&str, Element> {
    let (input, list) = delimited(
        tag("["),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            alt((list, number)),
        ),
        tag("]"),
    )(input)?;
    Ok((input, Element::List(list)))
}

#[derive(Debug)]
enum Element {
    Int(u32),
    List(Vec<Element>),
}

#[derive(Debug)]
enum CResult {
    InOrder,
    NotInOrder,
    Unsure,
}

fn compare_lists(l1: &Vec<Element>, l2: &Vec<Element>) -> CResult {
    let fallback = l1.len() <= l2.len();
    let same = l1.len() == l2.len();
    for pair in l1.into_iter().zip(l2.into_iter()) {
        match pair {
            (Element::Int(x), Element::Int(y)) => {
                if y < x {
                    return CResult::NotInOrder;
                } else if y != x {
                    return CResult::InOrder;
                }
            }
            (Element::List(x), Element::List(y)) => {
                let res = compare_lists(x, y);
                if let CResult::Unsure = res {
                } else {
                    return res;
                }
            }
            (Element::Int(x), Element::List(y)) => {
                let res = compare_lists(&vec![Element::Int(*x)], y);
                if let CResult::Unsure = res {
                } else {
                    return res;
                }
            }
            (Element::List(x), Element::Int(y)) => {
                let res = compare_lists(x, &vec![Element::Int(*y)]);
                if let CResult::Unsure = res {
                } else {
                    return res;
                }
            }
        }
    }
    if !fallback {
        return CResult::NotInOrder;
    } else {
        if !same {
            return CResult::InOrder;
        } else {
            return CResult::Unsure;
        }
    }
}

fn sorted_next(sorted: &mut Vec<usize>, packets: &Vec<Vec<Element>>) {
    for (i, packet) in packets.iter().enumerate() {
        if !sorted.contains(&i) {
            let mut is_next = true;
            for (j, packet2) in packets.iter().enumerate() {
                if !sorted.contains(&j) {
                    if let CResult::NotInOrder = compare_lists(packet, packet2) {
                        is_next = false;
                    }
                }
            }
            if is_next {
                sorted.push(i);
            }
        }
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut packets = Vec::new();
    for line in inp.clone() {
        if let Element::List(l) = list(line).unwrap().1 {
            packets.push(l);
        }
    }
    for (i, x) in packets.chunks(2).enumerate() {
        if let CResult::InOrder = compare_lists(&x[0], &x[1]) {
            res.part_1 += i + 1;
        }
    }
    for line in ["[[2]]", "[[6]]"] {
        if let Element::List(l) = list(line).unwrap().1 {
            packets.push(l);
        }
    }
    let mut sorted = Vec::new();
    while sorted.len() < packets.len() {
        sorted_next(&mut sorted, &packets);
    }
    for (i, j) in sorted.into_iter().enumerate() {
        if j >= inp.len() {
            res.part_2 *= i+1;
        }
    }
}
