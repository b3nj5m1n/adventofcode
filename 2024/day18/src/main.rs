use std::env;
use std::io::Read;

use petgraph::algo::{astar, dijkstra};
use petgraph::data::{DataMap, DataMapMut};
use petgraph::graph::UnGraph;

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
        part_2: "".to_string(),
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: usize,
    part_2: String,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let inp = inp
        .into_iter()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .map(|l| {
            l.into_iter()
                .map(|n| n.parse::<usize>().expect("Couldn't parse number"))
                .collect::<Vec<_>>()
        })
        .map(|l| (l[0], l[1]))
        .collect::<Vec<_>>();
    let max_coord = inp
        .iter()
        .map(|(a, b)| a.max(b))
        .reduce(std::cmp::max)
        .expect("Couldn't find max");
    let n = *max_coord as usize;

    let mut graph = UnGraph::new_undirected();
    let mut node_indices = vec![vec![None; n + 1]; n + 1];

    for x in 0..=n {
        for y in 0..=n {
            let node = graph.add_node((x, y));
            node_indices[x][y] = Some(node);
        }
    }

    for x in 0..=n {
        for y in 0..=n {
            if x < n {
                if let (Some(node1), Some(node2)) = (node_indices[x][y], node_indices[x + 1][y]) {
                    graph.add_edge(node1, node2, ());
                }
            }
            if y < n {
                if let (Some(node1), Some(node2)) = (node_indices[x][y], node_indices[x][y + 1]) {
                    graph.add_edge(node1, node2, ());
                }
            }
        }
    }

    let m = 12;
    let m = 1024;
    for obstacle in inp.clone().into_iter().take(m) {
        let index = find_node_by_weight(&graph, obstacle).expect("Fuck");
        graph.remove_node(index);
    }

    res.part_1 = find_connecting_path(&graph, n).expect("Part 1 unsolvable") - 1;

    for obstacle in inp.into_iter().skip(m) {
        let index = find_node_by_weight(&graph, obstacle).expect("Fuck");
        graph.remove_node(index);
        if let None = find_connecting_path(&graph, n) {
            res.part_2 = format!("{},{}", obstacle.0, obstacle.1);
            return;
        }
    }
}

fn find_connecting_path(
    graph: &petgraph::Graph<(usize, usize), (), petgraph::Undirected>,
    n: usize,
) -> Option<usize> {
    astar(
        &graph,
        find_node_by_weight(&graph, (0, 0)).expect("Couldn't find end"),
        |finish| graph.node_weight(finish) == Some(&(n, n)),
        |_| 1,
        |nindex| {
            let (a, b) = graph.node_weight(nindex).expect("f");
            n.abs_diff(*a) + n.abs_diff(*b)
        },
    )
    .map(|res| res.1.len())
}

fn find_node_by_weight(
    graph: &petgraph::Graph<(usize, usize), (), petgraph::Undirected>,
    weight: (usize, usize),
) -> Option<petgraph::graph::NodeIndex> {
    graph
        .node_indices() // Iterate over all node indices
        .find(|&index| graph[index] == weight) // Check if the node weight matches
}
