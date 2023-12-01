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

/* fn contains_nums_as_text(inp: &str) -> bool {
    inp.contains("one")
        || inp.contains("two")
        || inp.contains("three")
        || inp.contains("four")
        || inp.contains("five")
        || inp.contains("six")
        || inp.contains("seven")
        || inp.contains("eight")
        || inp.contains("nine")
}
fn replace_text_with_nums(inp: &str) -> String {
    inp.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
} */

fn convert_text_to_nums(inp: &str) -> String {
    /* let mut result = String::new();
    let mut current = inp.to_owned();
    'outer: while !current.is_empty() {
        // println!("Current: {current}\nCurent result: {result}");
        for i in 1..=current.clone().chars().count() {
            let sub = current.clone().chars().take(i).collect::<String>();
            // println!("Checking substring: {sub}");
            if contains_nums_as_text(&sub) {
                result.push_str(&replace_text_with_nums(&sub));
                current = current.chars().skip(i).collect::<String>();
                // break
                continue 'outer;
            }
        }
        break;
    }
    result.push_str(&current);
    result */
    let mut result = String::new();
    for i in 0..=inp.chars().count() {
        let current = inp.chars().skip(i).collect::<String>();
        // println!("Substring: {current}");
        if current.starts_with(|c: char| c.is_digit(10)) {
            result.push(current.chars().into_iter().nth(0).expect("Unreachable"))
        }
        if current.starts_with("one") {
            result.push('1');
        }
        if current.starts_with("two") {
            result.push('2');
        }
        if current.starts_with("three") {
            result.push('3');
        }
        if current.starts_with("four") {
            result.push('4');
        }
        if current.starts_with("five") {
            result.push('5');
        }
        if current.starts_with("six") {
            result.push('6');
        }
        if current.starts_with("seven") {
            result.push('7');
        }
        if current.starts_with("eight") {
            result.push('8');
        }
        if current.starts_with("nine") {
            result.push('9');
        }
    }
    // println!("{result}");
    result
}

fn line_to_result(inp: &str) -> u32 {
    let numbers: Vec<_> = inp.chars().filter_map(|c| c.to_digit(10)).collect();
    10 * numbers.first().expect("No numbers found on line")
        + numbers.last().expect("No numbers found on line")
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    for line in inp {
        res.part_1 += line_to_result(line);
        // println!("{}", convert_text_to_nums(line));
        res.part_2 += line_to_result(&convert_text_to_nums(line));
    }
}
