// Got hung up trying to do the decimal->snafu conversion from most sifnificant digit to least
// signficiant, looked up a solution in the end. I'll hopefully get to doing the part 2's I'm
// missing once I'm feeling better.
use itertools::Itertools;
use kdam::tqdm;
use rayon::prelude::*;
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
        part_1: String::from(""),
        part_2: String::from(""),
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: String,
}

fn decimal_to_snafu(decimal: impl ToString) -> String {
    let mut result = Vec::new();
    let mut current = decimal.to_string().parse::<i64>().unwrap();
    while current > 0 {
        let value = current % 5;
        result.push(match value {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => panic!("fuck"),
        });
        let carry = match value {
            3 | 4 => 1,
            _ => 0,
        };
        current = current / 5 + carry;
    }
    result.into_iter().rev().collect::<String>()
}

fn snafu_to_decimal(snafu: impl ToString) -> String {
    let mut result = 0;
    for (i, c) in snafu.to_string().chars().rev().enumerate() {
        let worth = 5_u64.pow(i as u32) as i64;
        let x = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '-' => -1,
            '=' => -2,
            _ => panic!("Unknown numeral"),
        };
        result += worth * x;
    }
    result.to_string()
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut result = 0;
    for line in inp {
        result += snafu_to_decimal(line).parse::<i64>().unwrap();
    }
    res.part_1 = decimal_to_snafu(result);
}

#[test]
fn test_snafu_to_decimal() {
    assert_eq!(snafu_to_decimal("1=-0-2"), "1747");
    assert_eq!(snafu_to_decimal("12111"), "906");
    assert_eq!(snafu_to_decimal("2=0="), "198");
    assert_eq!(snafu_to_decimal("21"), "11");
    assert_eq!(snafu_to_decimal("2=01"), "201");
    assert_eq!(snafu_to_decimal("111"), "31");
    assert_eq!(snafu_to_decimal("20012"), "1257");
    assert_eq!(snafu_to_decimal("112"), "32");
    assert_eq!(snafu_to_decimal("1=-1="), "353");
    assert_eq!(snafu_to_decimal("1-12"), "107");
    assert_eq!(snafu_to_decimal("12"), "7");
    assert_eq!(snafu_to_decimal("1="), "3");
    assert_eq!(snafu_to_decimal("122"), "37");
}
