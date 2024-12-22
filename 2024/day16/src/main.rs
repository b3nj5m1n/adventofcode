use std::borrow::Borrow;
use std::cell::RefCell;
use std::env;
use std::io::Read;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use petgraph::algo::{astar, dijkstra};
use petgraph::data::{DataMap, DataMapMut};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::{Graph, Undirected};

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
    let obstacles = inp
        .clone()
        .into_iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '#' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let start = inp
        .clone()
        .into_iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()[0];
    let end = inp
        .clone()
        .into_iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(x, c)| if c == 'E' { Some((x, y)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()[0];
    /* dbg!(start);
    dbg!(end); */
    let max_x = *obstacles
        .iter()
        .map(|(x, _)| x)
        .reduce(std::cmp::max)
        .expect("Couldn't find max");
    let max_y = *obstacles
        .iter()
        .map(|(_, y)| y)
        .reduce(std::cmp::max)
        .expect("Couldn't find max");
    // let n = *max_coord as usize;

    let mut graph = UnGraph::new_undirected();
    let mut node_indices = vec![vec![None; max_x + 1]; max_y + 1];

    for x in 0..=max_x {
        for y in 0..=max_y {
            let node = graph.add_node((x, y));
            node_indices[y][x] = Some(node);
        }
    }

    for x in 0..=max_x {
        for y in 0..=max_y {
            if y < max_y {
                if let (Some(node1), Some(node2)) = (node_indices[y][x], node_indices[y + 1][x]) {
                    graph.add_edge(node1, node2, ());
                }
            }
            if x < max_x {
                if let (Some(node1), Some(node2)) = (node_indices[y][x], node_indices[y][x + 1]) {
                    graph.add_edge(node1, node2, ());
                }
            }
        }
    }

    // dbg!(find_node_by_weight(&graph, start).expect("Couldn't find start"));

    for obstacle in obstacles.into_iter() {
        assert!(obstacle != start);
        let index = find_node_by_weight(&graph, obstacle).expect("Fuck");
        graph.remove_node(index);
    }

    res.part_1 =
        find_connecting_path(&graph, start, end, max_x, max_y).expect("Part 1 unsolvable") - 1;

    /* for obstacle in inp.into_iter().skip(m) {
        let index = find_node_by_weight(&graph, obstacle).expect("Fuck");
        graph.remove_node(index);
        if let None = find_connecting_path(&graph, n) {
            res.part_2 = format!("{},{}", obstacle.0, obstacle.1);
            return;
        }
    } */
}

fn find_connecting_path(
    graph: &petgraph::Graph<(usize, usize), (), petgraph::Undirected>,
    start: (usize, usize),
    end: (usize, usize),
    max_x: usize,
    max_y: usize,
) -> Option<usize> {
    /* let path = astar(
        &graph,
        find_node_by_weight(&graph, start).expect("Couldn't find start"),
        |finish| graph.node_weight(finish) == Some(&end),
        |_| 1,
        |nindex| {
            let (a, b) = graph.node_weight(nindex).expect("f");
            max_x.abs_diff(*a) + max_y.abs_diff(*b)
        },
    )
    .map(|res| res.1)
    .expect("Couldn't find path")
    .into_iter()
    .map(|n| graph.node_weight(n).expect("F"))
    .collect::<Vec<_>>(); */

    let start = find_node_by_weight(&graph, start).expect("Couldn't find start");
    let end = find_node_by_weight(&graph, end).expect("Couldn't find end");
    Some(
        find_all_paths(&graph, start, end)
            .into_iter()
            .map(|path| {
                let path = path
                    .into_iter()
                    .map(|n| graph.node_weight(n).expect("F"))
                    .collect::<Vec<_>>();
                // println!("Path: {}", &path.iter().map(|(a,b)| format!("{a},{b} ")).collect::<String>());
                let directions = path
                    .as_slice()
                    .windows(2)
                    .map(|window| {
                        let (a, b) = (
                            (window[0].0 as i64, window[0].1 as i64),
                            (window[1].0 as i64, window[1].1 as i64),
                        );
                        (a.0 - b.0, a.1 - b.1)
                    })
                    .collect::<Vec<_>>();
                let score = directions
                    .iter()
                    .fold((0, (1, 0)), |(score, last_dir), &new_dir| {
                        if last_dir == new_dir {
                            (score + 1, new_dir)
                        } else {
                            (score + 1000 + 1, new_dir)
                        }
                    })
                    .0;
                // println!("Score: {}", score);
                score + 1
            })
            .min()?,
    )
    /* dbg!();

    // dbg!(path.iter().fold(0, |a, ));

    Some(path.len()) */
}

fn find_node_by_weight(
    graph: &petgraph::Graph<(usize, usize), (), petgraph::Undirected>,
    weight: (usize, usize),
) -> Option<petgraph::graph::NodeIndex> {
    graph
        .node_indices() // Iterate over all node indices
        .find(|&index| graph[index] == weight) // Check if the node weight matches
}

fn find_all_paths(
    graph: &Graph<(usize, usize), (), Undirected>,
    start: petgraph::graph::NodeIndex,
    end: petgraph::graph::NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    println!("Finding all paths...");
    let mut paths = Vec::new();
    let mut visited = vec![false; graph.node_count()];
    let mut current_path = Vec::new();
    let mut best_path = RefCell::new(None);

    fn dfs(
        graph: &Graph<(usize, usize), (), Undirected>,
        current: NodeIndex,
        end: NodeIndex,
        visited: &mut Vec<bool>,
        current_path: &mut Vec<(NodeIndex, (i64, i64))>,
        paths: &mut Vec<Vec<NodeIndex>>,
        num_turns: usize,
        best_path: &mut RefCell<Option<usize>>,
    ) {
        visited[current.index()] = true;
        if current_path.is_empty() {
            current_path.push((current, (1, 0)));
        } else {
            let last_idx = current_path.last().unwrap().0;
            let a = graph
                .node_weight(current)
                .expect("Couldn't find current node");
            let b = graph
                .node_weight(last_idx)
                .expect("Couldn't find last node");
            let dir = (a.0 as i64 - b.0 as i64, a.1 as i64 - b.1 as i64);
            current_path.push((current, dir));
        }

        let _best_path: &Option<usize> = &best_path.replace_with(|x| *x);
        let best = _best_path.clone();
        // std::mem::drop(_best_path);
        if let Some(best) = best {
            if num_turns > best {
                visited[current.index()] = false;
                current_path.pop();
                return;
            }
        }
        if current == end {
            paths.push(current_path.clone().into_iter().map(|(i, _)| i).collect());
            let new_best = match _best_path {
                Some(best) => Some(num_turns.min(*best)),
                None => Some(num_turns),
            };

            *best_path.get_mut() = new_best;

            println!("Found {} paths, current best score is ~ {}.", paths.len(), new_best.expect("Fuck") * 1000);
        } else {
            for neighbor in graph.neighbors(current) {
                if !visited[neighbor.index()] {
                    dfs(
                        graph,
                        neighbor,
                        end,
                        visited,
                        current_path,
                        paths,
                        current_path
                            .iter()
                            .skip(1)
                            .map(|(_, d)| d)
                            .fold((0, (1, 0)), |(num_turns, last_dir), &new_dir| {
                                if last_dir == new_dir {
                                    (num_turns, new_dir)
                                } else {
                                    (num_turns + 1, new_dir)
                                }
                            })
                            .0,
                        best_path,
                    );
                }
            }
        }

        // Backtrack
        visited[current.index()] = false;
        current_path.pop();
    }

    dfs(
        graph,
        start,
        end,
        &mut visited,
        &mut current_path,
        &mut paths,
        0,
        &mut best_path,
    );

    println!("Found all paths! There are {}.", paths.len());
    paths
}
