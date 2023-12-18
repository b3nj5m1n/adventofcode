// I got very close on my own, eventually gave up because I couldn't figure out why the shoelace
// formula wasn't working but stumbled upon Picks Theorem on reddit a little while later.

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
    part_1: i64,
    part_2: i64,
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
            "U" | "3" => Ok(Self::Up),
            "D" | "1" => Ok(Self::Down),
            "L" | "2" => Ok(Self::Left),
            "R" | "0" => Ok(Self::Right),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DigInstruction {
    direction: Direction,
    count: u32,
    color: String,
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
        Ok(Self {
            direction,
            count,
            color: s_color[2..8].to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Point {
    x: i64,
    y: i64,
}

fn solve_grid(grid: HashSet<Point>, bound_top_left: Point, bound_bottom_right: Point) -> usize {
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
    // println!("Finished traversing horizontal edges");
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
    // println!("Finished traversing vertical edges");
    let mut to_search_open_edges = VecDeque::new();
    for open_edge in open_edges.iter() {
        to_search_open_edges.push_front(open_edge.clone());
    }
    open_edges.drain();
    // println!("Starting to flood fill open edges");
    let mut i = 0;
    while !to_search_open_edges.is_empty() {
        i += 1;
        if i % 100000 == 0 {
            println!("{i} ({})", to_search_open_edges.len());
        }
        let current = to_search_open_edges.pop_back().expect("Unreachable");
        if open_edges.contains(&current) {
            continue;
        }
        if !grid.contains(&current) {
            open_edges.insert(current);
        } else {
            continue;
        }
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
                    // open_edges.insert(neighbor);
                    to_search_open_edges.push_front(neighbor);
                }
            }
        }
    }
    // println!("Finished flood fill of open edges");
    ((1 + bound_bottom_right.x.abs_diff(bound_top_left.x))
        * (1 + bound_bottom_right.y.abs_diff(bound_top_left.y))) as usize
        - open_edges.len()
}

// https://stackoverflow.com/a/6989383
fn point_less_clockwise(
    a: Point,
    b: Point,
    bound_top_left: Point,
    bound_bottom_right: Point,
) -> bool {
    let center = Point {
        x: (bound_top_left.x + bound_bottom_right.x) / 2,
        y: (bound_top_left.y + bound_bottom_right.y) / 2,
    };
    let center = Point::default();
    if a.x - center.x >= 0 && b.x - center.x < 0 {
        return true;
    }
    if a.x - center.x < 0 && b.x - center.x >= 0 {
        return false;
    }
    if a.x - center.x == 0 && b.x - center.x == 0 {
        if a.y - center.y >= 0 || b.y - center.y >= 0 {
            return a.y > b.y;
        }
        return b.y > a.y;
    }

    let det = (a.x - center.x) * (b.y - center.y) - (b.x - center.x) * (a.y - center.y);
    if det < 0 {
        return true;
    }
    if det > 0 {
        return false;
    }

    let d1 = (a.x - center.x) * (a.x - center.x) + (a.y - center.y) * (a.y - center.y);
    let d2 = (b.x - center.x) * (b.x - center.x) + (b.y - center.y) * (b.y - center.y);
    return d1 > d2;
}

fn get_area(instructions: &Vec<DigInstruction>) -> i64 {
    let mut bound_top_left = Point::default();
    let mut bound_bottom_right = Point::default();
    let mut current = Point::default();
    let mut grid = HashSet::new();
    grid.insert(current);
    let mut perimiter = 0;
    let mut edge_points = Vec::new();
    for instruction in instructions {
        let dir = instruction.direction.get_direction();
        edge_points.push(current);
            current = Point {
                x: current.x + dir.x * instruction.count as i64,
                y: current.y + dir.y * instruction.count as i64,
            };
            perimiter += instruction.count as i64;
        /* for offset in 1..=instruction.count {
            perimiter += 1;
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
        } */
    }
    /* edge_points.sort_by(|a, b| {
        let less = point_less_clockwise(*a, *b, bound_top_left, bound_bottom_right);
        if less {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }); */
    // Shoelace Formula
    let mut area = 0;
    let mut p0 = edge_points[edge_points.len() - 1];
    for p1 in edge_points.iter() {
        area += (p0.y * p1.x) - (p0.x * p1.y);
        p0 = *p1;
    }
    area = area / 2;
    area = area.abs();
    // Picks's theorem
    area = area + (perimiter / 2) + 1;
    // dbg!(&edge_points);
    // dbg!(area);
    // println!("Finished creating grid");
    // (grid, bound_top_left, bound_bottom_right, edge_points)
    area
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let instructions: Vec<_> = inp
        .into_iter()
        .map(|l| DigInstruction::from_str(l).expect("Failed parsing dig instruction"))
        .collect();
    // let (grid, bound_top_left, bound_bottom_right, edge_points) = get_area(&instructions);
    /* for edge_point in edge_points.iter() {
        println!(
            "({}, {})",
            yansi::Paint::new(edge_point.x).fg(yansi::Color::Fixed(
                (edge_point.x + edge_point.y << 5 | 0b100101 % 255) as u8,
            )),
            yansi::Paint::new(edge_point.y).fg(yansi::Color::Fixed(
                (edge_point.x + edge_point.y << 5 | 0b100101 % 255) as u8,
            )),
        );
    } */
    /* for y in bound_top_left.y..=bound_bottom_right.y {
        for x in bound_top_left.x..=bound_bottom_right.x {
            if edge_points.contains(&Point { x, y }) {
                print!(
                    "{}",
                    yansi::Paint::new(y).fg(yansi::Color::Fixed(
                        (x + y << 5 | 0b100101 % 255) as u8,
                    )),
                );
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
    res.part_1 = get_area(&instructions);

    /* let mut a = 0;
    for y in bound_top_left.y..=bound_bottom_right.y {
        for x in bound_top_left.x..=bound_bottom_right.x {

        }
    }
    dbg!(a); */

    let instructions: Vec<_> = instructions
        .into_iter()
        .map(|i| {
            let new_dir = Direction::from_str(&i.color.as_str()[5..6]).expect("Unreachable");
            let new_count = u32::from_str_radix(&i.color.as_str()[0..5], 16).expect("Unreachable");
            DigInstruction {
                direction: new_dir,
                count: new_count,
                color: "".to_string(),
            }
        })
        .collect();
    /* let (grid, bound_top_left, bound_bottom_right) = create_grid(&instructions);
    res.part_2 = solve_grid(grid, bound_top_left, bound_bottom_right); */
    res.part_2 = get_area(&instructions);
}
