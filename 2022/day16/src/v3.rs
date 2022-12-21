// This was horrible, it took me 16 hours to get part 1, and I actually spend a significant amount
// of those 16 hours actively working on the solution. In the end, I had to resort to outside help.
// I initially had some massive misconceptions about how this problem works, which lead to me
// implementing it incorrectly. It gets even weirder though, on the example input, the result is
// off by 20, on the actual input, it produces the correct answer. I have no idea why this is and
// I'm too tired to care, I literally spend the entire day working on this. We'll see if and when I
// get to part two. I'll include my original versions, in v1 I got very close to the correct answer
// to the example, but I think that's more luck than anything else, the second version uses
// petgraph und should, in theory, be much nicer to work with, unfortunately I couldn't figure out
// which algorthms to actually use to get the answer.

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env::{self, current_dir};
use std::hash::Hash;
use std::io::Read;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space0};
use nom::character::is_newline;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, terminated};
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

#[derive(Debug, Clone)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    leads_to: Vec<&'a str>,
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ")(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, _) = tag(" has flow rate")(input)?;
    let (input, flow_rate) = delimited(tag("="), nom::character::complete::u32, tag(";"))(input)?;
    let (input, _) = alt((
        tag(" tunnels lead to valves "),
        tag(" tunnel leads to valve "),
    ))(input)?;
    let (input, leads_to) = separated_list0(terminated(tag(","), space0), alphanumeric1)(input)?;
    Ok((
        input,
        Valve {
            name,
            flow_rate,
            leads_to,
        },
    ))
}

#[derive(Eq, PartialEq, Debug)]
struct CallSig<'a> {
    current: &'a str,
    opened: HashSet<&'a str>,
    minutes_left: u32,
}

impl Hash for CallSig<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        for open in self.opened.iter() {
            open.hash(state);
        }
        /* self.opened
        .clone()
        .into_iter()
        .collect::<String>()
        .hash(state); */
        self.minutes_left.hash(state);
    }
}

fn get_max_flow<'a>(
    current: &'a str,
    opened: HashSet<&'a str>,
    minutes_left: u32,
    valves: &'a HashMap<&str, Valve>,
    memo: &'a RefCell<HashMap<CallSig<'a>, u32>>,
) -> u32 {
    // println!("{minutes_left}");
    let call_sig = CallSig {
        current,
        opened: opened.clone(),
        minutes_left,
    };
    if memo.borrow().contains_key(&call_sig) {
        // println!("Cache hit");
        return *memo.borrow().get(&call_sig).unwrap();
    }
    // println!("Cache miss");
    // dbg!(memo);
    // println!("{current}");
    if minutes_left == 0 {
        return 0;
    }
    if opened.contains(current) {
        return 0;
    }
    let mut current_best = 0;
    let current_valve = valves.get(&current).unwrap();
    let current_value = current_valve.flow_rate * (minutes_left - 1);
    let mut opened_current = opened.clone();
    opened_current.insert(current);
    for neighbour in current_valve.leads_to.iter() {
        current_best = max(
            current_best,
            get_max_flow(neighbour, opened.clone(), minutes_left - 1, valves, &memo),
        );
        if current_value != 0 {
            current_best = max(
                current_best,
                current_value
                    + get_max_flow(neighbour, opened_current.clone(), minutes_left - 2, valves, &memo),
            );
        }
    }
    memo.borrow_mut().insert(call_sig, current_best);
    current_best
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut valves = HashMap::new();
    for line in inp {
        let valve = parse_valve(line).unwrap().1;
        valves.insert(valve.name, valve);
    }
    res.part_1 = get_max_flow(
        "AA",
        HashSet::new(),
        30,
        &valves,
        &mut RefCell::new(HashMap::new()),
    )
    .to_string();
    // dbg!(valves);
}
