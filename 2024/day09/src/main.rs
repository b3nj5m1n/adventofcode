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
    let mut map: Vec<i32> = Vec::new();
    let mut id = 0;
    for s in inp[0].chars().collect::<Vec<_>>().as_slice().chunks(2) {
        if let &[files, space] = s {
            let count_files = files
                .to_string()
                .parse::<u8>()
                .expect("Couldn't parse number");
            let count_space = space
                .to_string()
                .parse::<u8>()
                .expect("Couldn't parse number");
            for _ in 0..count_files {
                map.push(id);
            }
            for _ in 0..count_space {
                map.push(-1);
            }
        } else if let &[files] = s {
            let count_files = files
                .to_string()
                .parse::<u8>()
                .expect("Couldn't parse number");
            for _ in 0..count_files {
                map.push(id);
            }
        }
        id += 1;
    }
    let mut i = map.len() - 1;
    while i > 0 {
        /* println!(
            "{}",
            map.iter()
                .map(|n| if *n == -1 {
                    ".".to_string()
                } else {
                    n.to_string()
                })
                .collect::<Vec<_>>()
                .concat()
        ); */
        let mut done = true;
        for k in 0..i {
            if map[k] == -1 {
                done = false;
            }
        }
        if map[i] == -1 || done {
            i = i - 1;
            continue;
        }
        let mut j = 0;
        while map[j] != -1 {
            j += 1;
        }
        let temp = map[i];
        map[i] = map[j];
        map[j] = temp;
        i = i - 1;
    }
    /* println!(
        "{}",
        map.iter()
            .map(|n| if *n == -1 {
                ".".to_string()
            } else {
                n.to_string()
            })
            .collect::<Vec<_>>()
            .concat()
    ); */
    res.part_1 = map
        .iter()
        .enumerate()
        .map(|(i, n)| if *n == -1 { 0 } else { i * (*n as usize) })
        .sum();
}
