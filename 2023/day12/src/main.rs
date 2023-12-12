use std::collections::VecDeque;
use std::env;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

use cached::proc_macro::cached;

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
    part_1: u64,
    part_2: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Broken,
    Operational,
    Unknown,
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Broken),
            "." => Ok(Self::Operational),
            "?" => Ok(Self::Unknown),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    tiles: Vec<Tile>,
    nums: Vec<u64>,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (tiles, nums) = s
            .split_once(" ")
            .ok_or(anyhow::anyhow!("Unexpected format"))?;
        let tiles = tiles
            .chars()
            .map(|c| Tile::from_str(&c.to_string()))
            .collect::<std::result::Result<Vec<_>, anyhow::Error>>()?;
        let nums = nums
            .split(",")
            .map(|n| n.parse::<u64>())
            .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;

        Ok(Self { tiles, nums })
    }
}

#[cached]
fn num_of_valid_options(
    tiles: Vec<Tile>,
    stack: VecDeque<u64>,
    // start_at: usize,
    current: u64,
    switched: bool,
    count_damaged: u64,
) -> u64 {
    // println!("{}", tiles.len());
    // let mut count_damaged = 0;
    let mut count_damaged = count_damaged;
    // let mut current = stack.pop_front().expect("Stack unexpectedly empty");
    let mut current = current;
    // let mut switched = false;
    let mut switched = switched;
    let mut stack = stack;
    for (i, tile) in tiles.iter().enumerate() {
        // println!("On tile {i} ({tile:?}), current: {current}, count_damaged: {count_damaged}");
        if *tile == Tile::Broken {
            count_damaged += 1;
            if switched {
                // println!( "Path wasn't valid, hit broken tile while after completing a broken block.");
                return 0;
            }
        } else if *tile == Tile::Operational && count_damaged != 0 {
            // println!("Path wasn't valid, hit operational tile while in the middle of a broken block.");
            return 0;
        } else if *tile == Tile::Unknown {
            let mut tiles: Vec<Tile> = tiles
                .clone()
                .into_iter()
                .skip(i)
                .collect();
            tiles[0] = Tile::Broken;
            let broken = tiles.clone();
            if count_damaged < current && count_damaged != 0 {
                // println!("Recursion case 1");
                return num_of_valid_options(
                    broken,
                    stack.clone(),
                    current,
                    switched,
                    count_damaged,
                );
            }
            tiles[0] = Tile::Operational;
            let operational = tiles.clone();
            // println!("Recursion case 2");
            return num_of_valid_options(
                operational,
                stack.clone(),
                current,
                switched,
                count_damaged,
            ) + num_of_valid_options(
                broken,
                stack.clone(),
                current,
                switched,
                count_damaged,
            );
        }
        if count_damaged == current {
            current = match stack.pop_front() {
                Some(n) => n,
                None => {
                    for j in (i + 1)..tiles.len() {
                        if tiles[j] == Tile::Broken {
                            return 0;
                        }
                    }
                    return 1;
                }
            };
            count_damaged = 0;
            switched = true;
        } else {
            switched = false;
        }
    }
    // println!("Path wasn't valid, hit end of tiles before exhausting stack.");
    return 0;
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let lines = inp
        .into_iter()
        .map(|l| Line::from_str(l))
        .collect::<std::result::Result<Vec<Line>, anyhow::Error>>()
        .expect("Parsing failed");
    for line in lines.clone() {
        let mut stack = VecDeque::from(line.nums.clone());
        let first = stack.pop_front().expect("Stack unexpectedly empty.");
        res.part_1 += num_of_valid_options(line.tiles, stack, first, false, 0);
    }

    // Part 2
    let lines = lines.into_iter().map(|l| {
        let mut new_tiles = l.tiles.clone();
        new_tiles.push(Tile::Unknown);
        let mut new_tiles = new_tiles.repeat(5);
        new_tiles.pop();
        Line {
            tiles: new_tiles,
            nums: l.nums.repeat(5),
        }
    });
    for line in lines {
        let mut stack = VecDeque::from(line.nums.clone());
        let first = stack.pop_front().expect("Stack unexpectedly empty.");
        let result = num_of_valid_options(line.tiles, stack, first, false, 0);
        res.part_2 += result;
    }
}
