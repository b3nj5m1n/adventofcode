use std::env;
use std::io::Read;

use cached::proc_macro::cached;
use cached::SizedCache;

use roots::find_roots_quadratic;
use roots::Roots;

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
        part_1: 0.0,
        part_2: 0.0,
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: f64,
    part_2: f64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Race {
    time: usize,
    dist: usize,
}

fn input_part1(inp: &Vec<&str>) -> Vec<Race> {
    let times = inp[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let dists = inp[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let mut races = Vec::new();
    for (time, dist) in times.zip(dists) {
        races.push(Race { time, dist })
    }
    races
}
fn input_part2(inp: &Vec<&str>) -> Race {
    let time: usize = inp[0]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    let dist: usize = inp[1]
        .split_once(":")
        .unwrap()
        .1
        .trim()
        .split_whitespace()
        .collect::<String>()
        .parse()
        .unwrap();
    Race { time, dist }
}

fn wins(wait: usize, dist_to_beat: usize, time_total: usize) -> bool {
    let dist = wait * (time_total - wait);
    dist > dist_to_beat
}

// Caching this reduces run time by about 25%
/* #[cached(
    type = "SizedCache<(f64, f64), f64>",
    create = "{ SizedCache::with_size(500) }",
    convert = r#"{ (race.time, race.dist) }"#
)] */
fn ways_to_win(race: &Race) -> f64 {
    /* let mut start = 0;
    for ms in 1..race.time {
        if wins(ms, race.dist, race.time) {
            start = ms;
            break;
        }
    } */
    let (start_math, end_math) = if let Roots::Two([start, end]) =
        find_roots_quadratic(1f64, race.time as f64, race.dist as f64 + 1.0)
    {
        (f64::ceil(start), f64::floor(end))
    } else {
        panic!("Fuck");
    };
    // dbg!(start, start_math);
    /* let mut end = 0;
    for ms in (1..race.time).rev() {
        if wins(ms, race.dist, race.time) {
            end = ms;
            break;
        }
    } */
    // dbg!(end, end_math);
    // dbg!(end - start + 1, end_math - start_math + 1.0);
    // end - start + 1
    end_math - start_math + 1.0
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let races = input_part1(&inp);
    let race_part2 = input_part2(&inp);

    // Part 1
    res.part_1 = races
        .into_iter()
        .map(|race| ways_to_win(&race))
        .fold(1.0, |a, b| a * b);

    // Part 2
    res.part_2 = ways_to_win(&race_part2);
}
