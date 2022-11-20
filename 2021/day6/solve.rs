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
    let mut fish: Vec<Fish> = inp[0]
        .split(",")
        .map(|t| Fish {
            timer: t.parse::<u8>().unwrap(),
        })
        .collect();
    let mut fish_pool = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
    for f in fish.clone() {
        fish_pool[usize::from(f.timer)] += 1
    }
    let iterations = 80;
    for i in 0..iterations {
        fish = day(fish);
    }
    res.part_1 = fish.len();
    let iterations_part_2 = 256;
    /* for i in 0..(iterations_part_2 - iterations) {
        println!("{}", i);
        fish = day(fish);
    }
    res.part_2 = fish.len(); */
    for i in 0..iterations_part_2 {
        day_vec(&mut fish_pool);
    }
    /* println!("{:?}", fish_pool);
    day_vec(&mut fish_pool); */
    res.part_2 = fish_pool.into_iter().fold(0, |x, y| x + y);
}

fn day_vec(fish: &mut Vec<usize>) {
    let new_fish = fish[0];
    for i in 0..8 {
        fish[i] = fish[i+1];
    }
    fish[6] += new_fish;
    fish[8] = new_fish;
}

fn day(fish: Vec<Fish>) -> Vec<Fish> {
    let mut result = Vec::new();
    for f in fish {
        if f.timer == 0 {
            result.push(Fish { timer: 6 });
            result.push(Fish { timer: 8 });
        } else {
            result.push(Fish { timer: f.timer - 1 });
        }
    }
    result
}

#[derive(Debug, Clone, Copy)]
struct Fish {
    timer: u8,
}
