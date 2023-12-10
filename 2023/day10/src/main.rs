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
    part_2: u32,
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

fn get_neighbours(i: usize, j: usize, grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    /* println!(
        "Getting neighbors for {i},{j}, which is of type: {:?}",
        grid[i][j]
    ); */
    /* println!(
        "{}{}{}",
        grid[i - 1][j - 1].to_string(),
        grid[i - 1][j].to_string(),
        grid[i - 1][j + 1].to_string()
    );
    println!(
        "{}{}{}",
        grid[i][j - 1].to_string(),
        grid[i][j].to_string(),
        grid[i][j + 1].to_string()
    );
    println!(
        "{}{}{}",
        grid[i + 1][j - 1].to_string(),
        grid[i + 1][j].to_string(),
        grid[i + 1][j + 1].to_string()
    ); */
    let mut result = Vec::new();
    // Above
    if i > 0
        && grid[i][j] != Tile::EastWest
        && grid[i][j] != Tile::SouthWest
        && grid[i][j] != Tile::SouthEast
    {
        match grid[i - 1][j] {
            Tile::NorthSouth => result.push((i - 1, j)),
            Tile::EastWest => {}
            Tile::NorthEast => {}
            Tile::NorthWest => result.push((i - 1, j)),
            Tile::SouthWest => result.push((i - 1, j)),
            Tile::SouthEast => result.push((i - 1, j)),
            Tile::Ground => {}
            Tile::Start => result.push((i - 1, j)),
        }
    }
    // Below
    if i < grid.len() - 1
        && grid[i][j] != Tile::EastWest
        && grid[i][j] != Tile::NorthWest
        && grid[i][j] != Tile::NorthEast
    {
        match grid[i + 1][j] {
            Tile::NorthSouth => result.push((i + 1, j)),
            Tile::EastWest => {}
            Tile::NorthEast => result.push((i + 1, j)),
            Tile::NorthWest => result.push((i + 1, j)),
            Tile::SouthWest => {}
            Tile::SouthEast => {}
            Tile::Ground => {}
            Tile::Start => result.push((i + 1, j)),
        }
    }
    // Left
    if j > 0
        && grid[i][j] != Tile::NorthSouth
        && grid[i][j] != Tile::NorthEast
        && grid[i][j] != Tile::SouthEast
    {
        match grid[i][j - 1] {
            Tile::NorthSouth => {}
            Tile::EastWest => result.push((i, j - 1)),
            Tile::NorthEast => result.push((i, j - 1)),
            Tile::NorthWest => {}
            Tile::SouthWest => {}
            Tile::SouthEast => result.push((i, j - 1)),
            Tile::Ground => {}
            Tile::Start => result.push((i, j - 1)),
        }
    }
    // Right
    if j < grid[i].len() - 1
        && grid[i][j] != Tile::NorthSouth
        && grid[i][j] != Tile::NorthWest
        && grid[i][j] != Tile::SouthWest
    {
        match grid[i][j + 1] {
            Tile::NorthSouth => {}
            Tile::EastWest => result.push((i, j + 1)),
            Tile::NorthEast => {}
            Tile::NorthWest => result.push((i, j + 1)),
            Tile::SouthWest => result.push((i, j + 1)),
            Tile::SouthEast => {}
            Tile::Ground => {}
            Tile::Start => result.push((i, j + 1)),
        }
    }
    // dbg!(&result);
    assert!(result.len() == 2);
    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let grid = inp
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).expect("Parsing failed"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut current = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Tile::Start {
                // println!("Start found at {i}, {j}");
                let neighbours = get_neighbours(i, j, &grid);
                current = neighbours;
            }
        }
    }
    let mut visited = HashSet::new();
    while current[0] != current[1] {
        /* println!("--- New Iteration ---");
        dbg!(&current); */
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
        // dbg!(&current);
    }
    // res.part_1 += 1;

    // dbg!(&visited);
}
