use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io::Read;
use std::{env, thread, time};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Obstacle,
    Guard(Guard),
    Visited,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Empty => ".",
            Tile::Obstacle => "#",
            Tile::Guard(g) => match g {
                Guard::Up => "^",
                Guard::Down => "v",
                Guard::Right => ">",
                Guard::Left => "<",
            },
            Tile::Visited => "X",
        };
        write!(f, "{}", c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Guard {
    Up,
    Down,
    Right,
    Left,
}

fn print_grid(grid: &Vec<Vec<Tile>>) {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[y][x]);
        }
        print!("\n");
    }
}

fn run_simulation(
    guard_pos: Option<(usize, usize)>,
    grid: &Vec<Vec<Tile>>,
) -> Option<Vec<(usize, usize)>> {
    let mut guard_pos = guard_pos;
    let mut grid = grid.clone();
    let mut loop_points = HashSet::new();
    while let Some(guard_pos_) = guard_pos {
        let id = (guard_pos_, grid[guard_pos_.1][guard_pos_.0]);
        if loop_points.contains(&id) {
            return None;
        }
        loop_points.insert(id);
        // dbg!(guard_pos_);
        // print_grid(&grid);
        let target_pos = match grid[guard_pos_.1][guard_pos_.0] {
            Tile::Guard(g) => match g {
                Guard::Up => (guard_pos_.0 as i64, guard_pos_.1 as i64 - 1),
                Guard::Down => (guard_pos_.0 as i64, guard_pos_.1 as i64 + 1),
                Guard::Right => (guard_pos_.0 as i64 + 1, guard_pos_.1 as i64),
                Guard::Left => (guard_pos_.0 as i64 - 1, guard_pos_.1 as i64),
            },
            _ => unreachable!(),
        };
        if target_pos.0 < 0
            || target_pos.1 < 0
            || target_pos.0 >= grid[0].len() as i64
            || target_pos.1 >= grid.len() as i64
        {
            grid[guard_pos_.1][guard_pos_.0] = Tile::Visited;
            guard_pos = None;
        } else if grid[target_pos.1 as usize][target_pos.0 as usize] == Tile::Obstacle {
            match grid[guard_pos_.1][guard_pos_.0] {
                Tile::Guard(g) => match g {
                    Guard::Up => grid[guard_pos_.1][guard_pos_.0] = Tile::Guard(Guard::Right),
                    Guard::Down => grid[guard_pos_.1][guard_pos_.0] = Tile::Guard(Guard::Left),
                    Guard::Right => grid[guard_pos_.1][guard_pos_.0] = Tile::Guard(Guard::Down),
                    Guard::Left => grid[guard_pos_.1][guard_pos_.0] = Tile::Guard(Guard::Up),
                },
                _ => unreachable!(),
            };
        } else {
            // println!("Setting new guard pos");
            guard_pos = Some((target_pos.0 as usize, target_pos.1 as usize));
            grid[target_pos.1 as usize][target_pos.0 as usize] = grid[guard_pos_.1][guard_pos_.0];
            grid[guard_pos_.1][guard_pos_.0] = Tile::Visited;
        }
        // thread::sleep(time::Duration::from_secs(1));
    }
    Some(
        grid.iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .filter_map(|(x, t)| {
                        if let Tile::Visited = t {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>(),
    )
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // [y][x]
    let mut grid: Vec<Vec<Tile>> = inp
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| {
                    if c == '#' {
                        Tile::Obstacle
                    } else if c == '^' {
                        Tile::Guard(Guard::Up)
                    } else if c == 'v' {
                        Tile::Guard(Guard::Down)
                    } else if c == '>' {
                        Tile::Guard(Guard::Right)
                    } else if c == '<' {
                        Tile::Guard(Guard::Left)
                    } else {
                        Tile::Empty
                    }
                })
                .collect()
        })
        .collect();
    let initial_guard_pos = &grid
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.iter()
                .enumerate()
                .filter_map(move |(x, t)| match t {
                    Tile::Guard(_) => Some((x, y)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .filter(|n| n.len() != 0)
        .collect::<Vec<_>>()[0][0];
    let guard_pos = Some(*initial_guard_pos);

    let visited_coords_original_config = run_simulation(guard_pos, &grid).expect("");

    res.part_1 = visited_coords_original_config.len();

    let visited_coords_original_config: Vec<_> = visited_coords_original_config
        .into_iter()
        .filter(|(x, y)| !(*x == initial_guard_pos.0 && *y == initial_guard_pos.1))
        .collect();
    res.part_2 = visited_coords_original_config
        .iter()
        .map(|(x, y)| {
            println!("{}/{}", y, grid.len());
            let mut new_grid = grid.clone();
            new_grid[*y][*x] = Tile::Obstacle;
            if let None = run_simulation(guard_pos, &new_grid) {
                1
            } else {
                0
            }
        })
        .sum();
}
