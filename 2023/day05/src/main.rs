use std::collections::HashMap;
use std::env;
use std::io::Read;

use rayon::prelude::*;

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
    part_1: u64,
    part_2: u64,
}

fn idk(map: &mut HashMap<(u64, u64), u64>, lines: Vec<&str>) {
    for line in lines {
        // println!("{line}");
        let mut nums = line.split_whitespace().map(|n| n.parse::<u64>().unwrap());
        let start_destination = nums.next().unwrap();
        let start_source = nums.next().unwrap();
        let range_length = nums.next().unwrap();
        map.insert((start_source, range_length), start_destination);
        /* for i in 0..range_length {
            map.insert(start_source + i, start_destination + i);
        } */
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut seed_to_soil = HashMap::new();
    let mut soil_to_fert = HashMap::new();
    let mut fert_to_water = HashMap::new();
    let mut water_to_light = HashMap::new();
    let mut light_to_temp = HashMap::new();
    let mut temp_to_humid = HashMap::new();
    let mut humit_to_location = HashMap::new();

    let mut iter = inp.iter();

    let seeds: Vec<u64> = iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    // println!("Constructing seed_to_soil");
    // dbg!(&iter.clone().skip_while(|l| !l.contains("seed-to-soil")).collect::<Vec<_>>());
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("seed-to-soil"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut seed_to_soil, lines);

    // println!("Constructing soil_to_fert");
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("soil-to-fertilizer"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut soil_to_fert, lines);
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("fertilizer-to-water"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut fert_to_water, lines);
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("water-to-light"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut water_to_light, lines);
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("light-to-temperature"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut light_to_temp, lines);
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("temperature-to-humidity"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut temp_to_humid, lines);
    let lines = iter
        .clone()
        .skip_while(|l| !l.contains("humidity-to-location"))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut humit_to_location, lines);
    // dbg!(&humit_to_location);

    // dbg!(&seed_to_soil);
    let mut locations = Vec::new();
    for seed in seeds.clone() {
        let loc = seed_to_loc(
            seed,
            &mut seed_to_soil,
            &mut soil_to_fert,
            &mut fert_to_water,
            &mut water_to_light,
            &mut light_to_temp,
            &mut temp_to_humid,
            &mut humit_to_location,
        );
        locations.push(loc.clone());
    }
    res.part_1 = *locations.iter().min().unwrap();

    // Now runs in about 1.5 minutes on release build
    let mut locations = Vec::new();
    for i in (0..seeds.len()).filter(|n| n % 2 != 0) {
        // if i % 100 == 0 {
        // println!("i: {i}");
        // }
        let seeds_start = seeds[i - 1];
        let seeds_end = seeds[i];
        let mut locs = (seeds_start..(seeds_start + seeds_end)).into_par_iter().map(|seed| {
            /* if seed % 10000000 == 0 {
                // println!("seed: {}", (seed / (seeds_start + seeds_end)) * 100);
                println!("seed: {}\nof:   {}", seed, seeds_start + seeds_end);
            } */
            // println!("{seed}");
            let loc = seed_to_loc(
                seed,
                &seed_to_soil,
                &soil_to_fert,
                &fert_to_water,
                &water_to_light,
                &light_to_temp,
                &temp_to_humid,
                &humit_to_location,
            );
            loc
            // println!("{loc}\n");
            // locations.push(loc.clone());
        }).collect::<Vec<_>>();
        locations.append(&mut locs);
        // dbg!(seeds_start, seeds_end);
    }
    res.part_2 = *locations.iter().min().unwrap();
}

fn seed_to_loc(
    seed: u64,
    seed_to_soil: &HashMap<(u64, u64), u64>,
    soil_to_fert: &HashMap<(u64, u64), u64>,
    fert_to_water: &HashMap<(u64, u64), u64>,
    water_to_light: &HashMap<(u64, u64), u64>,
    light_to_temp: &HashMap<(u64, u64), u64>,
    temp_to_humid: &HashMap<(u64, u64), u64>,
    humit_to_location: &HashMap<(u64, u64), u64>,
) -> u64 {
    let soil = get(seed, &seed_to_soil);
    let fert = get(soil, &soil_to_fert);
    let water = get(fert, &fert_to_water);
    let light = get(water, &water_to_light);
    let temp = get(light, &light_to_temp);
    let hum = get(temp, &temp_to_humid);
    let loc = get(hum, &humit_to_location);
    loc
}

fn get(initial: u64, map: &HashMap<(u64, u64), u64>) -> u64 {
    let mut soil = initial;
    for ((start, range), mapsto) in map.iter() {
        // dbg!(start, range, mapsto);
        if initial >= *start && initial < *start + *range {
            // dbg!(start, range, mapsto);
            let offset = range - (start + range - initial);
            // dbg!(offset);
            soil = mapsto + offset;
        }
    }
    soil
}
