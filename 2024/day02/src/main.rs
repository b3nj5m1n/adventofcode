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

fn is_save_with_removal(report: Vec<u64>, allowed_removals: usize, index_to_remove: usize) -> bool {
    if allowed_removals > 0 {
        let mut new_report = report.clone();
        new_report.remove(index_to_remove);
        if is_save(new_report, allowed_removals - 1) {
            return true;
        }
    }
    false
}

fn is_save(report: Vec<u64>, allowed_removals: usize) -> bool {
    let mut ge = true;
    let mut le = true;

    for i in 1..report.len() {
        if ge && !report[i - 1].ge(&report[i]) {
            ge = false;
            if is_save_with_removal(report.clone(), allowed_removals, i)
                || is_save_with_removal(report.clone(), allowed_removals, i - 1)
            {
                return true;
            }
        }
        if le && !report[i - 1].le(&report[i]) {
            le = false;
            if is_save_with_removal(report.clone(), allowed_removals, i)
                || is_save_with_removal(report.clone(), allowed_removals, i - 1)
            {
                return true;
            }
        }
    }

    if !ge && !le {
        return false;
    }

    for i in 1..report.len() {
        let diff = report[i - 1].abs_diff(report[i]);
        if !(diff >= 1 && diff <= 3) {
            if is_save_with_removal(report.clone(), allowed_removals, i)
                || is_save_with_removal(report.clone(), allowed_removals, i - 1)
            {
                return true;
            } else {
                return false;
            }
        }
    }

    true
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let inp: Vec<Vec<u64>> = inp
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse().expect("Invalid number"))
                .collect()
        })
        .collect();
    /* for line in inp {
        println!("{:?}: {}", &line, is_save(line.clone(), 1))
    } */
    res.part_1 = inp
        .iter()
        .map(|l| if is_save(l.clone(), 0) { 1 } else { 0 })
        .sum();
    res.part_2 = inp
        .iter()
        .map(|l| if is_save(l.clone(), 1) { 1 } else { 0 })
        .sum();
}
