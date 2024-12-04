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

fn coords_to_check(
    (x, y): (usize, usize),
    height: usize,
    width: usize,
) -> Vec<(
    (usize, usize),
    (usize, usize),
    (usize, usize),
    (usize, usize),
)> {
    let mut result = Vec::new();
    // Horizontal
    if x + 3 < width {
        result.push(((x, y), (x + 1, y), (x + 2, y), (x + 3, y)));
        result.push(((x + 3, y), (x + 2, y), (x + 1, y), (x, y)));
    }
    // Vertical
    if y + 3 < height {
        result.push(((x, y), (x, y + 1), (x, y + 2), (x, y + 3)));
        result.push(((x, y + 3), (x, y + 2), (x, y + 1), (x, y)));
    }
    // Diagonal 1
    if x + 3 < width && y + 3 < height {
        result.push(((x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)));
        result.push(((x + 3, y + 3), (x + 2, y + 2), (x + 1, y + 1), (x, y)));
    }
    // Diagonal 2
    if x + 3 < width && y >= 3 {
        result.push(((x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)));
        result.push(((x + 3, y - 3), (x + 2, y - 2), (x + 1, y - 1), (x, y)));
    }

    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let grid = inp
        .into_iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let width = grid[0].len();
    let height = grid.len();
    let mut sparse_grid = vec![vec!["."; width]; height];
    for x in 0..width {
        for y in 0..height {
            for (a, b, c, d) in coords_to_check((x, y), height, width) {
                if grid[a.1][a.0] == 'X'
                    && grid[b.1][b.0] == 'M'
                    && grid[c.1][c.0] == 'A'
                    && grid[d.1][d.0] == 'S'
                {
                    res.part_1 += 1;
                    sparse_grid[a.1][a.0] = "X";
                    sparse_grid[b.1][b.0] = "M";
                    sparse_grid[c.1][c.0] = "A";
                    sparse_grid[d.1][d.0] = "S";
                }
            }
        }
    }
    dbg!(grid, width, height);
    println!(
        "{}",
        sparse_grid
            .into_iter()
            .map(|l| l.concat())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
