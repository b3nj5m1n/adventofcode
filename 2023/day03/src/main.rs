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
    part_1: u32,
    part_2: u32,
}

fn is_symbol(c: &char) -> bool {
    !c.is_digit(10) && *c != '.'
}

fn get_neighbors(x: usize, y: usize, max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    // LEFT
    if x > 0 {
        result.push((x - 1, y));
    }
    // ABOVE
    if y > 0 {
        result.push((x, y - 1));
    }
    // RIGHT
    if x < max_x - 1 {
        result.push((x + 1, y));
    }
    // BELOW
    if y < max_y - 1 {
        result.push((x, y + 1));
    }
    // BELOW and to the RIGHT
    if x < max_x - 1 && y < max_y - 1 {
        result.push((x + 1, y + 1));
    }
    // BELOW and to the LEFT
    if x < max_x - 1 && y > 0 {
        result.push((x + 1, y - 1));
    }
    // ABOVE and to the LEFT
    if x > 0 && y > 0 {
        result.push((x - 1, y - 1));
    }
    // ABOVE and to the RIGHT
    if x > 0 && y < max_y - 1 {
        result.push((x - 1, y + 1));
    }
    result
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let inp: Vec<Vec<_>> = inp.iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
    // dbg!(inp);
    let mut valid_nums = Vec::new();
    let mut invalid_nums = Vec::new();
    for (i_y, y) in inp.iter().enumerate() {
        let mut on_num = false;
        let mut num = Vec::new();
        let mut num_valid = false;
        for (i_x, x) in y.iter().enumerate() {
            if x.is_digit(10) {
                on_num = true;
                num.push(*x);
                for neighbor in get_neighbors(i_x, i_y, inp.len(), y.len()) {
                    if is_symbol(&inp[neighbor.1][neighbor.0]) {
                        num_valid = true;
                    }
                }
            }
            if !x.is_digit(10) || i_x == y.len() - 1 {
                if num_valid && on_num {
                    valid_nums.push(
                        num.into_iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .expect("Couldn't parse number"),
                    );
                } else if on_num {
                    invalid_nums.push(
                        num.into_iter()
                            .collect::<String>()
                            .parse::<u32>()
                            .expect("Couldn't parse number"),
                    );
                }
                on_num = false;
                num = Vec::new();
                num_valid = false;
            }
        }
    }
    /* dbg!(&valid_nums);
    dbg!(&invalid_nums); */
    res.part_1 = valid_nums.iter().sum();
}
