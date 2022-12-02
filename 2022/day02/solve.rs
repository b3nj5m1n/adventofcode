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
    part_1: i16,
    part_2: i16
}

#[derive(PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum State {
    Won,
    Drew,
    Lost,
    Fuck,
}

const SCORE_ROCK: i16 = 1;
const SCORE_PAPER: i16 = 2;
const SCORE_SCISSORS: i16 = 3;

const SCORE_LOST: i16 = 0;
const SCORE_DRAW: i16 = 3;
const SCORE_WON: i16 = 6;

fn play(opponent: &Shape, own: &Shape) -> State {
    let mut state = State::Fuck;
    if own == &Shape::Rock {
        if opponent == &Shape::Rock {
            state = State::Drew
        } else if opponent == &Shape::Paper {
            state = State::Lost
        } else if opponent == &Shape::Scissors {
            state = State::Won
        }
    } else if own == &Shape::Paper {
        if opponent == &Shape::Rock {
            state = State::Won
        } else if opponent == &Shape::Paper {
            state = State::Drew
        } else if opponent == &Shape::Scissors {
            state = State::Lost
        }
    } else if own == &Shape::Scissors {
        if opponent == &Shape::Rock {
            state = State::Lost
        } else if opponent == &Shape::Paper {
            state = State::Won
        } else if opponent == &Shape::Scissors {
            state = State::Drew
        }
    }
    return state;
}

fn win(opponent: &Shape, desired_state: &State) -> Shape {
    if desired_state == &State::Won {
        if opponent == &Shape::Rock {
            return Shape::Paper;
        } else if opponent == &Shape::Paper {
            return Shape::Scissors;
        } else if opponent == &Shape::Scissors {
            return Shape::Rock;
        }
    } else if desired_state == &State::Drew {
        if opponent == &Shape::Rock {
            return Shape::Rock;
        } else if opponent == &Shape::Paper {
            return Shape::Paper;
        } else if opponent == &Shape::Scissors {
            return Shape::Scissors;
        }
    } else if desired_state == &State::Lost {
        if opponent == &Shape::Rock {
            return Shape::Scissors;
        } else if opponent == &Shape::Paper {
            return Shape::Rock;
        } else if opponent == &Shape::Scissors {
            return Shape::Paper;
        }
    }
    panic!()
}

fn get_shape_score(shape: &Shape) -> i16 {
    match shape {
        Shape::Rock => SCORE_ROCK,
        Shape::Paper => SCORE_PAPER,
        Shape::Scissors => SCORE_SCISSORS,
        _ => panic!(),
    }
}

fn get_state_score(state: &State) -> i16 {
    match state {
        State::Won => SCORE_WON,
        State::Drew => SCORE_DRAW,
        State::Lost => SCORE_LOST,
        _ => panic!(),
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // let mut plays = Vec::new();
    let mut score_1 = 0;
    let mut score_2 = 0;
    for line in inp {
        let mut s = line.split(" ");
        let opponent = match s.next() {
            Some("A") => Shape::Rock,
            Some("B") => Shape::Paper,
            Some("C") => Shape::Scissors,
            _ => panic!(),
        };
        let (own, desired) = match s.next() {
            Some("X") => (Shape::Rock, State::Lost),
            Some("Y") => (Shape::Paper, State::Drew),
            Some("Z") => (Shape::Scissors, State::Won),
            _ => panic!(),
        };
        score_1 = score_1 + get_state_score(&play(&opponent, &own));
        score_1 = score_1 + get_shape_score(&own);

        score_2 = score_2 + get_shape_score(&win(&opponent, &desired));
        score_2 = score_2 + get_state_score(&desired);
    }
    res.part_1 = score_1;
    res.part_2 = score_2;
}
