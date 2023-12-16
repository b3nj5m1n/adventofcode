// Pretty easy problem, too lazy to improve this code

use std::collections::{HashMap, HashSet};
use std::env;
use std::io::Read;
use std::str::FromStr;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tile {
    Space,
    MirrorTop,
    MirrorBottom,
    SplitterH,
    SplitterV,
}
impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Space),
            "/" => Ok(Self::MirrorTop),
            "\\" => Ok(Self::MirrorBottom),
            "-" => Ok(Self::SplitterH),
            "|" => Ok(Self::SplitterV),
            _ => Err(anyhow::anyhow!("Failed parsing tile {}", s)),
        }
    }
}
impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Space => ".".to_string(),
            Tile::MirrorTop => "/".to_string(),
            Tile::MirrorBottom => "\\".to_string(),
            Tile::SplitterH => "-".to_string(),
            Tile::SplitterV => "|".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    xy: (usize, usize),
    direction: (i8, i8),
}
impl Default for Position {
    fn default() -> Self {
        Self {
            xy: (0, 0),
            direction: (1, 0),
        }
    }
}
impl Position {
    fn move_next(&self, grid_size: (usize, usize)) -> Option<(usize, usize)> {
        let new_x = match self.direction.0 {
            0 => self.xy.0,
            1 => match self.xy.0 + 1 {
                x if x < grid_size.0 => x,
                _ => return None,
            },
            -1 => match self.xy.0.checked_sub(1) {
                Some(x) => x,
                None => return None,
            },
            _ => unreachable!(),
        };
        let new_y = match self.direction.1 {
            0 => self.xy.1,
            1 => match self.xy.1 + 1 {
                x if x < grid_size.1 => x,
                _ => return None,
            },
            -1 => match self.xy.1.checked_sub(1) {
                Some(x) => x,
                None => return None,
            },
            _ => unreachable!(),
        };
        Some((new_x, new_y))
    }
}

fn get_energised(grid: Vec<Vec<Tile>>, starting_position: Position) -> usize {
    let grid_size = (grid[0].len(), grid.len());
    let mut positions = vec![starting_position];
    let mut energised = HashSet::new();
    let mut map = HashSet::new();
    let mut i = 0;
    while !positions.is_empty() {
        i += 1;
        /* if i > 20 {
            panic!()
        } */
        /* {
            let mut line = String::new();
            let input = std::io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
        } */
        // println!("{}", energised.len());
        // dbg!(&positions);
        let position = positions.pop().expect("Unreachable");
        // draw(position, grid.clone());
        if map.contains(&position) {
            // println!("Reached loop, abandoning path");
            continue;
        }
        if i != 1 {
            map.insert(position);
        }
        energised.insert(position.xy);
        let mut new_xy = match position.move_next(grid_size) {
            Some(xy) => xy,
            None => {
                // println!("Reached end of beam at {position:?}");
                continue;
            }
        };
        // This is a crime
        if i == 1 {
            new_xy = position.xy;
        }
        /* println!(
            "Current position: {new_xy:?} ({:?})",
            grid[new_xy.1][new_xy.0]
        );
        dbg!(grid[new_xy.1][new_xy.0]); */
        match grid[new_xy.1][new_xy.0] {
            Tile::Space => {
                positions.push(Position {
                    xy: new_xy,
                    direction: position.direction,
                })
            }
            Tile::MirrorTop => {
                let new_direction = match position.direction {
                    (1, 0) => (0, -1),
                    (-1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (0, -1) => (1, 0),
                    _ => unreachable!(),
                };
                positions.push(Position {
                    xy: new_xy,
                    direction: new_direction,
                })
            }
            Tile::MirrorBottom => {
                let new_direction = match position.direction {
                    (1, 0) => (0, 1),
                    (-1, 0) => (0, -1),
                    (0, 1) => (1, 0),
                    (0, -1) => (-1, 0),
                    _ => unreachable!(),
                };
                positions.push(Position {
                    xy: new_xy,
                    direction: new_direction,
                })
            }
            Tile::SplitterH => match position.direction {
                (1, 0) | (-1, 0) => {
                    positions.push(Position {
                        xy: new_xy,
                        direction: position.direction,
                    });
                }
                (0, 1) | (0, -1) => {
                    positions.push(Position {
                        xy: new_xy,
                        direction: (1, 0),
                    });
                    positions.push(Position {
                        xy: new_xy,
                        direction: (-1, 0),
                    });
                }
                _ => unreachable!(),
            },
            Tile::SplitterV => match position.direction {
                (1, 0) | (-1, 0) => {
                    positions.push(Position {
                        xy: new_xy,
                        direction: (0, 1),
                    });
                    positions.push(Position {
                        xy: new_xy,
                        direction: (0, -1),
                    });
                }
                (0, 1) | (0, -1) => {
                    positions.push(Position {
                        xy: new_xy,
                        direction: position.direction,
                    });
                }
                _ => unreachable!(),
            },
        }
    }
    /* for line in inp {
        println!("{}", line)
    } */
    energised.len()
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let grid: Vec<Vec<Tile>> = inp
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).expect("Failed parsing tile"))
                .collect()
        })
        .collect();
    res.part_1 = get_energised(grid.clone(), Position::default());
    // dbg!(Position::default());
    let mut max = 0;
    for x in 0..grid[0].len() {
        let pos = Position { xy: (x, 0), direction: (0, 1) };
        let e = get_energised(grid.clone(), pos);
        // println!("Checking {pos:?}, got {e}");
        if e > max {
            max = e;
        }
    }
    for x in 0..grid[0].len() {
        let pos = Position { xy: (x, grid.len()-1), direction: (0, -1) };
        let e = get_energised(grid.clone(), pos);
        // println!("Checking {pos:?}, got {e}");
        if e > max {
            max = e;
        }
    }
    for y in 0..grid.len() {
        let pos = Position { xy: (0, y), direction: (1, 0) };
        let e = get_energised(grid.clone(), pos);
        // println!("Checking {pos:?}, got {e}");
        if e > max {
            max = e;
        }
    }
    for y in 0..grid.len() {
        let pos = Position { xy: (grid[0].len()-1, y), direction: (-1, 0) };
        let e = get_energised(grid.clone(), pos);
        // println!("Checking {pos:?}, got {e}");
        if e > max {
            max = e;
        }
    }
    res.part_2 = max;
}

fn draw(position: Position, grid: Vec<Vec<Tile>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if position.xy == (x, y) {
                print!(
                    "{}",
                    yansi::Paint::new(grid[y][x].to_string())
                        .fg(yansi::Color::Red)
                        .bold()
                );
            } else {
                print!("{}", grid[y][x].to_string());
            }
        }
        print!("\n");
    }
}
