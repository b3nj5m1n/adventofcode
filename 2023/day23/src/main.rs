use std::collections::HashMap;
use std::env;
use std::io::Read;

use petgraph::{
    algo,
    graph::{DiGraph, NodeIndex},
};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Forest,
    Path,
    Slope(Slope),
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Slope {
    Up,
    Right,
    Down,
    Left,
}

fn get_node_index_by_point(graph: &DiGraph<Point, i32>, point: &Point) -> Option<NodeIndex> {
    for node_index in graph.node_indices() {
        if &graph[node_index] == point {
            return Some(node_index);
        }
    }
    None
}

fn construct_graph(inp: Vec<&str>, part_2: bool) -> (DiGraph<Point, i32>, NodeIndex, NodeIndex) {
    let mut graph = DiGraph::<Point, i32>::new();
    let max_y = inp.len();
    let max_x = inp[0].len();
    let mut map = HashMap::new();
    let mut from = None;
    let mut to = None;
    for (y, line) in inp.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let current = Point {
                x: x as i64,
                y: y as i64,
            };
            if c == '#' {
                map.insert(current, Tile::Forest);
                continue;
            } else if c == '.' {
                map.insert(current.clone(), Tile::Path);
                graph.add_node(current.clone());
                if y == 0 {
                    from = Some(current.clone());
                }
                if y == max_y - 1 {
                    to = Some(current.clone());
                }
            } else if c == '^' {
                map.insert(current.clone(), Tile::Slope(Slope::Up));
                graph.add_node(current);
            } else if c == '>' {
                map.insert(current.clone(), Tile::Slope(Slope::Right));
                graph.add_node(current);
            } else if c == 'v' {
                map.insert(current.clone(), Tile::Slope(Slope::Down));
                graph.add_node(current);
            } else if c == '<' {
                map.insert(current.clone(), Tile::Slope(Slope::Left));
                graph.add_node(current);
            } else {
                unreachable!()
            }
        }
    }
    let from = from.expect("Failed to find entry point");
    let to = to.expect("Failed to find exit point");
    for x in 0..max_x as i64 {
        for y in 0..max_y as i64 {
            let current = Point { x, y };
            let current_tile = map[&current];
            if let Tile::Forest = current_tile {
                continue;
            }
            for neighbour_d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let neighbour = Point {
                    x: x + neighbour_d.0,
                    y: y + neighbour_d.1,
                };
                if neighbour.x < 0
                    || neighbour.y < 0
                    || neighbour.x >= max_x as i64
                    || neighbour.y >= max_y as i64
                {
                    continue;
                }
                let neighbour_tile = map[&neighbour];
                if let Tile::Forest = neighbour_tile {
                    continue;
                }
                if !part_2 {
                    if let Tile::Slope(s) = current_tile {
                        match s {
                            Slope::Up => {
                                if neighbour_d != (0, -1) {
                                    continue;
                                }
                            }
                            Slope::Right => {
                                if neighbour_d != (1, 0) {
                                    continue;
                                }
                            }
                            Slope::Down => {
                                if neighbour_d != (0, 1) {
                                    continue;
                                }
                            }
                            Slope::Left => {
                                if neighbour_d != (-1, 0) {
                                    continue;
                                }
                            }
                        }
                    }
                }
                graph.add_edge(
                    get_node_index_by_point(&graph, &current)
                        .expect("Failed to find node index by point"),
                    get_node_index_by_point(&graph, &neighbour)
                        .expect("Failed to find node index by point"),
                    1,
                );
            }
        }
    }
    let from = get_node_index_by_point(&graph, &from).expect("Failed to find node index by point");
    let to = get_node_index_by_point(&graph, &to).expect("Failed to find node index by point");

    (graph, from, to)
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let (graph, from, to) = construct_graph(inp.clone(), false);

    let ways = algo::all_simple_paths(&graph, from, to, 0, None).collect::<Vec<Vec<_>>>();
    let ways_lens = ways
        .into_iter()
        .map(|path| path.len() - 1)
        .collect::<Vec<_>>();
    res.part_1 = ways_lens.into_iter().max().expect("Unreachable");

    let (graph, from, to) = construct_graph(inp, true);

    let ways = algo::all_simple_paths(&graph, from, to, 0, None).collect::<Vec<Vec<_>>>();
    let ways_lens = ways
        .into_iter()
        .map(|path| path.len() - 1)
        .collect::<Vec<_>>();
    res.part_2 = ways_lens.into_iter().max().expect("Unreachable");
}
