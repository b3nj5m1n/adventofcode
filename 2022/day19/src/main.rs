// 4 hours 41 minute runtime for the example, 2 hours 13 minutes for the actual input

use std::io::Read;
use std::{env, str::FromStr};

use cached::proc_macro::cached;
use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use yansi::Paint;

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}
impl FromStr for ResourceType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ore" => Ok(Self::Ore),
            "clay" => Ok(Self::Clay),
            "obsidian" => Ok(Self::Obsidian),
            "geode" => Ok(Self::Geode),
            _ => return Err(anyhow::anyhow!("Couldn't parse resource type")),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord, Default)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Blueprint {
    id: u32,
    cost_ore: Resources,
    cost_clay: Resources,
    cost_obsidian: Resources,
    cost_geode: Resources,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct Robots {
    count_ore_robots: u32,
    count_clay_robots: u32,
    count_obsidian_robots: u32,
    count_geode_robots: u32,
}
impl Default for Robots {
    fn default() -> Self {
        Self {
            count_ore_robots: 1,
            count_clay_robots: 0,
            count_obsidian_robots: 0,
            count_geode_robots: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct State {
    robots: Robots,
    robots_building_age0: Robots,
    robots_building_age1: Robots,
    resources: Resources,
}
impl Default for State {
    fn default() -> Self {
        State {
            robots: Robots::default(),
            robots_building_age0: Robots {
                count_ore_robots: 0,
                count_clay_robots: 0,
                count_obsidian_robots: 0,
                count_geode_robots: 0,
            },
            robots_building_age1: Robots {
                count_ore_robots: 0,
                count_clay_robots: 0,
                count_obsidian_robots: 0,
                count_geode_robots: 0,
            },
            resources: Resources::default(),
        }
    }
}

impl FromStr for Blueprint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (id, content) = s.split_once(":").expect("Parsing failed");
        let id = id
            .trim()
            .split_once(" ")
            .expect("Parsing failed")
            .1
            .parse::<u32>()
            .expect("Parsing failed");
        let costs = content
            .split(".")
            .filter(|l| !l.is_empty())
            .map(|s| {
                let tmp = s.replace("Each ", "").replace(" robot costs ", ",");
                let tmp = tmp.trim();
                let (type_s, costs) = tmp.split_once(",").expect("Fuck");
                let r_type = ResourceType::from_str(type_s).expect("Parsing resource type failed");
                let costs_raw = costs.split(" and ").map(|cost| {
                    let (amount, r_type) = cost.split_once(" ").expect("fuck");
                    let amount = amount
                        .parse::<u32>()
                        .expect("Couldn't parse blueprint cost as u32");
                    let r_type =
                        ResourceType::from_str(r_type).expect("Couldn't parse blueprint type");
                    (amount, r_type)
                });
                let mut costs = Resources::default();
                for cost in costs_raw {
                    match cost.1 {
                        ResourceType::Ore => costs.ore = cost.0,
                        ResourceType::Clay => costs.clay = cost.0,
                        ResourceType::Obsidian => costs.obsidian = cost.0,
                        ResourceType::Geode => costs.geode = cost.0,
                    }
                }
                (r_type, costs)
            })
            .collect::<Vec<(ResourceType, Resources)>>();
        let mut result = Self {
            id,
            cost_ore: Resources::default(),
            cost_clay: Resources::default(),
            cost_obsidian: Resources::default(),
            cost_geode: Resources::default(),
        };
        for (r_type, cost) in costs {
            match r_type {
                ResourceType::Ore => result.cost_ore = cost,
                ResourceType::Clay => result.cost_clay = cost,
                ResourceType::Obsidian => result.cost_obsidian = cost,
                ResourceType::Geode => result.cost_geode = cost,
            }
        }
        Ok(result)
    }
}

fn quality_level(state: &State, bp: Blueprint) -> u32 {
    state.resources.geode * bp.id
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let bps = inp
        .into_iter()
        .map(|l| Blueprint::from_str(l).expect("Parsing blueprint failed"))
        .collect::<Vec<_>>();
    dbg!(&bps);
    res.part_1 = bps
    .clone()
    .into_par_iter()
    .map(|bp| {
        let state = State::default();
        let best_cycle = best_cycle(state, bp, 0, 24, &mut 0)
            .expect("Didn't find valid cycle for blueprint");
        println!(
            "{} {} ({})",
            Paint::fixed(
                (((0b101010 | bp.id << 5) * bp.id) % 255) as u8,
                "Finished calculating best score for blueprint"
            ),
            Paint::new(bp.id).bold(),
            Paint::new(best_cycle.resources.geode)
                .bg(yansi::Color::Fixed(
                    (((0b101010 | bp.id << 5) * bp.id) % 255) as u8
                ))
                .dimmed()
                .bold()
        );
        quality_level(&best_cycle, bp)
    })
    .sum();
    res.part_2 = bps
        .into_par_iter()
        .take(3)
        .map(|bp| {
            let state = State::default();
            let best_cycle = best_cycle(state, bp, 0, 32, &mut 0)
                .expect("Didn't find valid cycle for blueprint");
            println!(
                "{} {} ({})",
                Paint::fixed(
                    (((0b101010 | bp.id << 5) * bp.id) % 255) as u8,
                    "Finished calculating best score for blueprint"
                ),
                Paint::new(bp.id).bold(),
                Paint::new(best_cycle.resources.geode)
                    .bg(yansi::Color::Fixed(
                        (((0b101010 | bp.id << 5) * bp.id) % 255) as u8
                    ))
                    .dimmed()
                    .bold()
            );
            best_cycle.resources.geode
        })
        .product();
}

fn best_cycle(
    current_state: State,
    bp: Blueprint,
    current_depth: u32,
    search_depth: u32,
    current_best: &mut u32,
) -> Option<State> {
    // Begin cycle by collecting resources from current robots
    let state = collect_resources(&current_state);

    // println!("{depth}");
    /* if *current_best
        > geode_robot_possible_1(
            state.resources.geode,
            state.robots.count_geode_robots,
            current_depth,
            search_depth,
        )
    {
        return None;
    } */
    /* if *current_best > geode_robot_possible_2(state, current_depth, search_depth, bp) {
        return None;
    } */
    if *current_best > geode_robot_possible_3(state, current_depth, search_depth, bp) {
        return None;
    }
    if current_depth == search_depth - 1 {
        // let mut best = best().lock().expect("Failed to obtain lock for best");
        if *current_best < state.resources.geode {
            let color = (((0b101010 | bp.id << 5) * bp.id) % 255) as u8;
            /* let r = ( color | 16 ) % 255;
            let g = ( color | 32 ) % 255;
            let b = ( color | 64 ) % 255; */
            println!(
                "Updated best from {} to {}",
                // Paint::rgb(r, g, b, current_best.to_string()),
                Paint::fixed(color, current_best.to_string()).bold(),
                Paint::fixed(color, state.resources.geode.to_string()).bold(),
            );
            *current_best = state.resources.geode;
            return Some(state);
        }
        return None;
    }

    let to_beat = current_best.clone();
    match [
        Some(ResourceType::Geode),
        Some(ResourceType::Obsidian),
        Some(ResourceType::Clay),
        Some(ResourceType::Ore),
        None,
    ]
    .into_iter()
    .filter(|resource_type| can_build(*resource_type, &state, bp))
    // .map(|resource_type| register_building_robot(&state, resource_type, bp))
    // .map(|state| update_robots(&state))
    /* .filter(|state| {
        let best_for_this = geode_robot_possible_1(state.resources.geode, state.robots.count_geode_robots, current_depth, search_depth);
        best_for_this > to_beat
    }) */
    .filter_map(|resource_type| {
        /* if geode_robot_possible_1(current_state, current_depth, search_depth, bp) < *current_best {
            return None;
        }
        if geode_robot_possible_2(current_state, current_depth, search_depth, bp) < *current_best {
            return None;
        } */
        let state = register_building_robot(&state, resource_type, bp);
        let state = update_robots(&state);
        if to_beat > geode_robot_possible_3(state, current_depth, search_depth, bp) {
            return None;
        }
        best_cycle(state, bp, current_depth + 1, search_depth, current_best)
    })
    .reduce(|a, b| {
        if a.resources.geode > b.resources.geode {
            a
        } else {
            b
        }
    }) {
        Some(s) => Some(s),
        None => Some(state),
    }
}

fn geode_robot_possible_1(
    current_geode_count: u32,
    current_geode_robots_count: u32,
    current_depth: u32,
    search_depth: u32,
) -> u32 {
    let most_possible_robots = ((search_depth - 2) * (search_depth - 1)) / 2
        - ((current_depth.checked_sub(2).unwrap_or_default())
            * (current_depth.checked_sub(1).unwrap_or_default()))
            / 2;
    let most_possible_geodes =
        current_geode_count + current_geode_robots_count + most_possible_robots;
    most_possible_geodes
}

// Given how much obsidian it takes to build a geode robot, check if that amount of osidian can be
// obtained in the remaining time (upper bound)
// Consider the case where we're building an obsidian robot every minute
fn geode_robot_possible_2(
    current_state: State,
    current_depth: u32,
    search_depth: u32,
    bp: Blueprint,
) -> u32 {
    let mut state = current_state.clone();
    for _ in current_depth..=search_depth - 1 {
        state.resources.ore += state.robots.count_ore_robots;
        state.resources.clay += state.robots.count_clay_robots;
        state.resources.obsidian += state.robots.count_obsidian_robots;
        state.resources.geode += state.robots.count_geode_robots;
        state.robots.count_ore_robots += 1;
        state.robots.count_clay_robots += 1;
        state.robots.count_obsidian_robots += 1;
        state.robots.count_geode_robots += 1;
    }
    state.resources.geode += state.robots.count_geode_robots;

    state.resources.geode
}

fn geode_robot_possible_3(
    current_state: State,
    current_depth: u32,
    search_depth: u32,
    bp: Blueprint,
) -> u32 {
    let mut state = current_state.clone();
    for _ in current_depth..=search_depth - 1 {
        state = update_robots(&state);
        state.resources.ore += state.robots.count_ore_robots;
        state.resources.clay += state.robots.count_clay_robots;
        state.resources.obsidian += state.robots.count_obsidian_robots;
        state.resources.geode += state.robots.count_geode_robots;

        for r_type in [
            Some(ResourceType::Ore),
            Some(ResourceType::Clay),
            Some(ResourceType::Obsidian),
            Some(ResourceType::Geode),
        ] {
            let resources = state.resources;
            state = register_building_robot(&state, r_type, bp);
            state.resources = resources;
        }
    }
    state.resources.geode += state.robots.count_geode_robots;

    state.resources.geode
}

fn update_robots(current_state: &State) -> State {
    let mut new_state = current_state.clone();
    new_state.robots.count_ore_robots += current_state.robots_building_age1.count_ore_robots;
    new_state.robots.count_clay_robots += current_state.robots_building_age1.count_clay_robots;
    new_state.robots.count_obsidian_robots +=
        current_state.robots_building_age1.count_obsidian_robots;
    new_state.robots.count_geode_robots += current_state.robots_building_age1.count_geode_robots;
    new_state.robots_building_age1 = current_state.robots_building_age0;
    new_state.robots_building_age0.count_ore_robots = 0;
    new_state.robots_building_age0.count_clay_robots = 0;
    new_state.robots_building_age0.count_obsidian_robots = 0;
    new_state.robots_building_age0.count_geode_robots = 0;
    new_state
}

fn register_building_robot(
    current_state: &State,
    want_to_build: Option<ResourceType>,
    bp: Blueprint,
) -> State {
    let mut new_state = current_state.clone();
    if !can_build(want_to_build, current_state, bp) {
        return *current_state;
        // panic!("Called build_robot with a state that cannot produce requested robot");
    }
    let want_to_build = if let Some(want_to_build) = want_to_build {
        want_to_build
    } else {
        return *current_state;
    };
    match want_to_build {
        ResourceType::Ore => {
            new_state.robots_building_age0.count_ore_robots += 1;
            new_state.resources.ore -= bp.cost_ore.ore;
            new_state.resources.clay -= bp.cost_ore.clay;
            new_state.resources.obsidian -= bp.cost_ore.obsidian;
        }
        ResourceType::Clay => {
            new_state.robots_building_age0.count_clay_robots += 1;
            new_state.resources.ore -= bp.cost_clay.ore;
            new_state.resources.clay -= bp.cost_clay.clay;
            new_state.resources.obsidian -= bp.cost_clay.obsidian;
        }
        ResourceType::Obsidian => {
            new_state.robots_building_age0.count_obsidian_robots += 1;
            new_state.resources.ore -= bp.cost_obsidian.ore;
            new_state.resources.clay -= bp.cost_obsidian.clay;
            new_state.resources.obsidian -= bp.cost_obsidian.obsidian;
        }
        ResourceType::Geode => {
            new_state.robots_building_age0.count_geode_robots += 1;
            new_state.resources.ore -= bp.cost_geode.ore;
            new_state.resources.clay -= bp.cost_geode.clay;
            new_state.resources.obsidian -= bp.cost_geode.obsidian;
        }
    }
    new_state
}

fn collect_resources(current_state: &State) -> State {
    let mut new_state = current_state.clone();
    new_state.resources.ore += current_state.robots.count_ore_robots;
    new_state.resources.clay += current_state.robots.count_clay_robots;
    new_state.resources.obsidian += current_state.robots.count_obsidian_robots;
    new_state.resources.geode += current_state.robots.count_geode_robots;
    new_state
}

fn can_build(want_to_build: Option<ResourceType>, state: &State, bp: Blueprint) -> bool {
    let resource_stores = state.resources;
    let want_to_build = if let Some(want_to_build) = want_to_build {
        want_to_build
    } else {
        return true;
    };
    match want_to_build {
        ResourceType::Ore => {
            return resource_stores.ore >= bp.cost_ore.ore
                && resource_stores.clay >= bp.cost_ore.clay
                && resource_stores.obsidian >= bp.cost_ore.obsidian
        }
        ResourceType::Clay => {
            return resource_stores.ore >= bp.cost_clay.ore
                && resource_stores.clay >= bp.cost_clay.clay
                && resource_stores.obsidian >= bp.cost_clay.obsidian
        }
        ResourceType::Obsidian => {
            return resource_stores.ore >= bp.cost_obsidian.ore
                && resource_stores.clay >= bp.cost_obsidian.clay
                && resource_stores.obsidian >= bp.cost_obsidian.obsidian
        }
        ResourceType::Geode => {
            return resource_stores.ore >= bp.cost_geode.ore
                && resource_stores.clay >= bp.cost_geode.clay
                && resource_stores.obsidian >= bp.cost_geode.obsidian
        }
    }
}
