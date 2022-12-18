// Managed to do part 1 farily quickly and without help, currently too tired to do part 2
use std::borrow::Borrow;
use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::iter::Peekable;
use std::iter::{Cycle, Enumerate};
use std::vec::IntoIter;

use kdam::tqdm;

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

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
}
#[derive(Debug)]
struct Shape {
    rocks: Vec<Vec<bool>>,
    x: u64,
    y: u64,
}

fn draw_grid(grid: &HashSet<(u64, u64)>) {
    let mut result = Vec::new();
    for y in (0..10).into_iter().rev() {
        let mut row = Vec::new();
        for x in 0..7 {
            if grid.contains(&(x, y)) {
                row.push("#");
            } else {
                row.push(".");
            }
        }
        result.push(row.into_iter().collect::<String>());
    }
    println!("\n{}\n", result.join("\n"));
}

fn fits_shape(shape: &Shape, vector: (i64, i64), grid: &HashSet<(u64, u64)>) -> bool {
    let mut fits = true;
    for (y_offset, row) in shape.rocks.iter().rev().enumerate() {
        for (x_offset, rock) in row.iter().enumerate() {
            let x = (shape.x as i64 + x_offset as i64) + vector.0;
            if x < 0 {
                return false;
            }
            let y = (shape.y as i64 + y_offset as i64) + vector.1;
            if y < 0 {
                return false;
            }
            if grid.contains(&(x as u64, y as u64)) && *rock {
                return false;
            }
        }
    }
    fits
}

fn settle_shape(shape: &Shape, grid: &mut HashSet<(u64, u64)>) {
    for (y_offset, row) in shape.rocks.iter().rev().enumerate() {
        for (x_offset, rock) in row.iter().enumerate() {
            let x = shape.x as i64 + x_offset as i64;
            let y = shape.y as i64 + y_offset as i64;
            if *rock {
                grid.insert((x as u64, y as u64));
            }
        }
    }
}

fn get_max(grid: &HashSet<(u64, u64)>) -> u64 {
    let mut hightest_point = 0;
    for (_, y) in grid.iter() {
        hightest_point = max(hightest_point, *y + 1);
    }
    hightest_point
}

fn drop_shape(
    shape: &(usize, Vec<Vec<bool>>),
    grid: &mut HashSet<(u64, u64)>,
    // jet: &mut Cycle<std::slice::Iter<'_, Direction>>,
    // jet: &mut std::slice::Iter<'_, Direction>,
    // jet: &mut Cycle<Enumerate<std::slice::Iter<'_, Direction>>>,
    jet: &mut Peekable<Cycle<Enumerate<std::slice::Iter<'_, Direction>>>>,
) -> u64 {
    let hightest_point = get_max(&(*grid));
    let mut new_shape = Shape {
        x: 2,
        rocks: shape.1.clone(),
        y: hightest_point + 3,
    };
    let mut i = 0;
    let mut jet_indx = 0;
    // println!("New rock begins falling.");
    loop {
        /* if jet_indx == 0 && shape.0 == 0 {
            let m = get_max(grid).clone();
            if m != 0 {
                return m;
                /* println!("{}", m);
                panic!(); */
            }
        } */
        if i % 2 != 0 {
            let can_move_down = fits_shape(&new_shape, (0, -1), grid);
            if can_move_down {
                new_shape = Shape {
                    y: new_shape.y - 1,
                    ..new_shape
                };
                // println!("Rock falls 1 unit");
            }
            if !can_move_down {
                // println!("Rock comes to rest");
                break;
            }
        } else {
            let can_move_right = fits_shape(&new_shape, (1, 0), grid);
            let can_move_left = fits_shape(&new_shape, (-1, 0), grid);
            let dir = jet.next().unwrap();
            jet_indx = dir.0;
            // println!("{} {}", dir.0, shape.0);
            match (dir.1, can_move_right, can_move_left) {
                (Direction::Right, true, _) => {
                    if 7 - (new_shape.rocks[0].len() as i64) >= new_shape.x as i64 + 1 {
                        new_shape = Shape {
                            x: new_shape.x + 1,
                            ..new_shape
                        }
                    }
                }
                (Direction::Left, _, true) => {
                    new_shape = Shape {
                        x: new_shape.x - 1,
                        ..new_shape
                    }
                }
                _ => {}
            }
            // println!("Jet of gas pushes rock {dir:?}");
        }
        let mut tmp_grid = grid.clone();
        settle_shape(&new_shape, &mut tmp_grid);
        // draw_grid(&tmp_grid);
        i += 1;
    }
    settle_shape(&new_shape, grid);
    0
    // dbg!(&new_shape);
}

fn clean_grid(grid: &mut HashSet<(u64, u64)>) {
    let mut cutoff = 0;
    /* for y in (0..get_max(&(*grid))).into_iter().rev() {
        let mut full = true;
        for x in 0..7 {
            if !grid.contains(&(x, y)) {
                full = false;
            }
        }
        if full {
            cutoff = y;
            break;
        }
    } */
    cutoff = get_max(grid) - 20;
    for y in 0..cutoff {
        for x in 0..7 {
            grid.remove(&(x, y));
        }
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let jet_stream = inp[0]
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("Unexpected input"),
        })
        .collect::<Vec<Direction>>();
    // dbg!(jet_stream);
    let mut grid = HashSet::new();
    let shape1 = vec![vec![true, true, true, true]];
    let shape2 = vec![
        vec![false, true, false],
        vec![true, true, true],
        vec![false, true, false],
    ];
    let shape3 = vec![
        vec![false, false, true],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let shape4 = vec![vec![true], vec![true], vec![true], vec![true]];
    let shape5 = vec![vec![true, true], vec![true, true]];
    let mut shapes = vec![shape1, shape2, shape3, shape4, shape5]
        .into_iter()
        .enumerate()
        .cycle()
        .peekable();
    let mut jet = jet_stream.iter().enumerate().cycle().peekable();
    // draw_grid(&grid);
    for i in tqdm!(0..2022_i64) {
    // for i in tqdm!(0..1000000000000_i64) {
        let y = drop_shape(&shapes.next().unwrap(), &mut grid, &mut jet);
        /* if y != 0 {
            println!("{i} -> {y}");
            // panic!();
        } */
        /* if i % 5000 == 0 {
            clean_grid(&mut grid);
        } */
    }
    draw_grid(&grid);
    dbg!(get_max(&grid));
}
