// Fairly straight forward but I'm pretty sick so still a challenge. It's pretty obvious that you
// just have to use a linked list and only do one insertion and one deletion per number, but the
// docs on linked lists were weird and I wanted to get part 1, which is why This implementation
// sucks so much. I think you can probably get away with mod somehow but my naive approach didn't
// work. I'll get back to this later.
use std::collections::LinkedList;
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

fn calc_index(i: i64, len: usize) -> usize {
    /* let mut res = i % len as i64;
    if res < 0 {
        res += len as i64;
    }
    res as usize */
    if i < 0 {
        (len as i64 + (i % len as i64)) as usize
    } else {
        (i % len as i64) as usize
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let l_orig = inp
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i, x.parse::<i64>().unwrap() * 811589153))
        .collect::<Vec<(usize, i64)>>();
    let len = l_orig.len();
    let mut l = l_orig
        .clone();
    let l_copy = l.clone();
    for _ in 0..10 {
        dbg!(&l);
        for elem in l_copy.clone().into_iter() {
            let mut current_index = l.iter().position(|x| *x == elem).unwrap();
            if elem.1.signum() > 0 {
                for _ in 0..elem.1 {
                    /* println!(
                        "Swapping {} with {} ({}).",
                        calc_index(current_index as i64, len),
                        calc_index(current_index as i64 + 1, len),
                        current_index
                    ); */
                    if current_index == len - 1 {
                        // current_index = calc_index(current_index as i64 - 1, len);
                        current_index = 0;
                        let x = l.pop().unwrap();
                        l.insert(0, x);
                    }
                    l.swap(
                        calc_index(current_index as i64, len),
                        calc_index(current_index as i64 + 1, len),
                    );
                    // current_index += 1;
                    current_index = calc_index(current_index as i64 + 1, len)
                }
            } else if elem.1.signum() < 0 {
                for _ in (0..=elem.1.abs()).into_iter() {
                    /* println!(
                        "Swapping {} with {} ({}).",
                        calc_index(current_index as i64, len),
                        calc_index(current_index as i64 - 1, len),
                        current_index
                    ); */
                    if current_index == 0 {
                        // current_index = calc_index(current_index as i64 - 1, len);
                        current_index = len - 1;
                        let x = l.remove(0);
                        l.push(x);
                    }
                    l.swap(
                        calc_index(current_index as i64 - 1, len),
                        calc_index(current_index as i64, len),
                    );
                    /* if current_index == 0 {
                        current_index = len;
                    }
                    current_index = current_index - 1; */
                    current_index = calc_index(current_index as i64 - 1, len)
                }
            }
        }
    }
    dbg!(&l);
    let zero_index = l.iter().position(|x| x.1 == 0).unwrap();
    /* res.part_1 = (1..=3)
        .into_iter()
        .map(|x| l[calc_index(zero_index as i64 + x * 1000, len)].1)
        .sum::<i64>()
        .to_string(); */
    let x1 = l[calc_index(zero_index as i64 + 1000, len)];
    let r1 = l_orig[l_orig.iter().position(|&(i, _)| i == x1.0).unwrap()];
    let x2 = l[calc_index(zero_index as i64 + 2000, len)];
    let r2 = l_orig[l_orig.iter().position(|&(i, _)| i == x2.0).unwrap()];
    let x3 = l[calc_index(zero_index as i64 + 3000, len)];
    let r3 = l_orig[l_orig.iter().position(|&(i, _)| i == x3.0).unwrap()];
    dbg!(r1, r2, r3);
    dbg!(l_orig);
}
