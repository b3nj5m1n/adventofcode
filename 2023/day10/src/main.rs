use std::collections::HashSet;
use std::env;
use std::io::Read;
use std::str::FromStr;

use anyhow;

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
    part_2: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl FromStr for Tile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "|" => Ok(Self::NorthSouth),
            "-" => Ok(Self::EastWest),
            "L" => Ok(Self::NorthEast),
            "J" => Ok(Self::NorthWest),
            "7" => Ok(Self::SouthWest),
            "F" => Ok(Self::SouthEast),
            "." => Ok(Self::Ground),
            "S" => Ok(Self::Start),
            _ => Err(anyhow::anyhow!("Invalid tile identifier")),
        }
    }
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::NorthSouth => "┃".to_string(),
            Tile::EastWest => "━".to_string(),
            Tile::NorthEast => "┗".to_string(),
            Tile::NorthWest => "┛".to_string(),
            Tile::SouthWest => "┓".to_string(),
            Tile::SouthEast => "┏".to_string(),
            Tile::Ground => ".".to_string(),
            Tile::Start => "S".to_string(),
        }
    }
}

// Get neighbours you can legally travel to, for tiles in the loop this always returns 2
fn get_neighbours(i: usize, j: usize, grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    // Above
    if i > 0
        && grid[i][j] != Tile::EastWest
        && grid[i][j] != Tile::SouthWest
        && grid[i][j] != Tile::SouthEast
    {
        match grid[i - 1][j] {
            Tile::NorthSouth => result.push((i - 1, j)),
            Tile::SouthWest => result.push((i - 1, j)),
            Tile::SouthEast => result.push((i - 1, j)),
            Tile::Start => result.push((i - 1, j)),
            _ => {}
        }
    }
    // Below
    if i < grid.len()
        && grid[i][j] != Tile::EastWest
        && grid[i][j] != Tile::NorthWest
        && grid[i][j] != Tile::NorthEast
    {
        match grid[i + 1][j] {
            Tile::NorthSouth => result.push((i + 1, j)),
            Tile::NorthEast => result.push((i + 1, j)),
            Tile::NorthWest => result.push((i + 1, j)),
            Tile::Start => result.push((i + 1, j)),
            _ => {}
        }
    }
    // Left
    if j > 0
        && grid[i][j] != Tile::NorthSouth
        && grid[i][j] != Tile::NorthEast
        && grid[i][j] != Tile::SouthEast
    {
        match grid[i][j - 1] {
            Tile::EastWest => result.push((i, j - 1)),
            Tile::NorthEast => result.push((i, j - 1)),
            Tile::SouthEast => result.push((i, j - 1)),
            Tile::Start => result.push((i, j - 1)),
            _ => {}
        }
    }
    // Right
    if j < grid[i].len()
        && grid[i][j] != Tile::NorthSouth
        && grid[i][j] != Tile::NorthWest
        && grid[i][j] != Tile::SouthWest
    {
        match grid[i][j + 1] {
            Tile::EastWest => result.push((i, j + 1)),
            Tile::NorthWest => result.push((i, j + 1)),
            Tile::SouthWest => result.push((i, j + 1)),
            Tile::Start => result.push((i, j + 1)),
            _ => {}
        }
    }
    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // Parse input into 2d array of Tiles
    let mut grid = inp
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).expect("Parsing failed"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // We'll traverse the graph in two directions simulataneously, this vector will contain the 2
    // elements that are currently being considered
    let mut current = Vec::new();

    // Find the Start tile and figure out which Tile the Start tile actually is
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Tile::Start {
                let neighbours = get_neighbours(i, j, &grid);

                // Figure out which tile Start is
                let mut solutions = Vec::new();
                'fuck: for potential_type in [
                    Tile::NorthSouth,
                    Tile::EastWest,
                    Tile::NorthEast,
                    Tile::NorthWest,
                    Tile::SouthWest,
                    Tile::SouthEast,
                ] {
                    let mut grid_clone = grid.clone();
                    grid_clone[i][j] = potential_type;
                    for neighbour in neighbours.iter() {
                        if !get_neighbours(neighbour.0, neighbour.1, &grid_clone).contains(&(i, j))
                        {
                            continue 'fuck;
                        }
                    }
                    solutions.push(potential_type);
                }
                assert!(solutions.len() == 1);
                grid[i][j] = solutions[0];
                current = neighbours;
                break 'outer;
            }
        }
    }

    // Traverse the graph in two directions until both end up on the same spot
    let mut visited = HashSet::new();
    while current[0] != current[1] {
        res.part_1 += 1;
        let c1 = current.swap_remove(0);
        visited.insert(c1);
        let c2 = current.swap_remove(0);
        visited.insert(c2);

        current.append(
            &mut get_neighbours(c1.0, c1.1, &grid)
                .into_iter()
                .filter(|xy| !visited.contains(xy))
                .collect(),
        );
        current.append(
            &mut get_neighbours(c2.0, c2.1, &grid)
                .into_iter()
                .filter(|xy| !visited.contains(xy))
                .collect(),
        );
    }
    visited.insert(current[0]);

    // Unfortunately had to get some help with this, had a working solution for everything except
    // for the "squeezing between pipes is also allowed!".
    // Basically traverse the 2d array line by line, for each line start as assuming you're outside
    // the loop, then check if the tile you're on would make you be inside the loop, if so flip the
    // state.
    let mut tiles_inside = Vec::new();
    for i in 0..grid.len() {
        let mut inside = false;
        for j in 0..grid[0].len() {
            if visited.contains(&(i, j))
                && (grid[i][j] == Tile::NorthEast
                    || grid[i][j] == Tile::NorthWest
                    || grid[i][j] == Tile::NorthSouth)
            {
                inside = !inside;
            } else if inside && !visited.contains(&(i, j)) {
                tiles_inside.push((i, j));
            }
        }
    }
    res.part_2 = tiles_inside.len();
}
