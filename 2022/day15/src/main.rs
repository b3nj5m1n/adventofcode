// So far, I struggled the most with this day, but I managed to solve it without any help by myself,
// which I'm quite proud of, even though it took me 4 hours to do it.
// I initally though implementing part 2 properly would be way harder than it actually turned out
// to be, which is why my original strategy was to optimise the wrong solution (brute-force) as
// much as possible, then let it run while I take a nap. I _think_ I probably got the runtime down
// to about 4 hours or so on my machine, but I didn't end up running it to completion, so I don't
// know for sure. I'll include the source code for that in _threads.rs_ for my amusement.

use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::io::Read;

use anyhow::bail;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

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

#[derive(Debug, Clone)]
struct Sensor {
    x: i64,
    y: i64,
}
#[derive(Debug, Clone)]
struct Beacon {
    x: i64,
    y: i64,
}

fn parse_x_y(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), nom::character::complete::i64),
        tag(", "),
        preceded(tag("y="), nom::character::complete::i64),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> {
    let (input, _) = tag("Sensor at ")(input)?;
    let (input, sensor) = parse_x_y(input)?;
    let (input, _) = tag(": closest beacon is at ")(input)?;
    let (input, beacon) = parse_x_y(input)?;
    Ok((
        input,
        (
            Sensor {
                x: sensor.0,
                y: sensor.1,
            },
            Beacon {
                x: beacon.0,
                y: beacon.1,
            },
        ),
    ))
}

fn dist(sensor: (i64, i64), beacon: (i64, i64)) -> i64 {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn update_bounds(
    sensor: &Sensor,
    distance: &i64,
    bounds: (i64, i64, i64, i64),
) -> (i64, i64, i64, i64) {
    let mut b = bounds;
    if sensor.x + distance > b.0 {
        b.0 = sensor.x + distance;
    }
    if sensor.x - distance < b.1 {
        b.1 = sensor.x - distance;
    }
    if sensor.y + distance > b.2 {
        b.2 = sensor.y + distance;
    }
    if sensor.y - distance < b.3 {
        b.3 = sensor.y - distance;
    }
    b
}

fn possible_beacon(
    pos: (i64, i64),
    sb: &Vec<((i64, i64), i64)>,
    beacon_locations: &HashSet<(i64, i64)>,
) -> bool {
    for (sensor, d_orig) in sb.iter() {
        let d_self = dist(*sensor, pos);
        if d_self <= *d_orig && !beacon_locations.contains(&(pos.0, pos.1)) {
            return false;
        }
    }
    true
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut beacon_locations = HashSet::new();
    let mut bounds = (0, 0, 0, 0);
    let mut sb = Vec::new();
    let cap = 4000000;
    for line in inp {
        let (sensor, beacon) = parse_line(line).unwrap().1;
        let d = dist((sensor.x, sensor.y), (beacon.x, beacon.y));
        bounds = update_bounds(&sensor, &d, bounds);
        sb.push((
            (sensor.x, sensor.y),
            d,
        ));
        beacon_locations.insert((beacon.x, beacon.y));
    }
    let search_height = 2000000;
    let mut i = 0;
    for x in bounds.1..bounds.0 {
        if !possible_beacon((x, search_height), &sb, &beacon_locations) {
            i += 1;
        }
    }
    res.part_1 = i.to_string();
    sb.sort();
    for y in 0..=cap {
        let mut occupied = Vec::new();
        for (coords, d) in sb.iter() {
            if (coords.1 - d..=coords.1 + d).contains(&y) {
                let z = if y < coords.1 {
                    ((coords.1 - d) - y).abs()
                } else if y > coords.1 {
                    ((coords.1 + d) - y).abs()
                } else {
                    *d
                };
                let start = coords.0 - z;
                let end = coords.0 + z;
                let r = start..=end;
                occupied.push(r);
            }
        }
        occupied.sort_unstable_by(|a, b| {
            if a.start() < b.start() {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        let mut iter = occupied.into_iter();
        let first = iter.next().unwrap();
        let range = iter.try_fold(first, |a, b| {
            if a.contains(b.start()) || a.contains(&(*b.start() - 1)) {
                Ok((min(*a.start(), *b.start()))..=(max(*a.end(), *b.end())))
            } else {
                for x in 0..=cap {
                    if possible_beacon((x, y), &sb, &beacon_locations) {
                        res.part_2 = (x * 4000000 + y).to_string();
                    }
                }
                bail!("Huraay")
            }
        });
        if range.is_err() {
            break;
        }
    }
}
