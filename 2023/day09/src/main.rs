use std::borrow::BorrowMut;
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
    part_1: i64,
    part_2: i64,
}

fn dx(values: &Vec<i64>) -> Vec<i64> {
    values
        .windows(2)
        .map(|w| match w {
            &[a, b] => b - a,
            _ => unreachable!(),
        })
        .collect()
}
fn dx2(values: &Vec<i64>) -> Vec<i64> {
    values
        .windows(2)
        .map(|w| match w {
            &[a, b] => a-b,
            _ => unreachable!(),
        })
        .collect()
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let values = inp
        .clone()
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().expect("Couldn't parse number"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for hist in values {
        let mut dxs = Vec::new();
        dxs.push(hist);
        while dxs
            .last()
            .expect("shit")
            .iter()
            .filter(|&&e| e != 0)
            .count()
            > 0
        {
            let hist_ = dx(&dxs.last().expect("shit"));
            dxs.push(hist_);
        }

        dxs.last_mut().expect("shit").push(0);
        // for (i,dx) in dxs.iter().rev().skip(1).enumerate() {
        for i in (0..dxs.len() - 1).rev() {
            let last_element = dxs[i].last().expect("fuck").clone();
            let slope = dxs[i + 1].last().expect("fuck").clone();
            dxs[i].push(last_element + slope);
        }
        let next = dxs[0].last().expect("fuck");
        res.part_1 += next;
    }

    let values = inp
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().expect("Couldn't parse number"))
                .rev()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for hist in values {
        let mut dxs = Vec::new();
        dxs.push(hist);
        while dxs
            .last()
            .expect("shit")
            .iter()
            .filter(|&&e| e != 0)
            .count()
            > 0
        {
            let hist_ = dx2(&dxs.last().expect("shit"));
            dxs.push(hist_);
        }

        dxs.last_mut().expect("shit").push(0);
        // for (i,dx) in dxs.iter().rev().skip(1).enumerate() {
        for i in (0..dxs.len() - 1).rev() {
            let last_element = dxs[i].last().expect("fuck").clone();
            let slope = dxs[i + 1].last().expect("fuck").clone();
            dxs[i].push(last_element - slope);
        }
        let next = dxs[0].last().expect("fuck");
        res.part_2 += next;
    }
}
