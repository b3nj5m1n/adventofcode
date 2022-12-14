use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::io::Read;

use nom::bytes::complete::tag;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
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
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: String,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn down_left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    fn down_right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, (x, y)) = separated_pair(
        nom::character::complete::u32,
        tag(","),
        nom::character::complete::u32,
    )(input)?;
    Ok((input, Point { x, y }))
}

fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list0(tag(" -> "), point)(input)
}

fn simulate_sand(
    sand_origin: &Point,
    rocks: &HashSet<Point>,
    sand: &mut HashSet<Point>,
    lowest_point: u32,
    p2: bool,
) -> bool {
    let mut current_pos = Point {
        x: sand_origin.x,
        y: sand_origin.y,
    };
    if sand.contains(sand_origin) {
        return false;
    }
    loop {
        if current_pos.y > lowest_point && !p2 {
            return false;
        } else if p2 && current_pos.y == lowest_point + 1 {
            break;
        } else if !rocks.contains(&current_pos.down()) && !sand.contains(&current_pos.down()) {
            current_pos = current_pos.down();
        } else if !rocks.contains(&current_pos.down_left())
            && !sand.contains(&current_pos.down_left())
        {
            current_pos = current_pos.down_left();
        } else if !rocks.contains(&current_pos.down_right())
            && !sand.contains(&current_pos.down_right())
        {
            current_pos = current_pos.down_right();
        } else {
            break;
        }
    }
    sand.insert(current_pos);
    true
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut paths = Vec::new();
    let mut rocks = HashSet::new();
    for line in inp {
        paths.push(path(line).unwrap().1);
    }
    let mut lowest_point = 0;
    for path in paths {
        for pair in path.windows(2) {
            let x1 = min(pair.get(0).unwrap().x, pair.get(1).unwrap().x);
            let x2 = max(pair.get(0).unwrap().x, pair.get(1).unwrap().x);
            let y1 = min(pair.get(0).unwrap().y, pair.get(1).unwrap().y);
            let y2 = max(pair.get(0).unwrap().y, pair.get(1).unwrap().y);
            for x in x1..=x2 {
                for y in y1..=y2 {
                    rocks.insert(Point { x, y });
                    if y > lowest_point {
                        lowest_point = y;
                    }
                }
            }
        }
    }
    let mut sand = HashSet::new();
    let sand_origin = Point { x: 500, y: 0 };
    let mut i = 0;
    while simulate_sand(&sand_origin, &rocks, &mut sand, lowest_point, false) {
        i += 1;
    }
    res.part_1 = i.to_string();
    while simulate_sand(&sand_origin, &rocks, &mut sand, lowest_point, true) {
        i += 1;
    }
    res.part_2 = i.to_string();
}
