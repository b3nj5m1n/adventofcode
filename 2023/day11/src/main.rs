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
    part_1: u128,
    part_2: u128,
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
    // let mut count = 0;
    /* for x in xs {
        for row in grid.iter_mut() {
            row.insert(x + count, Tile::Space);
        }
        count += 1;
    } */

    // Fill rows
    let mut ys = Vec::new();
    'outer: for y in 0..grid.len() {
        if grid[y].contains(&Tile::Galaxy) {
            continue 'outer;
        }
        ys.push(y);
    }
    // let mut count = 0;
    /* for y in ys {
        grid.insert(
            y + count,
            std::iter::repeat(Tile::Space).take(grid[0].len()).collect(),
        );
        count += 1;
    } */

    // Part 1
    let mut galaxies_cords_part_1 = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Tile::Galaxy {
                galaxies_cords_part_1.push((
                    (y + ys.iter().filter(|&e| y > *e).count() * 1) as i128,
                    (x + xs.iter().filter(|&e| x > *e).count() * 1) as i128,
                ));
            }
        }
    }
    let mut m = HashSet::new();
    for (a, b) in galaxies_cords_part_1
        .clone()
        .into_iter()
        .cartesian_product(galaxies_cords_part_1)
    {
        if m.contains(&(a, b)) || m.contains(&(b, a)) || a == b {
            continue;
        }
        m.insert((a, b));
        res.part_1 += ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u128;
    }

    // Part 2
    let factor = 1000000;
    let mut galaxies_cords_part_2 = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == Tile::Galaxy {
                galaxies_cords_part_2.push((
                    (y + ys.iter().filter(|&e| y > *e).count() * ( factor - 1 )) as i128,
                    (x + xs.iter().filter(|&e| x > *e).count() * ( factor - 1 )) as i128,
                ));
            }
        }
    }
    let mut m = HashSet::new();
    for (a, b) in galaxies_cords_part_2
        .clone()
        .into_iter()
        .cartesian_product(galaxies_cords_part_2)
    {
        if m.contains(&(a, b)) || m.contains(&(b, a)) || a == b {
            continue;
        }
        m.insert((a, b));
        res.part_2 += ((b.0 - a.0).abs() + (b.1 - a.1).abs()) as u128;
    }
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

// As usual made things harder for myself than necessary, for part 1 I actually created the whole
// graph using petgraph, then ran astar over all the galaxy nodes to find the solution. (about 3
// minutes of runtime)
// Struggled way too hard with this until I finally realised how easily you can compute the
// distance between two points and at that point it was pretty obvious you didn't actually have to
// insert any rows/columns for empty space.
// Took way longer than I would have liked but at least I didn't have to look up any solution like
// yesterday.
