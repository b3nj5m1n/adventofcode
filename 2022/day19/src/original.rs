// This was a bit annoying tbh, very similar to day 16, I was way to tired to start working on this
// when it came out, so started a couple hours late, then decided to take another break after
// implementing most of the solution which produced 0 because of a very stupid error on my part.
// Part 1 finished in 31 minutes and 49 seconds.
use lru::LruCache;
use memoize::memoize;
use rayon::prelude::*;
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::num::NonZeroUsize;

use nom::bytes::complete::{tag, take_till, take_until, take_while};
use nom::character::complete::{alpha0, multispace0};
use nom::character::is_alphabetic;
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, separated_pair};
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

#[derive(Debug, Hash, PartialEq, Eq)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    ore_robot: Cost,
    clay_robot: Cost,
    obsidian_robot: Cost,
    geode_robot: Cost,
}

fn extract_resource_cost(l: &Vec<(u32, &str)>, resource_type: &str) -> u32 {
    l.iter()
        .filter(|(_, r_type)| *r_type == resource_type)
        .map(|(num, _)| *num)
        .next()
        .unwrap_or(0)
}

fn parse_cost(input: &str) -> IResult<&str, Cost> {
    let (input, _) = take_while(|c: char| !c.is_digit(10))(input)?;
    let (input, l) = separated_list1(
        tag(" and "),
        separated_pair(nom::character::complete::u32, multispace0, alpha0),
    )(input)?;
    let (input, _) = tag(".")(input)?;
    let ore = extract_resource_cost(&l, "ore");
    let clay = extract_resource_cost(&l, "clay");
    let obsidian = extract_resource_cost(&l, "obsidian");
    Ok((
        input,
        Cost {
            ore,
            clay,
            obsidian,
        },
    ))
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = multispace0(input)?;
    let (input, id) = delimited(tag("Blueprint "), nom::character::complete::u32, tag(":"))(input)?;
    let (input, ore_robot) = parse_cost(input)?;
    let (input, clay_robot) = parse_cost(input)?;
    let (input, obsidian_robot) = parse_cost(input)?;
    let (input, geode_robot) = parse_cost(input)?;
    Ok((
        input,
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        },
    ))
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct CallSig {
    c_ore: u32,
    c_clay: u32,
    c_obsi: u32,
    c_geo: u32,
    cr_ore: u32,
    cr_clay: u32,
    cr_obsi: u32,
    cr_geo: u32,
    min_remaining: u32,
    blueprint: (
        (u32, u32, u32),
        (u32, u32, u32),
        (u32, u32, u32),
        (u32, u32, u32),
    ),
}

// #[memoize(Capacity: 20_000_000)]
fn dfs<'a>(
    c_ore: u32,
    c_clay: u32,
    c_obsi: u32,
    c_geo: u32,
    cr_ore: u32,
    cr_clay: u32,
    cr_obsi: u32,
    cr_geo: u32,
    min_remaining: u32,
    blueprint: (
        (u32, u32, u32),
        (u32, u32, u32),
        (u32, u32, u32),
        (u32, u32, u32),
    ),
    first: bool,
    // cache: &'a mut HashMap<CallSig, u32>,
    cache: &'a mut LruCache<CallSig, u32>,
) -> u32 {
    if min_remaining == 0 {
        return c_geo;
    }
    let call_sig = CallSig {
        c_ore,
        c_clay,
        c_obsi,
        c_geo,
        cr_ore,
        cr_clay,
        cr_obsi,
        cr_geo,
        min_remaining,
        blueprint,
    };
    if let Some(x) = cache.get(&call_sig) {
        return *x;
    }
    // println!("{best:?}");
    /* let c_ore = pc_ore + cr_ore;
    let c_clay = pc_clay + cr_clay;
    let c_obsi = pc_obsi + cr_obsi;
    let c_geo = pc_geo + cr_geo;
    println!("{cr_ore}, {cr_clay}, {cr_obsi}, {cr_geo}"); */
    // dbg!(cr_geo);
    let mut best = 0;
    if c_ore >= (blueprint.3).0 && c_clay >= (blueprint.3).1 && c_obsi >= (blueprint.3).2 {
        let x = dfs(
            c_ore - (blueprint.3).0 + cr_ore,
            c_clay - (blueprint.3).1 + cr_clay,
            c_obsi - (blueprint.3).2 + cr_obsi,
            c_geo + cr_geo,
            cr_ore,
            cr_clay,
            cr_obsi,
            cr_geo + 1,
            min_remaining - 1,
            blueprint,
            false,
            cache,
        );
        return x;
    }
    for (i, robot) in [
        blueprint.0,
        blueprint.1,
        blueprint.2,
        (0, 0, 0),
    ]
    .into_iter()
    .enumerate()
    {
        if c_ore >= robot.0 && c_clay >= robot.1 && c_obsi >= robot.2 {
            let x = dfs(
                c_ore - robot.0 + cr_ore,
                c_clay - robot.1 + cr_clay,
                c_obsi - robot.2 + cr_obsi,
                c_geo + cr_geo,
                cr_ore + if i == 0 { 1 } else { 0 },
                cr_clay + if i == 1 { 1 } else { 0 },
                cr_obsi + if i == 2 { 1 } else { 0 },
                cr_geo,
                min_remaining - 1,
                blueprint,
                false,
                cache,
            );
            // println!("{x}");
            if x > best {
                best = x;
            }
        }
    }
    // dbg!(best);
    // dbg!(min_remaining);
    cache.put(call_sig, best);
    best
}

fn simulate_blueprint(blueprint: &Blueprint) -> u32 {
    let tuple = (
        (
            blueprint.ore_robot.ore,
            blueprint.ore_robot.clay,
            blueprint.ore_robot.obsidian,
        ),
        (
            blueprint.clay_robot.ore,
            blueprint.clay_robot.clay,
            blueprint.clay_robot.obsidian,
        ),
        (
            blueprint.obsidian_robot.ore,
            blueprint.obsidian_robot.clay,
            blueprint.obsidian_robot.obsidian,
        ),
        (
            blueprint.geode_robot.ore,
            blueprint.geode_robot.clay,
            blueprint.geode_robot.obsidian,
        ),
    );
    // dfs(0, 0, 0, 0, 1, 0, 0, 0, 24, tuple, true, &mut HashMap::new())
    let x = dfs(
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
        32,
        tuple,
        true,
        &mut LruCache::new(NonZeroUsize::new(30_000_000).unwrap()),
    );
    println!("finished");
    x
    /* let count_ore = 0; let count_clay = 0; let count_obsidian = 0; let count_geode = 0; let robots_ore = 1; let robots_clay = 0; let robots_obsidian = 0; let robots_geode = 0; 1 */
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let blueprints = inp
        .into_iter()
        .map(|line| parse_blueprint(line).unwrap().1)
        .collect::<Vec<Blueprint>>();
    let x: u32 = blueprints
        .par_iter()
        .take(3)
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * simulate_blueprint(x))
        .sum();
    res.part_1 = x.to_string()
    // dbg!(simulate_blueprint(&blueprints[1]));
}
