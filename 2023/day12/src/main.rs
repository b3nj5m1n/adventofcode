use std::collections::VecDeque;
use std::env;
use std::io::Read;
use std::num::ParseIntError;
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
    part_1: u32,
    part_2: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line {
    tiles: Vec<Tile>,
    nums: Vec<u32>,
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
            .map(|n| n.parse::<u32>())
            .collect::<std::result::Result<Vec<_>, ParseIntError>>()?;

        Ok(Self { tiles, nums })
    }
}

impl Line {
    fn num_of_valid_options(&self) -> u32 {
        let mut stack = VecDeque::from(self.nums.clone());
        let mut count_damaged = 0;
        let mut current = stack.pop_front().expect("Stack unexpectedly empty");
        let mut switched = false;
        for (i, tile) in self.tiles.iter().enumerate() {
            // println!("On tile {i} ({tile:?})");
            if *tile == Tile::Broken {
                count_damaged += 1;
                if switched {
                    return 0;
                }
            } else if *tile == Tile::Operational && count_damaged != 0 {
                return 0;
            } else if *tile == Tile::Unknown {
                let mut tiles = self.tiles.clone();
                tiles[i] = Tile::Operational;
                let operational = Self {
                    tiles: tiles.clone(),
                    nums: self.nums.clone(),
                };
                tiles[i] = Tile::Broken;
                let broken = Self {
                    tiles: tiles.clone(),
                    nums: self.nums.clone(),
                };
                return operational.num_of_valid_options() + broken.num_of_valid_options();
            }
            if count_damaged == current {
                current = match stack.pop_front() {
                    Some(n) => n,
                    None => {
                        for j in (i + 1)..self.tiles.len() {
                            if self.tiles[j] == Tile::Broken {
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
        return 0;
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let lines = inp
        .into_iter()
        .map(|l| Line::from_str(l))
        .collect::<std::result::Result<Vec<Line>, anyhow::Error>>()
        .expect("Parsing failed");
    for line in lines {
        // println!("{:?}", line);
        res.part_1 += line.num_of_valid_options();
    }
}
