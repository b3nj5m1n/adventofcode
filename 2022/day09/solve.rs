use std::collections::HashSet;
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
    part_1: usize,
    part_2: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Move {
    R(u32),
    L(u32),
    U(u32),
    D(u32),
}

fn update_tail(position_T: &Position, position_H: &Position) -> Position {
    let mut result = Position {
        x: position_T.x,
        y: position_T.y,
    };
    if position_H.y == position_T.y + 2 && position_H.x == position_T.x {
        result = Position {
            x: position_T.x,
            y: position_T.y + 1,
        };
    }
    if position_H.y == position_T.y - 2 && position_H.x == position_T.x {
        result = Position {
            x: position_T.x,
            y: position_T.y - 1,
        };
    }

    if position_H.x == position_T.x + 1 && position_H.y == position_T.y + 2
        || position_H.x == position_T.x + 2 && position_H.y == position_T.y + 1
    {
        result = Position {
            x: position_T.x + 1,
            y: position_T.y + 1,
        };
    }
    if position_H.x == position_T.x - 1 && position_H.y == position_T.y - 2
        || position_H.x == position_T.x - 2 && position_H.y == position_T.y - 1
    {
        result = Position {
            x: position_T.x - 1,
            y: position_T.y - 1,
        };
    }
    if position_H.x == position_T.x + 1 && position_H.y == position_T.y - 2
        || position_H.x == position_T.x + 2 && position_H.y == position_T.y - 1
    {
        result = Position {
            x: position_T.x + 1,
            y: position_T.y - 1,
        };
    }
    if position_H.x == position_T.x - 2 && position_H.y == position_T.y + 1
        || position_H.x == position_T.x - 1 && position_H.y == position_T.y + 2
    {
        result = Position {
            x: position_T.x - 1,
            y: position_T.y + 1,
        };
    }
    if position_H.x == position_T.x + 2 && position_H.y == position_T.y {
        result = Position {
            x: position_T.x + 1,
            y: position_T.y,
        };
    }
    if position_H.x == position_T.x - 2 && position_H.y == position_T.y {
        result = Position {
            x: position_T.x - 1,
            y: position_T.y,
        };
    }
    // Part 2
    if position_H.x == position_T.x + 2 && position_H.y == position_T.y + 2 {
        result = Position {
            x: position_T.x + 1,
            y: position_T.y + 1,
        };
    }
    if position_H.x == position_T.x - 2 && position_H.y == position_T.y + 2 {
        result = Position {
            x: position_T.x - 1,
            y: position_T.y + 1,
        };
    }
    if position_H.x == position_T.x - 2 && position_H.y == position_T.y - 2 {
        result = Position {
            x: position_T.x - 1,
            y: position_T.y - 1,
        };
    }
    if position_H.x == position_T.x + 2 && position_H.y == position_T.y - 2 {
        result = Position {
            x: position_T.x + 1,
            y: position_T.y - 1,
        };
    }
    result
}

fn update_knots(knots: &mut Vec<Position>, position_H: &Position) {
    knots[0] = update_tail(&knots[0], &position_H);
    knots[1] = update_tail(&knots[1], &knots[0]);
    knots[2] = update_tail(&knots[2], &knots[1]);
    knots[3] = update_tail(&knots[3], &knots[2]);
    knots[4] = update_tail(&knots[4], &knots[3]);
    knots[5] = update_tail(&knots[5], &knots[4]);
    knots[6] = update_tail(&knots[6], &knots[5]);
    knots[7] = update_tail(&knots[7], &knots[6]);
    knots[8] = update_tail(&knots[8], &knots[7]);
}

fn update(
    vector: (i32, i32),
    position_H: &mut Position,
    position_T: &mut Position,
    knots: &mut Vec<Position>,
    visited_T: &mut HashSet<Position>,
    visited_9: &mut HashSet<Position>,
) {
    position_H.x += vector.0;
    position_H.y += vector.1;
    *position_T = update_tail(&position_T, &position_H);
    update_knots(knots, &position_H);
    visited_T.insert(position_T.clone());
    visited_9.insert(knots[8].clone());
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut moves = Vec::new();
    for line in inp {
        match line.split(" ").collect::<Vec<&str>>()[..] {
            ["R", x] => moves.push(Move::R(x.parse::<u32>().unwrap())),
            ["L", x] => moves.push(Move::L(x.parse::<u32>().unwrap())),
            ["U", x] => moves.push(Move::U(x.parse::<u32>().unwrap())),
            ["D", x] => moves.push(Move::D(x.parse::<u32>().unwrap())),
            _ => panic!(),
        }
    }
    let mut position_H = Position { x: 0, y: 0 };
    let mut position_T = Position { x: 0, y: 0 };
    let mut knots = vec![Position { x: 0, y: 0 }; 9];
    let mut visited_T = HashSet::new();
    let mut visited_9 = HashSet::new();
    for m in moves {
        let vector = match m {
            Move::R(_) => (1, 0),
            Move::L(_) => (-1, 0),
            Move::U(_) => (0, -1),
            Move::D(_) => (0, 1),
        };
        let count = match m {
            Move::R(x) => x,
            Move::L(x) => x,
            Move::U(x) => x,
            Move::D(x) => x,
        };
        for i in 0..count {
            update(
                vector,
                &mut position_H,
                &mut position_T,
                &mut knots,
                &mut visited_T,
                &mut visited_9,
            );
        }
    }
    res.part_1 = visited_T.len();
    res.part_2 = visited_9.len();
}
