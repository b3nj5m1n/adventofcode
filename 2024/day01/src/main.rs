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
    let lists: Vec<(usize, usize)> = match inp.into_iter().map(|line| {
        match line.split_whitespace().collect::<Vec<&str>>().as_slice() {
            &[l, r] => (l,r),
            _ => unreachable!(),
        }
    }).unzip::<&str, &str, Vec<&str>, Vec<&str>>() {
        (mut l, mut r) => {
            l.sort();
            r.sort();
            l.into_iter().zip(r.into_iter())
        },
        _ => unreachable!(),
    }.map(|(l,r)| {
            match (l.parse::<usize>(), r.parse::<usize>()) {
                (Ok(l), Ok(r)) => (l,r),
                _ => unreachable!()
            }
        }).collect();

    res.part_1 = lists.iter().map(|&(l,r)| l.abs_diff(r)).sum();

    let (l,r): (Vec<usize>, Vec<usize>) = lists.into_iter().unzip();
    res.part_2 = l.into_iter().map(|n| {
        n * r.iter().filter(|&m| *m == n).count()
    }).sum();

}
