use std::collections::HashSet;
use std::env;
use std::io::Read;
use std::thread;

use kdam::tqdm;
use memoize::memoize;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

fn output(result: &Result) { println!("Part 1: {}", &result.part_1); println!("Part 2: {}", &result.part_2); }

fn main() { let args: Vec<String> = env::args().collect(); let mut file_handle = std::fs::File::open(&args[1]).unwrap(); let mut inp = String::new(); file_handle.read_to_string(&mut inp).unwrap(); let inp: Vec<&str> = inp.split("\n").filter(|line| !line.is_empty()).collect(); let mut result: Result = Result { part_1: String::from(""), part_2: String::from(""), }; solve(inp, &mut result); output(&result); }

struct Result { part_1: String, part_2: String, }

#[derive(Debug, Clone)]
struct Sensor { x: i64, y: i64, }
#[derive(Debug, Clone)]
struct Beacon { x: i64, y: i64, }

fn parse_x_y(input: &str) -> IResult<&str, (i64, i64)> { separated_pair( preceded(tag("x="), nom::character::complete::i64), tag(", "), preceded(tag("y="), nom::character::complete::i64),)(input) }

fn parse_line(input: &str) -> IResult<&str, (Sensor, Beacon)> { let (input, _) = tag("Sensor at ")(input)?; let (input, sensor) = parse_x_y(input)?; let (input, _) = tag(": closest beacon is at ")(input)?; let (input, beacon) = parse_x_y(input)?; Ok(( input, ( Sensor { x: sensor.0, y: sensor.1, }, Beacon { x: beacon.0, y: beacon.1, },),)) }

fn dist(sensor: (i64, i64), beacon: (i64, i64)) -> i64 { (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs() }

fn update_bounds( sensor: &Sensor, beacon: &Beacon, bounds: (i64, i64, i64, i64),) -> (i64, i64, i64, i64) { let mut b = bounds; if sensor.x > b.0 { b.0 = sensor.x; } if sensor.x < b.1 { b.1 = sensor.x; } if sensor.y > b.2 { b.2 = sensor.y; } if sensor.y < b.3 { b.3 = sensor.y; } if beacon.x > b.0 { b.0 = beacon.x; } if beacon.x < b.1 { b.1 = beacon.x; } if beacon.y > b.2 { b.2 = beacon.y; } if beacon.y < b.3 { b.3 = beacon.y; } b }

fn possible_beacon( pos: (i64, i64), sb: &Vec<((i64, i64), i64)>, beacon_locations: &HashSet<(i64, i64)>, sensor_locations: &HashSet<(i64, i64)>,) -> bool { for (sensor, d_orig) in sb.iter() { let d_self = dist(*sensor, pos); if d_self <= *d_orig { return false; } } true }

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // let mut covered_locations = HashSet::new();
    let mut sensor_locations = HashSet::new(); let mut beacon_locations = HashSet::new(); let mut bounds = (0, 0, 0, 0); let mut sb = Vec::new(); let mut sb_p2 = Vec::new();
    let cap = 4000000;
    for line in inp {
        let (sensor, beacon) = parse_line(line).unwrap().1;
        bounds = update_bounds(&sensor, &beacon, bounds);
        let d = dist((sensor.x, sensor.y), (beacon.x, beacon.y));
        sb.push((sensor.clone(), d));
        sensor_locations.insert((sensor.x, sensor.y));
        beacon_locations.insert((beacon.x, beacon.y));
        sb_p2.push((
            (sensor.x, sensor.y),
            dist((sensor.x, sensor.y), (beacon.x, beacon.y)),
        ));
    }
    let mut i = 0;
    sb_p2.sort();
    let mut threads = Vec::new();
    for i in 0..8 {
        let sb_p2_ = sb_p2.clone();
        let beacon_locations_ = beacon_locations.clone();
        let sensor_locations_ = sensor_locations.clone();
        threads.push(thread::spawn(move || {
            for x in (cap/8 * i)..=(cap/8 * i) + (cap/8) {
                    if x % 100 == 0 {
                        println!("{x}");
                    }
                for y in 0..=cap {
                    if possible_beacon((x, y), &sb_p2_, &beacon_locations_, &sensor_locations_) {
                        println!("solution::{}::solution", x * 4000000 + y);
                        panic!();
                    }
                }
            }
        }));
    }
    for thread in threads {
        let _ = thread.join();
    }
}
