use std::collections::{HashMap, HashSet, VecDeque};
use std::env::{self, current_dir};
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

fn get_highest_path<'a>(
    current_valve: &'a Valve,
    valves: &'a HashMap<&str, Valve>,
    valves_open: HashSet<&str>,
    visited_valves: &mut HashSet<&'a str>,
    current_path: Vec<&'a str>,
) -> Option<Vec<&'a str>> {
    let mut most_neighbour_rate = 0;
    let mut most_neighbour_name = "";
    let mut most_neighbour_path = current_path.clone();
    if visited_valves.contains(current_valve.name) {
        return None;
    }
    let mut new_path = current_path.clone();
    new_path.push(current_valve.name);
    visited_valves.insert(current_valve.name);
    for neighbour_name in current_valve.leads_to.iter() {
        let neighbour = valves.get(neighbour_name).unwrap();
        if neighbour.flow_rate > most_neighbour_rate && !valves_open.contains(neighbour_name) {
            most_neighbour_name = neighbour_name;
            most_neighbour_rate = neighbour.flow_rate;
            let mut new_path = new_path.clone();
            new_path.push(most_neighbour_name);
            most_neighbour_path = new_path.clone();
        }
        let recursive_ = get_highest_path(
            neighbour,
            valves,
            valves_open.clone(),
            visited_valves,
            new_path.clone(),
        );
        if let Some(recursive) = recursive_ {
            let rate_recursive = valves.get(recursive.last().unwrap()).unwrap().flow_rate;
            // println!("Comparing {recursive:?} to current winner: {most_neighbour_path:?}, len of {} vs {}", most_neighbour_path.len(), recursive.len());
            let score_new = rate_recursive as f64 * 2_f64 / recursive.len() as f64;
            let score_current =
                most_neighbour_rate as f64 * 2_f64 / most_neighbour_path.len() as f64;
            // dbg!(score_current, score_new);
            // if rate_recursive > most_neighbour_rate && most_neighbour_path.len() > recursive.len() {
            if score_new > score_current {
                most_neighbour_name = recursive.last().unwrap();
                most_neighbour_rate = rate_recursive;
                most_neighbour_path = recursive;
            }
        }
    }
    return if most_neighbour_name != "" && most_neighbour_path.len() > 0 {
        Some(most_neighbour_path)
    } else {
        None
    };
}

fn len_back_to_aa<'a>(current_valve: &'a str, valves: &'a HashMap<&str, Valve>, len: u32) -> u32 {
    if *current_valve == *"AA" {
        return len;
    }
    let mut queue = VecDeque::new();
    queue.push_front(current_valve);
    let mut visited = HashMap::new();
    visited.insert(current_valve, current_valve);
    let mut parent = current_valve;
    // dbg!(&visited);
    while queue.len() > 0 {
        let current = queue.pop_front().unwrap();
        visited.insert(&current, &parent);
        parent = current;
        for neighbour_name in valves.get(&current).unwrap().leads_to.iter() {
            if visited.contains_key(neighbour_name) {
                continue;
            }
            if *neighbour_name == "AA" {
                let mut x = 1;
                let mut y = *visited.get(current).unwrap();
                // dbg!(&visited, current);
                while y != *visited.get(y).unwrap() {
                    // println!("{y}");
                    y = *visited.get(y).unwrap();
                    x += 1;
                }
                return x;
            }
            queue.push_back(&neighbour_name);
        }
    }
    u32::MAX
}

fn get_highest<'a>(
    current_valve: &'a str,
    valves: &'a HashMap<&str, Valve>,
    valves_open: HashSet<&str>,
    visited_valves: &mut HashSet<&'a str>,
    len: u32,
) -> (u32, u32) {
    if visited_valves.contains(current_valve) && *current_valve != *"AA" {
        if valves_open.contains(current_valve) {
            return (0, len + 0);
        }
        return (valves.get(current_valve).unwrap().flow_rate, len + 0);
    }
    // println!("len: {len}");
    // dbg!(current_valve, &visited_valves);
    let mut most_neighbour_rate = valves.get(current_valve).unwrap().flow_rate;
    let mut most_len = len;
    // dbg!(&visited_valves);
    visited_valves.insert(current_valve);
    // dbg!(&current_valve, &visited_valves);
    for neighbour_name in valves.get(current_valve).unwrap().leads_to.iter() {
        if visited_valves.contains(neighbour_name) && *neighbour_name != "AA" {
            continue;
        }
        // println!("{neighbour_name}");
        let neighbour = valves.get(neighbour_name).unwrap();
        if neighbour.flow_rate > most_neighbour_rate && !valves_open.contains(neighbour_name) {
            most_neighbour_rate = neighbour.flow_rate;
            most_len = len + 1;
        }
        let recursive = get_highest(
            neighbour.name,
            valves,
            valves_open.clone(),
            &mut visited_valves.clone(),
            len + 1,
        );
        if recursive.0 > most_neighbour_rate {
            most_neighbour_rate = recursive.0;
            most_len = recursive.1;
        }
    }
    (most_neighbour_rate, most_len)
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut valves = HashMap::new();
    for line in inp {
        let valve = parse_valve(line).unwrap().1;
        valves.insert(valve.name, valve);
    }
    /* dbg!(len_back_to_aa("DD", &valves, 0));
    return; */
    /* for valve in valves.clone().values() {

        for neighbour in valve.leads_to.iter() {
            let mut neighbour = valves.get_mut(neighbour).unwrap();
            neighbour.leads_to.push(valve.name);
        }
    }
    dbg!(&valves); */
    let mut valves_open = HashSet::new();
    let mut rate_open = 0;
    let mut pressure_released = 0;
    let mut minute_counter = 0;
    let mut current_valve = valves.get("AA").unwrap();
    let mut stack = Vec::new();
    stack.push(current_valve.name);
    while minute_counter < 30 {
        minute_counter += 1;
        pressure_released += rate_open;
        current_valve = valves.get(stack.last().unwrap()).unwrap();
        // dbg!(&stack);
        println!("\n== Minute {} ==", minute_counter);
        println!("Releasing {} pressure.", rate_open);
        // dbg!(&current_valve);
        let mut set = HashSet::new();
        set.insert(current_valve.name);
        let max_aa = if current_valve.leads_to.contains(&"AA") {
            (
                current_valve
                    .leads_to
                    .iter()
                    .position(|&x| x == "AA")
                    .unwrap(),
                get_highest(
                    "AA",
                    &valves,
                    valves_open.clone(),
                    &mut stack.clone().into_iter().collect(),
                    0,
                ),
            )
        } else {
            (0, (0, 0))
        };
        let max = current_valve
            .leads_to
            .clone()
            .into_iter()
            // .map(|x| get_highest(x, &valves, valves_open.clone(), &mut set.clone()))
            .map(|x| {
                if x != "AA" {
                    get_highest(
                        x,
                        &valves,
                        valves_open.clone(),
                        &mut stack.clone().into_iter().collect(),
                        0,
                    )
                } else {
                    (0, 0)
                }
            })
            .enumerate()
            .fold(
                (0, (0, 0)),
                |a, b| if (b.1).0 > 2 * (a.1).0 { b } else { a },
            );
        // dbg!(max);
        if current_valve.flow_rate >= (max.1).0 - (max.1).1
            && !valves_open.contains(current_valve.name)
            && current_valve.flow_rate != 0
        {
            valves_open.insert(current_valve.name);
            rate_open += current_valve.flow_rate;
            // println!("You open valve {}", current_valve.name);
            continue;
        }
        // dbg!(max, max_aa, len_back_to_aa(current_valve.name, &valves, 0));
        let cost_aa = match ((max_aa.1)
                .0
                .checked_sub(((max_aa.1).1 + len_back_to_aa(current_valve.name, &valves, 0))))
            {
                Some(x) => x,
                None => 0,
            };
        let new_valve = if (max.1).0 == 0
            || cost_aa  > (max.1).0 - (max.1).1 {
            current_valve.leads_to.get(max_aa.0).unwrap()
        } else {
            current_valve.leads_to.get(max.0).unwrap()
        };
        /* if stack.len() > 2 &&
        stack.get(stack.len() - 2).unwrap() == new_valve { */
        /* if stack.contains(new_valve) && *new_valve != "AA" {
            stack.pop();
            current_valve = valves.get(stack.last().unwrap()).unwrap();
            let max_aa = if current_valve.leads_to.contains(&"AA") {
                (
                    current_valve
                        .leads_to
                        .iter()
                        .position(|&x| x == "AA")
                        .unwrap(),
                    get_highest(
                        "AA",
                        &valves,
                        valves_open.clone(),
                        &mut stack.clone().into_iter().collect(),
                        0,
                    ),
                )
            } else {
                (0, (0, 0))
            };
            let max = current_valve
                .leads_to
                .clone()
                .into_iter()
                // .map(|x| get_highest(x, &valves, valves_open.clone(), &mut set.clone()))
                .map(|x| {
                    if x != "AA" {
                        get_highest(
                            x,
                            &valves,
                            valves_open.clone(),
                            &mut stack.clone().into_iter().collect(),
                            0,
                        )
                    } else {
                        (0, 0)
                    }
                })
                .enumerate()
                .fold(
                    (0, (0, 0)),
                    |a, b| if (b.1).0 > 2 * (a.1).0 { b } else { a },
                );
            // dbg!(max, &current_valve.leads_to);
            new_valve = if (max.1).0 == 0 {
                // || ((max_aa.1).0 > (max.1).0 && (max_aa.1 .1) < (max.1).1) {
                current_valve.leads_to.get(max_aa.0).unwrap()
            } else {
                current_valve.leads_to.get(max.0).unwrap()
            };
            println!("Moin");
        } */
        stack.push(new_valve);
        // println!("You move to valve {}", new_valve);

        /* if *stack.last().unwrap() == "AA" {
            while stack.len() > 1 {
                stack.pop();
                let l = valves.get_mut(stack.last().unwrap()).unwrap();
                l.leads_to.pop();
            }
            continue;
        } */
        /* if current_valve.name == *stack.last().unwrap() {
            stack.pop();
            let l = valves.get_mut(stack.last().unwrap()).unwrap();
            l.leads_to.pop();
        } */
    }
    /* while minute_counter < 5 {
        minute_counter += 1;
        println!("== Minute {} ==", minute_counter);
        pressure_released += rate_open;
        let most_neighbour_path = get_highest_path(
            current_valve,
            &valves,
            valves_open.clone(),
            &mut HashSet::new(),
            Vec::new(),
        );
        dbg!(&current_valve, &most_neighbour_path);
        if let Some(most_neighbour_path) = most_neighbour_path {
            let most_neighbour_name = most_neighbour_path.last().unwrap();
            let most_neighbour_rate = valves.get(most_neighbour_name).unwrap().flow_rate;
            // dbg!(&most_neighbour_rate);
            if most_neighbour_rate > current_valve.flow_rate
                || valves_open.contains(current_valve.name)
            {
                current_valve = valves.get(most_neighbour_path.get(1).unwrap()).unwrap();
            } else {
                valves_open.insert(current_valve.name);
                rate_open += current_valve.flow_rate;
            }
        } else if !valves_open.contains(current_valve.name) {
            valves_open.insert(current_valve.name);
            rate_open += current_valve.flow_rate;
        }
    }*/
    dbg!(pressure_released);
}
