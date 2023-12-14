use std::collections::HashMap;
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
    let north = transpose(inp.into_iter().map(|l| l.to_string()).collect(), 1);
    let north = tilt_north(north);
    let back = transpose(north.clone(), 3);
    // dbg!(&transpose(north.clone(), 3));
    res.part_1 = count(back);
    let west = transpose(north, 3);
    let west = tilt_north(west);
    // dbg!(&transpose(west.clone(), 0));
    let south = transpose(west.clone(), 3);
    let south = tilt_north(south);
    // dbg!(&transpose(south.clone(), 1));
    let east = transpose(south, 3);
    let east = tilt_north(east);
    // dbg!(&transpose(east.clone(), 2));
    let mut back = transpose(east.clone(), 2);

    // Find a cycle in the NWSE shifts, once a cycle has been found we can skip most of the
    // iterations and jump straight to the last few
    let mut map = HashMap::new();
    let bound = 1000000000;
    map.insert(back.clone(), bound);
    let mut i = bound - 1;
    while i > 0 {
        back = cycle(back);
        if map.contains_key(&back.clone()) {
            let c_len = map.get(&back.clone()).unwrap() - i;
            i = (i - 1) % (c_len);
            break;
        }
        map.insert(back.clone(), i);
        i -= 1;
    }
    for _ in 0..i {
        back = cycle(back);
    }

    res.part_2 = count(back);
}

fn cycle(back: Vec<String>) -> Vec<String> {
    let north = transpose(back, 1);
    let north = tilt_north(north);
    let west = transpose(north, 3);
    let west = tilt_north(west);
    let south = transpose(west.clone(), 3);
    let south = tilt_north(south);
    let east = transpose(south, 3);
    let east = tilt_north(east);
    transpose(east.clone(), 2)
}

fn count(inp: Vec<String>) -> usize {
    inp.iter()
        .rev()
        .enumerate()
        .map(|(i, l)| l.to_string().chars().filter(|c| c == &'O').count() * (i + 1))
        .sum()
}

fn transpose(inp: Vec<String>, count: u8) -> Vec<String> {
    let mut transposed: Vec<String> = inp.clone().into_iter().map(|l| l.to_string()).collect();
    for _ in 0..count {
        transposed = (0..transposed.clone()[0].to_string().len())
            .rev()
            .map(|col| {
                (0..transposed.len())
                    .map(|row| {
                        transposed[row]
                            .to_string()
                            .chars()
                            .nth(col)
                            .expect("fuck")
                            .clone()
                    })
                    .collect::<String>()
            })
            .collect();
    }
    transposed
    // transposed.into_iter().map(|l| l.to_string()).collect::<Vec<String>>()
}

fn tilt_north(inp: Vec<String>) -> Vec<String> {
    let mut grid = Vec::new();
    for line in inp {
        let mut new_line = String::new();
        let mut skip = false;
        for (i, c) in line.to_string().chars().enumerate() {
            if c == '#' {
                new_line.push_str("#");
                skip = false;
            } else if skip {
                continue;
            } else {
                let s = &line
                    .to_string()
                    .chars()
                    .skip(i)
                    .filter(|c| c != &'.')
                    .take_while(|c| c != &'#')
                    .collect::<String>();
                let dots = &line
                    .to_string()
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
    grid
}
