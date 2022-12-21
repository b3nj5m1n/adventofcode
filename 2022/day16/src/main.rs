use itertools::Itertools;
use petgraph::adj::NodeIndex;
use petgraph::algo::{self, floyd_warshall};
use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
use petgraph::visit::{depth_first_search, Bfs, Dfs, DfsEvent, DfsPostOrder, IntoEdges, NodeRef};
use petgraph::{Direction, Graph};
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
    // output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
struct CallSig {
    current: petgraph::graph::NodeIndex,
    opened: HashSet<petgraph::graph::NodeIndex>,
    minutes_left: u32,
}

impl Hash for CallSig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.current.hash(state);
        for open in self.opened.iter() {
            open.hash(state);
        }
        self.minutes_left.hash(state);
    }
}

fn get_max_flow<'a>(
    current: petgraph::graph::NodeIndex,
    opened: HashSet<petgraph::graph::NodeIndex>,
    minutes_left: u32,
    valves: &Graph<Valve, u32>,
    memo: &'a RefCell<HashMap<CallSig, u32>>,
) -> u32 {
    let call_sig = CallSig {
        current,
        opened: opened.clone(),
        minutes_left,
    };
    if memo.borrow().contains_key(&call_sig) {
        return *memo.borrow().get(&call_sig).unwrap();
    }
    if minutes_left == 0 {
        return 0;
    }
    if opened.contains(&current) {
        return 0;
    }
    let mut current_best = 0;
    // todo!()
    let current_value = valves[current].flow_rate * (minutes_left - 1);
    let mut opened_current = opened.clone();
    opened_current.insert(current);
    let mut neighbours_ = valves
        .neighbors_directed(current, Direction::Outgoing)
        .detach();
    let mut neighbors = Vec::new();
    while let Some(n) = neighbours_.next_node(&valves) {
        neighbors.push(n);
    }
    for neighbour in neighbors {
        let cost = valves
            .edge_weight(valves.find_edge(current, neighbour).unwrap())
            .unwrap();
        if minutes_left > *cost {
            current_best = max(
                current_best,
                get_max_flow(
                    neighbour,
                    opened.clone(),
                    minutes_left - cost,
                    &valves,
                    &memo,
                ),
            );
        }
        if minutes_left > (cost + 1) {
            current_best = max(
                current_best,
                current_value
                    + get_max_flow(
                        neighbour,
                        opened_current.clone(),
                        minutes_left - (cost + 1),
                        &valves,
                        &memo,
                    ),
            );
        }
    }
    memo.borrow_mut().insert(call_sig, current_best);
    current_best
}

fn solve(inp: Vec<&str>, res: &mut Result) {
    // This holds all the parsed valve structs
    let mut valves = HashMap::new();
    // Petgraph for the valve/tunnel layout
    let mut graph = Graph::<Valve, u32>::new();
    // This will hold the node index of the root node
    let mut root = None;
    // Parse input into valves and add nodes to graph
    for line in inp {
        let valve = parse_valve(line).unwrap().1;
        let node = graph.add_node(valve.clone());
        if valve.name == "AA" {
            root = Some(node);
        }
        valves.insert(valve.name, (valve.clone(), node));
    }
    // Extract root
    let mut root = root.expect("Root node not found");
    // Add connecting edges between nodes
    for (valve, node) in valves.values() {
        for neighbor in valve.leads_to.iter() {
            let (_, neighbor_node) = valves.get(neighbor).unwrap();
            let weight = 1;
            graph.add_edge(*node, *neighbor_node, weight);
        }
    }
    // Vector containing node indicies for valves with flow rate 0
    let mut to_remove = Vec::new();
    // Do a traversal of the graph and remove valves with flow rate 0 to reduce search space
    // This could get problematic with multiple 0s in sequence, I hope the fact this is a bfs gets
    // rid of that problem since we're adding the new edges immediately.
    let mut bfs = Bfs::new(&graph, root);
    while let Some(nx) = bfs.next(&graph) {
        if graph[nx].flow_rate == 0 && nx != root {
            let mut neighbors_in_ = graph.neighbors_directed(nx, Direction::Incoming).detach();
            let mut neighbors_in = Vec::new();
            while let Some(n) = neighbors_in_.next_node(&graph) {
                neighbors_in.push(n);
            }
            let mut neighbors_out_ = graph.neighbors_directed(nx, Direction::Outgoing).detach();
            let mut neighbors_out = Vec::new();
            while let Some(n) = neighbors_out_.next_node(&graph) {
                neighbors_out.push(n);
            }
            let mut new_edges = Vec::new();
            for (a, b) in neighbors_in
                .into_iter()
                .cartesian_product(neighbors_out.into_iter())
            {
                if a == b {
                    continue;
                }
                let w1 = graph.edge_weight(graph.find_edge(a, nx).unwrap()).unwrap();
                let w2 = graph.edge_weight(graph.find_edge(b, nx).unwrap()).unwrap();
                new_edges.push((a, b, w1 + w2));
            }
            for (a, b, w) in new_edges {
                graph.add_edge(a, b, w);
            }
            to_remove.push(nx);
        }
    }
    // Remove the valves with flow rate 0 from the graph
    for nx in to_remove {
        graph.remove_node(nx);
    }
    let mut bfs = Bfs::new(&graph, petgraph::graph::NodeIndex::new(0));
    while let Some(nx) = bfs.next(&graph) {
        if graph[nx].name == "AA" {
            root = nx;
        }
    }
    // let res = floyd_warshall(&graph, |edge| 1).unwrap();
    // println!("{:?}", Dot::new(&graph));
    dbg!(get_max_flow(
        root,
        HashSet::new(),
        30,
        &graph,
        &RefCell::new(HashMap::new()),
    ));
}
