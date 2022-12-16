use petgraph::adj::NodeIndex;
use petgraph::algo::{self, floyd_warshall};
use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::visit::{depth_first_search, Bfs, Dfs, DfsEvent, DfsPostOrder, NodeRef};
use petgraph::{Direction, Graph};
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
    // output(&result);
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

fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut valves = HashMap::new();
    let mut graph = Graph::<Valve, u32>::new();
    let mut root = None;
    for line in inp {
        let valve = parse_valve(line).unwrap().1;
        let node = graph.add_node(valve.clone());
        if valve.name == "AA" {
            root = Some(node);
        }
        valves.insert(valve.name, (valve.clone(), node));
    }
    for (valve, node) in valves.values() {
        for neighbor in valve.leads_to.iter() {
            let (neighbor_valve, neighbor_node) = valves.get(neighbor).unwrap();
            let weight = if neighbor_valve.name == "AA" {
                10000
            } else {
                1
            };
            graph.add_edge(*node, *neighbor_node, weight);
        }
    }
    let res = floyd_warshall(&graph, |edge| 1).unwrap();
    /* for ((start, end), len) in res.iter() {
        println!(
            "{:?} -> {:?} takes {} steps.",
            graph.node_weight(start).unwrap().name,
            graph.node_weight(end).unwrap().name,
            len
        );
    } */

    let mut minutes = 0;
    // let mut opened = HashSet::new();
    let root = root.expect("Root node could not be located.");
    let mut current = root;
    let mut current_rate = 0;

    /* let mut stack = Vec::new();
    depth_first_search(&graph, Some(root), |event| {
        dbg!(&stack);
        match event {
            // DfsEvent::TreeEdge(n1, n2) => { stack.push(n2); },
            // DfsEvent::TreeEdge(n1, n2) => { dbg!(&stack); },
            // DfsEvent::Discover(n, t) => { stack.push(n2); },
            DfsEvent::Discover(n, t) => { stack.push(&graph[n].name); },
            DfsEvent::CrossForwardEdge(n1, n2) => { stack.pop(); },
            _ => ()
        };
    }) */

    /* while minutes < 30 {
        // Get next destination
        let mut highest = (0, current);
        let mut dfs = DfsPostOrder::new(&graph, current);
        while let Some(nx) = dfs.next(&graph) {
            let node = graph.node_weight(nx).unwrap();
            let cost = match node.flow_rate.checked_sub(*res.get(&(current, nx)).unwrap()) {
                Some(x) => x,
                None => 0,
            };
            if cost > highest.0 && !opened.contains(&nx) {
                highest = (cost, nx);
            }
            println!("{:?}, ", node.name);
        }
        println!("Highest: {}, label: {:?}", highest.0, graph[highest.1].name);
        // Go to next destination
        let path = algo::all_simple_paths::<Vec<_>, _>(
            &graph,
            current,
            highest.1,
            0,
            Some(*res.get(&(current, highest.1)).unwrap() as usize),
        )
        .collect::<Vec<_>>();
        for node in path.clone()[0].iter().skip(1) {
            minutes += 1;
            println!("== Minute {minutes} ==");
            println!("Outputting {} per minute", current_rate);
            println!(
                "You move to valve {:?}",
                graph.node_weight(*node).unwrap().name
            );
        }
        opened.insert(highest.1);
        minutes += 1;
        println!("== Minute {minutes} ==");
        println!(
            "You open valve {:?}",
            graph.node_weight(highest.1).unwrap().name
        );
        current = highest.1;
        current_rate += graph.node_weight(highest.1).unwrap().flow_rate;
    } */

    let mut dfs = Dfs::new(&graph, root);
    while let Some(nx) = dfs.next(&graph) {
        /* if graph.neighbors(nx).count() != 0 {
            continue;
        } */
        println!(
            "\n\n\n{:?}, Stack:\n-----\n",
            graph.node_weight(nx).unwrap().name
        );
        for node in dfs.stack.iter() {
            println!("{:?}", graph.node_weight(*node).unwrap().name);
        }
    }

    /* for neighbor in graph.neighbors_directed(root, Direction::Outgoing).skip(2).next() {
        println!("{:?}", graph.node_weight(neighbor));
        let mut dfs = DfsPostOrder::new(&graph, neighbor);
        while let Some(nx) = dfs.next(&graph) {
            if graph.neighbors(nx).count() != 1 {
                continue;
            }
            println!(
                "\n\n\n{:?}, Stack:\n-----\n",
                graph.node_weight(nx).unwrap().name
            );
            for node in dfs.stack.iter() {
                println!("{:?}", graph.node_weight(*node).unwrap().name);
            }
        }
    } */
    // println!("{:?}", Dot::new(&graph));
}
