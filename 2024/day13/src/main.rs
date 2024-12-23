use std::env;
use std::io::Read;

use anyhow::anyhow;
use nalgebra::{ComplexField, Matrix2, Vector2};

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

#[derive(Debug)]
struct Machine {
    button_a: Vector2<f64>,
    button_b: Vector2<f64>,
    prize: Vector2<f64>,
}

impl TryFrom<String> for Machine {
    type Error = anyhow::Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let parse_line = |line: &str| -> Option<(f64, f64)> {
            let mut components = line[line.find(": ").expect("Invalid input") + 2..]
                .split_whitespace()
                .map(|s| {
                    s.replace("X", "")
                        .replace("Y", "")
                        .replace(",", "")
                        .replace("+", "")
                        .replace("=", "")
                });
            let x = components.next()?.parse().ok()?;
            let y = components.next()?.parse().ok()?;
            Some((x, y))
        };

        let mut lines = value.lines().map(parse_line).flatten();
        let button_a = lines
            .next()
            .ok_or(anyhow!("Couldn't find line for Button A"))?;
        let button_b = lines
            .next()
            .ok_or(anyhow!("Couldn't find line for Button B"))?;
        let prize = lines
            .next()
            .ok_or(anyhow!("Couldn't find line for Prize"))?;

        Ok(Machine {
            button_a: Vector2::new(button_a.0, button_a.1),
            button_b: Vector2::new(button_b.0, button_b.1),
            prize: Vector2::new(prize.0, prize.1),
        })
    }
}

impl Machine {
    fn get_matrix(&self) -> Matrix2<f64> {
        Matrix2::new(
            self.button_a[0],
            self.button_a[1],
            self.button_b[0],
            self.button_b[1],
        )
        .transpose()
    }

    fn get_solution(&self) -> Option<Vector2<f64>> {
        let solution = self
            .get_matrix()
            .lu()
            .solve(&self.prize)
            .expect("Claws are linearly dependent");

        const TOLERANCE: f64 = 10E-5;
        if (solution[0] - solution[0].round()).abs() < TOLERANCE
            && (solution[0] - solution[0].round()).abs() < TOLERANCE
        {
            Some(solution)
        } else {
            None
        }
    }

    fn get_score(&self, max_moves: usize) -> Option<usize> {
        let solution_vec = self.get_solution()?;
        let solution = (
            solution_vec[0].round() as usize,
            solution_vec[1].round() as usize,
        );
        if solution.0.max(solution.1) > max_moves {
            return None;
        }
        Some(solution.0 * 3 + solution.1 * 1)
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    // dbg!(&inp);
    let machines = inp
        // .split(|l| l.is_empty())
        .chunks(3)
        .map(|lines| lines.join("\n"))
        .into_iter()
        .map(|l| Machine::try_from(l).expect("Couldn't parse machine"))
        .collect::<Vec<_>>();
    res.part_1 = machines.iter().filter_map(|m| m.get_score(100)).sum();
    res.part_2 = machines
        .iter()
        .map(|machine| Machine {
            prize: machine.prize + Vector2::<f64>::new(10000000000000.0, 10000000000000.0),
            ..*machine
        })
        .filter_map(|m| m.get_score(usize::MAX))
        .sum();
}
