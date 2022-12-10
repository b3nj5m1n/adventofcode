use std::env;
use std::io::Read;
use std::collections::HashMap;

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
    part_1: i32,
    part_2: String,
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i32),
}

#[derive(Debug, Clone)]
struct State {
    register: i32,
    cycle: i32,
}

fn execute_instruction(instruction: Instruction, state: &State) -> State {
    match instruction {
        Instruction::Noop => State {
            register: state.register,
            cycle: state.cycle + 1,
        },
        Instruction::Add(x) => State {
            register: state.register + x,
            cycle: state.cycle + 2,
        },
    }
}

fn pixel_visible(state: &State) -> bool {
    let sprite_position = state.register - 1..=state.register + 1;
    sprite_position.contains(&(state.cycle % 40 - 1))
}

const CHAR_LEN: usize = 4;
fn get_char(image: &Vec<String>, index: usize) -> Vec<String> {
    image
        .iter()
        .map(|row| {
            row.chars()
                .skip(index * (CHAR_LEN + 1))
                .take(CHAR_LEN)
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn ocr(letter: &Vec<String>) -> char {
    let mut letters = HashMap::new();
    // Char map from https://github.com/bsoyka/advent-of-code-ocr
    letters.insert(String::from(".##.\n#..#\n#..#\n####\n#..#\n#..#"), 'A');
    letters.insert(String::from("###.\n#..#\n###.\n#..#\n#..#\n###."), 'B');
    letters.insert(String::from(".##.\n#..#\n#...\n#...\n#..#\n.##."), 'C');
    letters.insert(String::from("####\n#...\n###.\n#...\n#...\n####"), 'E');
    letters.insert(String::from("####\n#...\n###.\n#...\n#...\n#..."), 'F');
    letters.insert(String::from(".##.\n#..#\n#...\n#.##\n#..#\n.###"), 'G');
    letters.insert(String::from("#..#\n#..#\n####\n#..#\n#..#\n#..#"), 'H');
    letters.insert(String::from(".###\n..#.\n..#.\n..#.\n..#.\n.###"), 'I');
    letters.insert(String::from("..##\n...#\n...#\n...#\n#..#\n.##."), 'J');
    letters.insert(String::from("#..#\n#.#.\n##..\n#.#.\n#.#.\n#..#"), 'K');
    letters.insert(String::from("#...\n#...\n#...\n#...\n#...\n####"), 'L');
    letters.insert(String::from(".##.\n#..#\n#..#\n#..#\n#..#\n.##."), 'O');
    letters.insert(String::from("###.\n#..#\n#..#\n###.\n#...\n#..."), 'P');
    letters.insert(String::from("###.\n#..#\n#..#\n###.\n#.#.\n#..#"), 'R');
    letters.insert(String::from(".###\n#...\n#...\n.##.\n...#\n###."), 'S');
    letters.insert(String::from("#..#\n#..#\n#..#\n#..#\n#..#\n.##."), 'U');
    letters.insert(String::from("#...\n#...\n.#.#\n..#.\n..#.\n..#."), 'Y');
    letters.insert(String::from("####\n...#\n..#.\n.#..\n#...\n####"), 'Z');
    *letters.get(&letter.join("\n")).unwrap()
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let ins: Vec<Instruction> = inp
        .into_iter()
        .map(|x| {
            let s = x.split(" ").take(2).collect::<Vec<&str>>();
            match s[..] {
                ["noop"] => Instruction::Noop,
                ["addx", x] => Instruction::Add(x.parse::<i32>().unwrap()),
                _ => panic!(),
            }
        })
        .collect();
    let mut state = State {
        register: 1,
        cycle: 1,
    };
    let mut states = vec![state.clone()];
    for instruction in ins {
        let new_state = execute_instruction(instruction, &state);
        for i in state.cycle + 1..new_state.cycle {
            states.push(State {
                register: state.register,
                cycle: i,
            });
        }
        states.push(new_state.clone());
        state = new_state;
    }
    res.part_1 = states
        .clone()
        .into_iter()
        .filter(|x| (x.cycle + 20) % 40 == 0)
        .map(|x| x.register * x.cycle)
        .sum::<i32>();
    let mut image = Vec::new();
    for row in states.clone()[..].chunks(40) {
        let mut pixels = Vec::new();
        for pixel in row {
            if pixel_visible(&pixel) {
                pixels.push("#");
            } else {
                pixels.push(".");
            }
        }
        if pixels.len() == 1 { break; }
        image.push(pixels.into_iter().collect::<String>());
    }
    let mut result_2 = Vec::new();
    for char_index in 0..image[0].len() / (CHAR_LEN + 1) {
        result_2.push(ocr(&get_char(&image, char_index)));
    }
    res.part_2 = result_2.into_iter().collect::<String>();
}
