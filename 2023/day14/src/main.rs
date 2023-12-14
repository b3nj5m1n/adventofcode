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

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    /* for (i, line) in inp.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'O' {
                for (ii, line) in grid.iter_mut().enumerate().rev() {
                    if ii < i {
                        if grid.clone()[ii-1].chars.nth(j) == ''
                        grid[i].chars().nth(j);
                    }
                }
            }
        }
        // println!("{}", line)
    } */
    let transposed: Vec<String> = (0..inp[0].len())
        .rev()
        .map(|col| {
            (0..inp.len())
                .map(|row| inp[row].chars().nth(col).expect("fuck").clone())
                .collect()
        })
        .collect();
    let mut grid = Vec::new();
    dbg!(&transposed);
    for line in transposed {
        let mut new_line = String::new();
        let mut skip = false;
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                new_line.push_str("#");
                skip = false;
            } else if skip {
                continue;
            } else {
                let s = &line
                    .chars()
                    .skip(i)
                    .filter(|c| c != &'.')
                    .take_while(|c| c != &'#')
                    .collect::<String>();
                let dots = &line
                    .chars()
                    .skip(i)
                    .take_while(|c| c != &'#')
                    .filter(|c| c == &'.')
                    .collect::<String>();
                new_line.push_str(s);
                new_line.push_str(dots);
                skip = true;
            }
        }
        grid.push(new_line);
    }
    let transposed_back: Vec<String> = (0..grid[0].len())
        .map(|col| {
            (0..grid.len())
                .rev()
                .map(|row| grid[row].chars().nth(col).expect("fuck").clone())
                .collect()
        })
        .collect();
    let r: usize = transposed_back
        .iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.chars().filter(|c| c == &'O').count() * ( i+1 ))
        .sum();
    dbg!(&transposed_back);
    dbg!(r);
}
