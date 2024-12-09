use std::collections::HashMap;
use std::env::{self, current_dir};
use std::io::Read;
use std::ops::Index;

use petgraph::adj::NodeIndex;
use petgraph::algo::{connected_components, has_path_connecting, toposort};
use petgraph::csr::IndexType;
use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;
use petgraph::visit::NodeRef;
use petgraph::Graph;

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
    let inp: Vec<&str> = inp.split("\n").collect(); // .filter(|line| !line.is_empty()).collect();

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
    let split = inp.split(|l| l.is_empty()).collect::<Vec<&[&str]>>();
    let rules = split[0]
        .into_iter()
        .map(|l| {
            let n = l
                .split("|")
                .map(|n| n.parse::<u32>().expect("Invalid number"))
                .collect::<Vec<u32>>();
            (n[0], n[1])
        })
        .collect::<Vec<(u32, u32)>>();
    let mut valid_nodes = rules
        .iter()
        .map(|(a, b)| vec![*a, *b])
        .collect::<Vec<Vec<u32>>>()
        .concat();
    valid_nodes.sort();
    valid_nodes.dedup();
    let updates = split[1]
        .into_iter()
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<u32>().expect("Invalid number"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut rule_graph = DiGraph::<u32, ()>::new();
    let mut nodes_id_to_n = HashMap::new();
    let mut nodes_n_to_id = HashMap::new();
    for n in valid_nodes {
        if !nodes_n_to_id.contains_key(&n) {
            let idx = rule_graph.add_node(n);
            nodes_id_to_n.insert(idx, n);
            nodes_n_to_id.insert(n, idx);
        }
    }
    for edge in rules {
        rule_graph.add_edge(nodes_n_to_id[&edge.0], nodes_n_to_id[&edge.1], ());
    }

    // 9734 - too high?

    let mut counter_valid = 0;
    'a: for update in updates {
        for (i, a) in update.iter().enumerate() {
            // dbg!(i);
            for b in update.iter().skip(i + 1) {
                if has_path_connecting(&rule_graph, nodes_n_to_id[&b], nodes_n_to_id[&a], None) {
                    if !has_path_connecting(&rule_graph, nodes_n_to_id[&a], nodes_n_to_id[&b], None)
                    {
                        continue 'a;
                    }
                }
            }
        }
        assert!(update.len() % 2 == 1);
        counter_valid += 1;
        // dbg!(&update);
        res.part_1 += update[(update.len() - 1) / 2] as usize;
    }
    dbg!(counter_valid);

    dbg!(connected_components(&rule_graph));

    /* println!(
        "{:?}",
        Dot::with_config(&rule_graph, &[Config::EdgeNoLabel])
    ); */

    /* let sorted: Vec<_> = toposort(&rule_graph, None)
        .expect("Couldn't toposort")
        .into_iter()
        .map(|n| nodes_id_to_n[&n])
        // .rev()
        .collect::<Vec<u32>>();
    dbg!(&sorted);

    let mut counter_valid = 0;
    for update in updates {
        dbg!(&update);

        let update_indicies: Vec<_> = update
            .iter()
            .map(|n| sorted.iter().position(|n_| *n_ == *n).expect("Unreachable"))
            .collect();
        if update_indicies.is_sorted() {
            counter_valid += 1;
        }
        dbg!(update_indicies);
        break;

        /* let mut current_index = 0;
        let mut valid = true;
        for n in update {
            let n_index = sorted.iter().position(|n_| *n_ == n).expect("Unreachable");
            dbg!(current_index, n_index);
            for n_ in
            if n_index >= current_index {
                valid = false;
            }
            current_index = n_index;
        }
        if valid {
            counter_valid += 1;
        } */
    }
    res.part_1 = counter_valid;
    dbg!(sorted); */
}
