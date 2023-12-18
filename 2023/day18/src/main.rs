use std::collections::{HashSet, VecDeque};
use std::env;
use std::io::Read;
use std::str::FromStr;

use rgb::*;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => Err(anyhow::anyhow!("Couldn't parse direction")),
        }
    }
}
impl Direction {
    fn get_direction(&self) -> Point {
        match self {
            Direction::Up => Point { x: 0, y: -1 },
            Direction::Down => Point { x: 0, y: 1 },
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct DigInstruction {
    direction: Direction,
    count: u32,
    color: RGB8,
}
impl FromStr for DigInstruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (s_direction, s_count, s_color) =
            if let (Some(s_direction), Some(s_count), Some(s_color)) =
                (parts.next(), parts.next(), parts.next())
            {
                (s_direction, s_count, s_color)
            } else {
                return Err(anyhow::anyhow!("Failed parsing dig instruction"));
            };
        let direction = Direction::from_str(s_direction)?;
        let count = s_count.parse::<u32>()?;
        let r_hex = &s_color[2..4];
        let g_hex = &s_color[4..6];
        let b_hex = &s_color[6..8];
        let r = u8::from_str_radix(r_hex, 16)?;
        let g = u8::from_str_radix(g_hex, 16)?;
        let b = u8::from_str_radix(b_hex, 16)?;
        Ok(Self {
            direction,
            count,
            color: RGB8 { r, g, b },
        })
    }
}
/* impl DigInstruction {
    fn get_direction(&self) -> Vec<Point> {
        let dir = self.direction.get_direction();
        let mut result = Vec::new();
        for i in 1..=self.count as i64 {
            result.push(Point {
                x: dir.x * i,
                y: dir.y * i,
            });
        }
        /* println!("Directions for {self:?}");
        dbg!(&result); */
        result
    }
} */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Point {
    x: i64,
    y: i64,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let instructions: Vec<_> = inp
        .into_iter()
        .map(|l| DigInstruction::from_str(l).expect("Failed parsing dig instruction"))
        .collect();
    // dbg!(&instructions);
    let mut bound_top_left = Point::default();
    let mut bound_bottom_right = Point::default();
    let mut current = Point::default();
    let mut grid = HashSet::new();
    grid.insert(current);
    for instruction in instructions {
        let dir = instruction.direction.get_direction();
        for offset in 1..=instruction.count {
            current = Point {
                x: current.x + dir.x,
                y: current.y + dir.y,
            };
            // dbg!(current);
            grid.insert(current);
            bound_top_left = Point {
                x: bound_top_left.x.min(current.x),
                y: bound_top_left.y.min(current.y),
            };
            bound_bottom_right = Point {
                x: bound_bottom_right.x.max(current.x),
                y: bound_bottom_right.y.max(current.y),
            };
        }
    }
    /* for y in bound_top_left.y..=bound_bottom_right.y {
        let mut inside = false;
        let mut last = false;
        for x in bound_top_left.x..=bound_bottom_right.x {
            if grid.contains(&Point { x, y }) {
                if !last {
                    inside = !inside;
                }
                last = true;
            } else {
                last = false;
            }
            if inside {
                grid.insert(Point { x, y });
            }
        }
    } */
    let mut open_edges = HashSet::new();
    for y in bound_top_left.y..=bound_bottom_right.y {
        let top = Point {
            x: bound_top_left.x,
            y,
        };
        if !grid.contains(&top) {
            open_edges.insert(top);
        }
        let bottom = Point {
            x: bound_bottom_right.x,
            y,
        };
        if !grid.contains(&bottom) {
            open_edges.insert(bottom);
        }
    }
    for x in bound_top_left.x..=bound_bottom_right.x {
        let top = Point {
            x,
            y: bound_top_left.y,
        };
        if !grid.contains(&top) {
            open_edges.insert(top);
        }
        let bottom = Point {
            x,
            y: bound_bottom_right.y,
        };
        if !grid.contains(&bottom) {
            open_edges.insert(bottom);
        }
    }
    let mut to_search_open_edges = VecDeque::new();
    for open_edge in open_edges.iter() {
        to_search_open_edges.push_front(open_edge.clone());
    }
    while !to_search_open_edges.is_empty() {
        let current = to_search_open_edges.pop_front().expect("Unreachable");
        for dir in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let neighbor = Point {
                x: current.x + dir.0,
                y: current.y + dir.1,
            };
            // println!("Checking {neighbor:?}");
            if !(neighbor.x > bound_top_left.x
                && neighbor.x < bound_bottom_right.x
                && neighbor.y > bound_top_left.y
                && neighbor.y < bound_bottom_right.y)
            {
                // println!("Skipping");
                continue;
            }
            if !grid.contains(&neighbor) {
                if !open_edges.contains(&neighbor) {
                    open_edges.insert(neighbor);
                    to_search_open_edges.push_front(neighbor);
                }
            }
        }
    }
    // dbg!(&open_edges);
    /* for y in bound_top_left.y..=bound_bottom_right.y {
        for x in bound_top_left.x..=bound_bottom_right.x {
            if open_edges.contains(&Point { x, y }) {
                print!("O");
                continue;
            }
            if grid.contains(&Point { x, y }) {
                print!("#");
                // res.part_1 += 1;
            } else {
                print!(".");
            }
        }
        print!("\n");
    } */
    res.part_1 = ( (1 + bound_bottom_right.x.abs_diff(bound_top_left.x) ) * (1+ bound_bottom_right.y.abs_diff(bound_top_left.y) ) ) as usize - open_edges.len();
}
