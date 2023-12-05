use std::collections::HashMap;
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

    create_map("seed-to-soil", &inp, &mut seed_to_soil);
    create_map("soil-to-fertilizer", &inp, &mut soil_to_fert);
    create_map("fertilizer-to-water", &inp, &mut fert_to_water);
    create_map("water-to-light", &inp, &mut water_to_light);
    create_map("light-to-temperature", &inp, &mut light_to_temp);
    create_map("temperature-to-humidity", &inp, &mut temp_to_humid);
    create_map("humidity-to-location", &inp, &mut humit_to_location);

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

    let mut seeds_part2: Vec<_> = seeds.chunks_exact(2).map(|e| match e {
        &[a, b] => a..(a + b),
        _ => unreachable!(),
    }).collect();

    seeds_part2.sort_by(|r1, r2| r1.start.partial_cmp(&r2.start).unwrap());

    let mut i = 0;
    'outer: loop {
        i += 1;
        let seed = loc_to_seed(
            i,
            &seed_to_soil,
            &soil_to_fert,
            &fert_to_water,
            &water_to_light,
            &light_to_temp,
            &temp_to_humid,
            &humit_to_location,
        );
        for seeds_range in seeds_part2.iter() {
            if seeds_range.contains(&seed) {
                res.part_2 = i;
                break 'outer;
            }
        }
    }
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
fn loc_to_seed(
    loc: u64,
    seed_to_soil: &HashMap<(u64, u64), u64>,
    soil_to_fert: &HashMap<(u64, u64), u64>,
    fert_to_water: &HashMap<(u64, u64), u64>,
    water_to_light: &HashMap<(u64, u64), u64>,
    light_to_temp: &HashMap<(u64, u64), u64>,
    temp_to_humid: &HashMap<(u64, u64), u64>,
    humit_to_location: &HashMap<(u64, u64), u64>,
) -> u64 {
    let hum = get_rev(loc, &humit_to_location);
    let temp = get_rev(hum, &temp_to_humid);
    let light = get_rev(temp, &light_to_temp);
    let water = get_rev(light, &water_to_light);
    let fert = get_rev(water, &fert_to_water);
    let soil = get_rev(fert, &soil_to_fert);
    let seed = get_rev(soil, &seed_to_soil);
    seed
}

fn get(initial: u64, map: &HashMap<(u64, u64), u64>) -> u64 {
    let mut soil = initial;
    for ((start, range), mapsto) in map.iter() {
        if initial >= *start && initial < *start + *range {
            let offset = range - (start + range - initial);
            soil = mapsto + offset;
        }
    }
    soil
}
fn get_rev(result: u64, map: &HashMap<(u64, u64), u64>) -> u64 {
    let mut seed = result;
    for ((mapsto, range), start) in map.iter() {
        if result >= *start && result < *start + *range {
            let offset = range - (start + range - result);
            seed = mapsto + offset;
        }
    }
    seed
}

// Create hashmap for given mappings, for example "seed-to-soil"
fn create_map(marker: &str, inp: &Vec<&str>, mut dest_map: &mut HashMap<(u64, u64), u64>) {
    let lines = inp
        .iter()
        .clone()
        .skip_while(|l| !l.contains(marker))
        .skip(1)
        .take_while(|l| !l.is_empty() && !l.contains("map"))
        .map(|n| *n)
        .collect();
    idk(&mut dest_map, lines);
}
