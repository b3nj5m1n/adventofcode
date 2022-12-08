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
    part_2: i32,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let grid: Vec<Vec<u32>> = inp
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    let mut count_visible = grid.len() * 2 + (grid[0].len() * 2 - 4);
    for (i, row) in grid.iter().enumerate().skip(1).take(grid.len() - 2) {
        for (j, tree) in row.iter().enumerate().skip(1).take(row.len() - 2) {
            let visible_top = (0..i).rev().map(|ii| grid[ii][j]).max().unwrap() < *tree;
            let visible_bottom = (i + 1..grid.len()).map(|ii| grid[ii][j]).max().unwrap() < *tree;
            let visible_left = (0..j).rev().map(|jj| grid[i][jj]).max().unwrap() < *tree;
            let visible_right = (j + 1..row.len()).map(|jj| grid[i][jj]).max().unwrap() < *tree;
            let visible = visible_top || visible_bottom || visible_left || visible_right;
            if visible {
                count_visible += 1;
            }
        }
    }
    let mut best_scenic_score = 0;
    for (i, row) in grid.iter().enumerate() {
        for (j, tree) in row.iter().enumerate() {
            let mut viewing_distance_top = 0;
            for x in (0..i).rev().map(|ii| grid[ii][j]) {
                viewing_distance_top += 1;
                if x >= *tree {
                    break;
                }
            }
            let mut viewing_distance_bottom = 0;
            for x in (i + 1..grid.len()).map(|ii| grid[ii][j]) {
                viewing_distance_bottom += 1;
                if x >= *tree {
                    break;
                }
            }
            let mut viewing_distance_left = 0;
            for x in (0..j).rev().map(|jj| grid[i][jj]) {
                viewing_distance_left += 1;
                if x >= *tree {
                    break;
                }
            }
            let mut viewing_distance_right = 0;
            for x in (j + 1..row.len()).map(|jj| grid[i][jj]) {
                viewing_distance_right += 1;
                if x >= *tree {
                    break;
                }
            }
            let scenic_score = viewing_distance_top * viewing_distance_bottom * viewing_distance_left * viewing_distance_right;
            if scenic_score > best_scenic_score {
                best_scenic_score = scenic_score;
            }
        }
    }
    res.part_1 = count_visible;
    res.part_2 = best_scenic_score;
}
