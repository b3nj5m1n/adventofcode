use std::collections::{HashMap, HashSet};
use std::env;
use std::io::Read;

use itertools::Itertools;
use petgraph::algo::connected_components;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::IntoNeighbors;
use rayon::iter::ParallelBridge;

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
    part_1: usize,
    part_2: usize,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let inp = inp
        .into_iter()
        .map(|l| {
            let mut i = l.split("-");
            (i.next().unwrap(), i.next().unwrap())
        })
        .collect::<Vec<_>>();

    let mut g = UnGraph::<&str, ()>::new_undirected();
    let mut node_map = HashMap::new();

    for &(src, dst) in &inp {
        let src_index = *node_map.entry(src).or_insert_with(|| g.add_node(src));
        let dst_index = *node_map.entry(dst).or_insert_with(|| g.add_node(dst));

        g.add_edge(src_index, dst_index, ());
    }
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    assert_eq!(connected_components(&g), 1);

    /* let test_1 = vec![node_map["aq"], node_map["cg"], node_map["yn"]]; // Is clique
    let test_2 = vec![node_map["ta"], node_map["ka"], node_map["yn"]]; // Is not a clique
    let test_3 = vec![node_map["ub"], node_map["tb"], node_map["wq"]]; // Is not a clique
    dbg!(is_clique(&g, &test_1));
    dbg!(is_clique(&g, &test_2));
    dbg!(is_clique(&g, &test_3)); */

    res.part_1 = std::iter::repeat(g.node_indices().into_iter())
        .take(3)
        .multi_cartesian_product()
        .filter_map(|v| {
            let mut v = v;
            v.sort();
            let (a, b, c) = (v[0], v[1], v[2]);
            if (a != b) && (b != c) && (c != a) {
                Some(v)
            } else {
                None
            }
        })
        .sorted_by(|a, b| a.first().cmp(&b.first()))
        .dedup()
        .sorted_by(|a, b| a[1].cmp(&b[1]))
        .dedup()
        .sorted_by(|a, b| a[2].cmp(&b[2]))
        .dedup()
        .filter(|v| is_clique(&g, v))
        .map(|v| {
            v.iter()
                .map(|n| g.node_weight(*n).unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| v.iter().filter(|s| s.starts_with("t")).count() > 0)
        .count();
}

fn is_clique(graph: &UnGraph<&str, ()>, nodes: &[NodeIndex]) -> bool {
    'outer: for node in nodes.iter() {
        let mut count_matches = 0;
        for neighbour in graph.neighbors(*node) {
            if nodes.contains(&neighbour) {
                count_matches += 1;
            }
        }
        if count_matches != nodes.len() - 1 {
            return false;
        }
    }
    true
}
