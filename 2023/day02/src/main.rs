use anyhow;
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
        part_2: String::from(""),
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: u32,
    part_2: String,
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
}

impl TryFrom<&str> for Color {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        // dbg!(value);
        match value {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err(anyhow::anyhow!("Failed to parse color.")),
        }
    }
}

#[derive(Debug)]
struct Set {
    color: Color,
    count: u32,
}

impl TryFrom<&str> for Set {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        // dbg!(value);
        let (str_num, str_color) = match value.split_once(" ") {
            Some(tuple) => tuple,
            None => {
                return Err(anyhow::anyhow!(
                    "\"Set\" didn't have expected format of 'number[whitespace]color'"
                ))
            }
        };
        // dbg!(str_num);
        // dbg!(str_color);
        let count = match str_num.parse::<u32>() {
            Ok(c) => c,
            Err(_) => return Err(anyhow::anyhow!("Parsing \"Set\" number")),
        };
        let color: Color = match str_color.try_into() {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
        Ok(Self { color, count })
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Vec<Set>>,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (str_id, str_game) = match value.split_once(": ") {
            Some(t) => t,
            None => return Err(anyhow::anyhow!("\"Game\" Format couldn't be parsed")),
        };
        let id = match str_id.split_once(" ").expect("Fuck").1.parse::<u32>() {
            Ok(id) => id,
            Err(_) => return Err(anyhow::anyhow!("Game id couldn't be parsed as u32")),
        };
        let subsets = str_game.split("; ");
        let mut subs: Vec<Vec<Set>> = Vec::new();
        for subset in subsets {
            let mut sets = Vec::new();
            for set in subset.split(", ") {
                // dbg!(set);
                let set: Set = match set.try_into() {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                };
                sets.push(set);
            }
            subs.push(sets);
        }

        Ok(Self {
            id,
            sets: subs,
        })
    }
}

impl Game {
    fn possible(&self, count_red: u32, count_blue: u32, count_green: u32) -> bool {
        for subset in &self.sets {
            for set in subset {
                match set.color {
                    Color::Red => if set.count > count_red { return false; },
                    Color::Blue => if set.count > count_blue { return false; },
                    Color::Green => if set.count > count_green { return false; },
                }
            }
        }
        true
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    for line in inp {
        // println!("{}", line);
        let g: Game = line.try_into().expect("Couldn't parse game");
        if g.possible(12,14,13) {
            res.part_1 += g.id;
        }
        // dbg!(g);
    }
}
