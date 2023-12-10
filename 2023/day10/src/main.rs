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

fn get_neighbours_raw(i: usize, j: usize, grid: &Vec<Vec<Tile>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    // Above
    if i > 0 {
        result.push((i - 1, j))
    }
    // Below
    if i < grid.len() {
        result.push((i + 1, j))
    }
    // Left
    if j > 0 {
        result.push((i, j - 1))
    }
    // Right
    if j < grid[i].len() {
        result.push((i, j + 1))
    }
    // Above & Left
    if i > 0 && j > 0 {
        result.push((i - 1, j - 1))
    }
    // Above & Right
    if i > 0 && j < grid[i].len() {
        result.push((i - 1, j + 1))
    }
    // Below & Left
    if i < grid.len() && j > 0 {
        result.push((i + 1, j - 1))
    }
    // Below & Right
    if i < grid.len() && j < grid[i].len() {
        result.push((i + 1, j + 1))
    }

    result
}

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
            Tile::EastWest => {}
            Tile::NorthEast => {}
            Tile::NorthWest => {}
            Tile::SouthWest => result.push((i - 1, j)),
            Tile::SouthEast => result.push((i - 1, j)),
            Tile::Ground => {}
            Tile::Start => result.push((i - 1, j)),
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
    if j < grid[i].len()
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
    // assert!(result.len() == 2);
    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid = inp
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| Tile::from_str(&c.to_string()).expect("Parsing failed"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut current = Vec::new();
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Tile::Start {
                // println!("Start found at {i}, {j}");
                let neighbours = get_neighbours(i, j, &grid);
                /* let start_type = match (neighbours[0], neighbours[1]) {

                }; */

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
                // dbg!(&solutions);
                assert!(solutions.len() == 1);
                grid[i][j] = solutions[0];
                current = neighbours;
                break 'outer;
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
    visited.insert(current[0]);

    /* let mut not_loop = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited.contains(&(i, j)) {
                not_loop.insert((i, j));
            }
        }
    } */

    /* // Find all ground tiles along the edges (not inside the loop)
    let mut starts = HashSet::new();
    for j in 0..grid[0].len() {
        if !visited.contains(&(0, j)) {
            starts.insert((0, j));
        }
        let last_index = grid.len() - 1;
        if !visited.contains(&(last_index, j)) {
            starts.insert((last_index, j));
        }
    }
    for i in 0..grid.len() {
        if !visited.contains(&(i, 0)) {
            starts.insert((i, 0));
        }
        let last_index = grid[i].len() - 1;
        if !visited.contains(&(i, last_index)) {
            starts.insert((i, last_index));
        }
    }
    // dbg!(&starts); */

    /* // Flood fill?
    let mut outside = HashSet::new();
    let mut to_search: Vec<_> = starts.into_iter().collect();
    while to_search.len() > 0 {
        let current = to_search.pop().expect("Fuck");
        outside.insert(current);
        for neighbour in get_neighbours_raw(current.0, current.1, &grid) {
            if !visited.contains(&neighbour) && !outside.contains(&neighbour) {
                to_search.push(neighbour);
            }
        }
    } */
    // dbg!(&outside);
    // res.part_2 = not_loop.difference(&outside).count();

    /* let mut result = HashSet::new();
    // Try to get from tile to outside without hitting main loop
    'help: for tile in not_loop.difference(&outside) {
        println!("Considering {},{}", tile.0, tile.1);
        let mut pipe_count_right = 0;
        for i in 0..tile.1 {
            if (grid[tile.0][i] == Tile::NorthSouth
                || grid[tile.0][i] == Tile::NorthEast
                || grid[tile.0][i] == Tile::NorthWest
                || grid[tile.0][i] == Tile::SouthEast
                || grid[tile.0][i] == Tile::SouthWest)
                && visited.contains(&(tile.0, i))
            {
                pipe_count_right += 1;
            }
            /* if grid[tile.0][i] != Tile::NorthSouth
                && grid[tile.0][i] != Tile::Ground
            {
                pipe_count_right = 1;
                break;
            } */
        }
        /* let mut pipe_count_left = 0;
        for i in 0..tile.1 {
            if grid[tile.0][i] == Tile::NorthSouth && visited.contains(&(tile.0, i)) {
                pipe_count_left += 1;
            }
            if grid[tile.0][i] != Tile::NorthSouth
                && grid[tile.0][i] != Tile::Ground
                && grid[tile.0][i] != Tile::Start
            {
                pipe_count_left = 1;
                break;
            }
        }
        let mut pipe_count_top = 0;
        for i in 0..tile.0 {
            if grid[i][tile.1] == Tile::EastWest && visited.contains(&(i, tile.1)) {
                pipe_count_top += 1;
            }
            if grid[i][tile.1] != Tile::EastWest && grid[i][tile.1] != Tile::Ground {
                pipe_count_top = 1;
                break;
            }
        }
        let mut pipe_count_bottom = 0;
        for i in tile.0..grid.len() {
            if grid[i][tile.1] == Tile::EastWest && visited.contains(&(i, tile.1)) {
                pipe_count_bottom += 1;
            }
            if grid[i][tile.1] != Tile::EastWest && grid[i][tile.1] != Tile::Ground {
                pipe_count_bottom = 1;
                break;
            }
        } */
        /* dbg!(pipe_count_left, pipe_count_right);
        if pipe_count_right % 2 == 0
            || pipe_count_left % 2 == 0
            || pipe_count_top % 2 == 0
            || pipe_count_bottom % 2 == 0 */
        println!("Found {pipe_count_right} vertical pipes");
        if pipe_count_right % 2 == 0 && pipe_count_right != 0 {
            continue 'help;
        } else {
            result.insert(tile);
            continue 'help;
        }
        unreachable!();
        let mut to_consider = vec![*tile];
        let mut considered = HashSet::new();
        'outer: while to_consider.len() > 0 {
            // dbg!(&to_consider);
            let current = to_consider.pop().expect("fuck");
            considered.insert(current);
            for n in get_neighbours_raw(current.0, current.1, &grid) {
                if considered.contains(&(n.0, n.1)) {
                    println!("Skipping because already considered");
                    continue;
                }
                if visited.contains(&(n.0, n.1)) {
                    println!("Skipping because loop border reached");
                    continue;
                }
                /* if result.contains(&(n.0, n.1)) {
                    continue;
                } */
                /* if n.0 == 0
                || n.0 == grid.len() - 1
                || n.1 == 0
                || n.1 == grid[0].len() - 1 */
                if outside.contains(&(n.0, n.1)) {
                    break 'outer;
                }
                to_consider.push((n.0, n.1));
            }
        }
        result.insert(tile);
        // println!("{}, {}", tile.0, tile.1);
    }
    // dbg!(&result); */

    // If we had an adjacency matrix of the graph, we could look for gaps in the loop to account
    // for the "squeezing between pipes is also allowed" bs

    // res.part_2 = result.len();

    // dbg!(outside.len());

    /* let mut visited: Vec<_> = visited.into_iter().collect();
    visited.sort(); */
    // dbg!(visited);

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
    // dbg!(&tiles_inside);
    res.part_2 = tiles_inside.len();
}
