// Got some inspiritation for solving this as a 3d-graph
use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::Display;
use std::io::Read;

use petgraph::algo::astar;
use petgraph::data::Build;
use petgraph::dot::{Config, Dot};
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

#[derive(Debug, Clone, Copy)]
enum Tile {
    Wall,
    Walkable,
    Blizzard(Blizzard),
}
#[derive(Debug, Clone, Copy)]
enum Blizzard {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
}

// https://gistlib.com/rust/find-the-least-common-multiple-of-a-list-of-numbers-in-rust
fn lcm(numbers: Vec<usize>) -> usize {
    let mut lcm = 1;
    for number in numbers {
        lcm = lcm * number / gcd(lcm, number);
    }
    lcm
}
// https://gistlib.com/rust/find-the-least-common-multiple-of-a-list-of-numbers-in-rust
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::new();
        for line in self.tiles.iter() {
            let mut result_line = "".to_string();
            for tile in line.iter() {
                result_line.push_str(match tile {
                    Tile::Wall => "#",
                    Tile::Walkable => ".",
                    Tile::Blizzard(b) => match b {
                        Blizzard::Right => ">",
                        Blizzard::Left => "<",
                        Blizzard::Up => "^",
                        Blizzard::Down => "v",
                    },
                });
            }
            result.push(result_line);
        }
        write!(f, "{}", result.join("\n"))
    }
}

impl Grid {
    fn parse(input: Vec<&str>) -> Self {
        let mut result = Self { tiles: Vec::new() };
        for line in input {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Walkable,
                    '>' => Tile::Blizzard(Blizzard::Right),
                    '<' => Tile::Blizzard(Blizzard::Left),
                    'v' => Tile::Blizzard(Blizzard::Down),
                    '^' => Tile::Blizzard(Blizzard::Up),
                    _ => panic!("Unkown tile"),
                });
            }
            result.tiles.push(row);
        }
        result
    }
    fn get_cycle_len(&self) -> usize {
        let h_x = self.tiles[0].len() - 2;
        let h_y = self.tiles.len() - 2;
        lcm(vec![h_x, h_y])
    }
    fn generate_cycle(&self, i: usize) -> Grid {
        let i = i as i32;
        let mut new_tiles = HashSet::new();
        let l_y = (self.tiles.len() - 2) as i32;
        for (y, line) in self
            .tiles
            .iter()
            .skip(1)
            .enumerate()
            .take(self.tiles.len() - 2)
        {
            let y = y as i32;
            // for (y, line) in self.tiles.iter().enumerate() {
            let l_x = (line.len() - 2) as i32;
            // for (x, tile) in line.iter().enumerate() {
            for (x, tile) in line.iter().skip(1).enumerate().take(line.len() - 2) {
                let x = x as i32;
                if let Tile::Blizzard(b) = tile {
                    new_tiles.insert(match b {
                        Blizzard::Right => ((x + i).rem_euclid(l_x) + 1, (y).rem_euclid(l_y) + 1),
                        Blizzard::Left => ((x - i).rem_euclid(l_x) + 1, (y).rem_euclid(l_y) + 1),
                        Blizzard::Up => ((x).rem_euclid(l_x) + 1, (y - i).rem_euclid(l_y) + 1),
                        Blizzard::Down => ((x).rem_euclid(l_x) + 1, (y + i).rem_euclid(l_y) + 1),
                    });
                }
            }
        }
        let mut tiles = Vec::new();
        tiles.push(self.tiles.iter().next().unwrap().clone());
        for (y, line) in self
            .tiles
            .iter()
            .enumerate()
            .skip(1)
            .take(self.tiles.len() - 2)
        {
            let mut row = Vec::new();
            row.push(line.iter().next().unwrap().clone());
            for (x, tile) in line.iter().enumerate().skip(1).take(line.len() - 2) {
                if new_tiles.contains(&(x as i32, y as i32)) {
                    row.push(Tile::Wall);
                } else {
                    row.push(Tile::Walkable);
                }
            }
            row.push(line.iter().last().unwrap().clone());
            tiles.push(row);
        }
        tiles.push(self.tiles.iter().last().unwrap().clone());
        Grid { tiles }
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let grid = Grid::parse(inp);
    let mut graph = Graph::<((usize, usize, usize), bool), usize>::new();
    let cycle_len = grid.get_cycle_len() + 1;
    let mut mappings = HashMap::new();
    // println!("Generating nodes for each minute");
    // Generate nodes for each minute
    for i in 0..=cycle_len {
        let g = grid.generate_cycle(i);
        for (y, line) in g.tiles.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let idx = graph.add_node((
                    (x, y, i),
                    if let Tile::Walkable = tile {
                        true
                    } else {
                        false
                    },
                ));
                mappings.insert((x, y, i), idx);
            }
        }
    }
    // println!("Generating edges connecting minutes");
    // Generate edges connecting minutes
    for nx in graph.node_indices() {
        let ((x, y, i), walkable) = graph[nx];
        if !walkable {
            continue;
        }
        let wait = Some((x, y, (i + 1) % cycle_len));
        let up = if y > 1 {
            Some((x, y - 1, (i + 1) % cycle_len))
        } else {
            None
        };
        let down = Some((x, y + 1, (i + 1) % cycle_len));
        let left = if x > 1 {
            Some((x - 1, y, (i + 1) % cycle_len))
        } else {
            None
        };
        let right = Some((x + 1, y, (i + 1) % cycle_len));
        for potential in [wait, up, down, right, left] {
            if let None = potential {
                continue;
            }
            let (x, y, i) = potential.unwrap();
            if !mappings.contains_key(&(x, y, i)) {
                continue;
            }
            let idx = mappings.get(&(x, y, i)).unwrap();
            graph.add_edge(nx, *idx, 1);
        }
    }
    // Find path
    let start = graph
        .node_indices()
        .find(|nx| {
            let ((_, y, i), walkable) = graph[*nx];
            y == 0 && i == 0 && walkable
        })
        .expect("Couldn't find start");
    let path = astar(
        &graph,
        start,
        |f| {
            let ((_, y, _), walkable) = graph[f];
            y == grid.tiles.len() - 1 && walkable
        },
        |_| 1,
        |_| 0,
    ).expect("Couldn't find path from start to finish");
    res.part_1 = path.0.to_string();
}
