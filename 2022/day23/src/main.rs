use std::collections::HashMap;
use std::env;
use std::io::Read;

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

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

fn round_1(
    grid: &HashMap<(i64, i64), bool>,
    directions: &Vec<Direction>,
) -> HashMap<(i64, i64), (i64, i64)> {
    let mut propsals = HashMap::new();
    let mut invalids = Vec::new();
    for (elf, v) in grid {
        if !v {
            continue;
        }
        // println!("{}, {}", elf.0, elf.1);
        let n = grid.get(&(elf.0, elf.1 - 1));
        let s = grid.get(&(elf.0, elf.1 + 1));
        let w = grid.get(&(elf.0 - 1, elf.1));
        let e = grid.get(&(elf.0 + 1, elf.1));
        let ne = grid.get(&(elf.0 + 1, elf.1 - 1));
        let nw = grid.get(&(elf.0 - 1, elf.1 - 1));
        let se = grid.get(&(elf.0 + 1, elf.1 + 1));
        let sw = grid.get(&(elf.0 - 1, elf.1 + 1));
        if n.is_none()
            && s.is_none()
            && w.is_none()
            && e.is_none()
            && ne.is_none()
            && nw.is_none()
            && se.is_none()
            && sw.is_none()
        {
            continue;
        }
        // println!("Making proposal");
        for direction in directions.iter() {
            let propose = match direction {
                Direction::North => n.is_none() && ne.is_none() && nw.is_none(),
                Direction::South => s.is_none() && se.is_none() && sw.is_none(),
                Direction::West => w.is_none() && nw.is_none() && sw.is_none(),
                Direction::East => e.is_none() && ne.is_none() && se.is_none(),
            };
            if propose {
                let key = match direction {
                    Direction::North => (elf.0, elf.1 - 1),
                    Direction::South => (elf.0, elf.1 + 1),
                    Direction::West => (elf.0 - 1, elf.1),
                    Direction::East => (elf.0 + 1, elf.1),
                };
                // println!("Proposing to move to ({}, {})", key.0, key.1);
                if propsals.contains_key(&key) {
                    // println!("Already a proposal for this tile, no one moves");
                    invalids.push(key);
                } else {
                    propsals.insert(key, *elf);
                }
                break;
            }
        }
    }
    for invalid in invalids {
        propsals.remove(&invalid);
    }
    propsals
}

fn round_2(grid: &mut HashMap<(i64, i64), bool>, proposals: HashMap<(i64, i64), (i64, i64)>) {
    for (to, from) in proposals {
        grid.remove(&from);
        grid.insert(to, true);
    }
}

fn get_bounds(grid: &HashMap<(i64, i64), bool>) -> (i64, i64, i64, i64) {
    let first = grid.keys().next().unwrap();
    let mut bounds = (first.0, first.0, first.1, first.1);
    for elf in grid.keys() {
        if elf.0 < bounds.0 {
            bounds.0 = elf.0;
        }
        if elf.0 > bounds.1 {
            bounds.1 = elf.0;
        }
        if elf.1 < bounds.2 {
            bounds.2 = elf.1;
        }
        if elf.1 > bounds.3 {
            bounds.3 = elf.1;
        }
    }
    bounds
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid = HashMap::new();
    for (y, line) in inp.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            /* grid.insert(
                (x.try_into().unwrap(), y.try_into().unwrap()),
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unkonwn symbol"),
                },
            ); */
            if c == '#' {
                grid.insert((x.try_into().unwrap(), y.try_into().unwrap()), true);
            }
        }
    }
    let mut directions = vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];
    let mut first_no_move = -1;
    for i in 0..10 {
        let r1 = round_1(&grid, &directions);
        if r1.len() == 0 {
            first_no_move = i + 1;
        }
        round_2(&mut grid, r1);
        directions.rotate_left(1);
    }
    let bounds = get_bounds(&grid);
    let mut count = 0;
    for x in bounds.0..=bounds.1 {
        for y in bounds.2..=bounds.3 {
            if grid.get(&(x, y)).is_none() {
                count += 1;
            }
        }
    }
    res.part_1 = count.to_string();
    if first_no_move == -1 {
        for i in 10.. {
            let r1 = round_1(&grid, &directions);
            if r1.len() == 0 {
                first_no_move = i + 1;
                break;
            }
            round_2(&mut grid, r1);
            directions.rotate_left(1);
        }
    }
    res.part_2 = first_no_move.to_string();
}
