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
    part_2: i64,
}

// Convert vector containing single digits of a binary number to decimal
fn binary_to_decimal(vector: &Vec<u8>) -> usize {
    let mut acc = 0;
    let mut pow = 1;
    for x in vector.iter().rev() {
        acc += pow * usize::from(*x);
        pow *= 2;
    }
    acc
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let width = inp[0].len();
    let height = inp.len();
    let flattened_array: Vec<usize> = inp
        .iter()
        .flat_map(|num| num.chars())
        .filter_map(|c| match c {
            '0' => Some(0),
            '1' => Some(1),
            _ => None,
        })
        .collect();
    let mut rate_gamma: Vec<u8> = Vec::new();
    for i in 0..width {
        let row_ones: usize = usize::from(
            flattened_array
                .iter()
                .enumerate()
                .filter(|&(j, _)| j % width == i)
                .map(|(_, e)| e)
                .sum::<usize>(),
        );
        let row_zeroes = height - usize::from(row_ones);
        if row_ones > row_zeroes {
            rate_gamma.push(1);
        } else {
            rate_gamma.push(0);
        }
    }
    let rate_epsilon: Vec<u8> = rate_gamma
        .iter()
        .map(|bit| if bit == &0 { 1 } else { 0 })
        .collect();
    res.part_1 = binary_to_decimal(&rate_gamma) * binary_to_decimal(&rate_epsilon);
    let nums: Vec<i64> = inp
        .iter()
        .map(|num| i64::from_str_radix(num, 2).unwrap())
        .collect();
    // Disclaimer: this is an absolute mess
    // Ok so here's another way for part 1
    let mut answer_1 = 0b0;
    for i in 0..width {
        answer_1 = answer_1 << 1;
        match get_most_common(&nums, (width - (i + 1))) {
            false => answer_1 = answer_1 + 0,
            true => answer_1 = answer_1 + 1,
        }
    }
    // This basically just gets the inverse of answer_1, but because by default that would involve
    // a bunch of leading 0's turning into 1's, we have to do some masking here
    let black_magic = !answer_1 & (2_i32.pow(5) - 1);
    let answer_1 = answer_1 * black_magic;
    // And after only one year, part 2
    let mut most_common_oxy = keep_most_common(nums.clone(), width - 1, false);
    let mut most_common_co2 = keep_most_common(nums.clone(), width - 1, true);
    let mut i = 1;
    loop {
        i = i + 1;
        if most_common_oxy.len() != 1 {
            most_common_oxy = keep_most_common(most_common_oxy, width - i, false);
        }
        if most_common_co2.len() != 1 {
            most_common_co2 = keep_most_common(most_common_co2, width - i, true);
        }
        if most_common_co2.len() == 1 && most_common_oxy.len() == 1 { break; }
    }
    println!("{:#05b}", most_common_oxy[0]);
    println!("{:#05b}", most_common_co2[0]);
    res.part_2 = most_common_oxy[0] * most_common_co2[0];
}

fn keep_most_common(numbers: Vec<i64>, position: usize, invert: bool) -> Vec<i64> {
    let most_common = get_most_common(&numbers, position) ^ invert;
    let mask = 0b1 << position;
    let mut result = Vec::new();
    for number in numbers {
        if number & mask == 0 && most_common == false {
            result.push(number);
        } else if number & mask != 0 && most_common == true {
            result.push(number);
        }
    }
    result
}

fn get_most_common(numbers: &Vec<i64>, position: usize) -> bool {
    let mask = 0b1 << position;
    let mut count_zero = 0;
    let mut count_one = 0;
    for number in numbers {
        let masked = number & mask;
        // println!("{:#07b}, {:#07b}", number, masked);
        if masked == 0 {
            count_zero += 1;
        } else {
            count_one += 1;
        }
    }
    // println!("{} zeroes, {} ones.", count_zero, count_one);
    count_zero <= count_one
}
