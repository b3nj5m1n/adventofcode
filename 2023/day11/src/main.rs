use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Display;
use std::io::Read;

use itertools::Itertools;
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
    part_1: u32,
    part_2: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Galaxy,
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Space => f.write_str("."),
            Tile::Galaxy => f.write_str("#"),
        }
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid: Vec<Vec<_>> = inp
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Space,
                    '#' => Tile::Galaxy,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    // Fill columns
    let mut xs = Vec::new();
    'outer: for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[y][x] == Tile::Galaxy {
                continue 'outer;
            }
        }
        xs.push(x);
    }
    let mut count = 0;
    for x in xs {
        for row in grid.iter_mut() {
            row.insert(x + count, Tile::Space);
        }
        count += 1;
    }

    // Fill rows
    let mut ys = Vec::new();
    'outer: for y in 0..grid.len() {
        if grid[y].contains(&Tile::Galaxy) {
            continue 'outer;
        }
        ys.push(y);
    }
    let mut count = 0;
    for y in ys {
        grid.insert(
            y + count,
            std::iter::repeat(Tile::Space).take(grid[0].len()).collect(),
        );
        count += 1;
    }

    /* // Print
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        print!("\n");
    } */

    let mut g = Graph::new();

    let mut nodes = HashMap::new();
    let mut galaxies = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let a = if !nodes.contains_key(&(y,x)) {
                let a = g.add_node((y, x));
                nodes.insert((y, x), a);
                a
            } else {
                *nodes.get(&(y,x)).unwrap()
            };
            for neighbour in get_neighbours((y, x), &grid) {
                if !nodes.contains_key(&neighbour) {
                    let b = g.add_node(neighbour);
                    nodes.insert(neighbour, b);
                }
                g.add_edge(a, *nodes.get(&neighbour).unwrap(), 1);
                g.add_edge(*nodes.get(&neighbour).unwrap(), a, 1);
            }
            if grid[y][x] == Tile::Galaxy {
                galaxies.push(a);
            }
        }
    }

    // println!("{:?}", petgraph::dot::Dot::with_config(&g, &[petgraph::dot::Config::EdgeNoLabel]));

    let mut m = HashSet::new();
    for (a, b) in galaxies.clone().into_iter().cartesian_product(galaxies) {
        if m.contains(&(a, b)) || m.contains(&(b, a)) || a == b {
            continue;
        }
        // println!("{:?}, {:?}", g[a], g[b]);
        m.insert((a, b));
        // dbg!(shortest_path(a, b, &grid));
        /* let a = g.add_node(a);
        let b = g.add_node(b); */
        // g.add_edge(a, b, 1);
        let path = petgraph::algo::astar(&g, a, |finish| finish == b, |e| *e.weight(), |_| 0).unwrap();
        // dbg!(&path);
        // assert!(path.is_some());
        res.part_1 += path.0;
    }
    // dbg!(galaxies);
}

fn get_neighbours(tile: (usize, usize), grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if 0 < tile.0 {
        result.push((tile.0 - 1, tile.1));
    }
    if tile.0 < grid.len() - 1 {
        result.push((tile.0 + 1, tile.1));
    }
    if 0 < tile.1 {
        result.push((tile.0, tile.1 - 1));
    }
    if grid[0].len() - 1 < tile.1 {
        result.push((tile.0, tile.1 + 1));
    }
    result
}

/* fn shortest_path(start: (usize, usize), end: (usize, usize), grid: &Vec<Vec<Tile>>) -> Option<u32> {
    println!("Finding shortest path from {start:?} to {end:?}");
    if start == end {
        return Some(0);
    }
    let neighbours = get_neighbours(start, grid)
        .into_iter()
        .map(|neighbour| {
            (
                f64::sqrt(
                    ((end.0 as f64 - neighbour.0 as f64) * (end.0 as f64 - neighbour.0 as f64)
                        + (end.1 as f64 - neighbour.1 as f64)
                            * (end.1 as f64 - neighbour.1 as f64)),
                ),
                neighbour,
            )
        })
        .sorted_by(|a, b| a.0.total_cmp(&b.0));
    let min = neighbours
        .clone()
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|n| n.0);
    let to_consider = neighbours.filter(|n| Some(n.0) == min);
    to_consider.into_iter().map(|current| shortest_path(current.1, end, grid)).min().flatten()
}

/* let mut steps = 0;
let mut current = start;
loop {
    let neighbours: Vec<_> = get_neighbours(current, grid)
        .into_iter()
        .map(|neighbour| {
            (
                f64::sqrt(
                    ((end.0 as f64 - neighbour.0 as f64) * (end.0 as f64 - neighbour.0 as f64)
                        + (end.1 as f64 - neighbour.1 as f64)
                            * (end.1 as f64 - neighbour.1 as f64)),
                ),
                neighbour,
            )
        })
        .sorted_by(|a, b| a.0.total_cmp(&b.0))
        .collect();
    let shortest_neighbor = neighbours[0].1;
    steps += 1;
    if shortest_neighbor == end {
        break;
    }
    current = shortest_neighbor;
    dbg!(current);
}
return steps; */ */
