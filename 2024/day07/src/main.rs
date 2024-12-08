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

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

#[derive(Debug, Clone)]
enum Operator {
    Plus,
    Mul,
    Concat,
}

impl Equation {
    fn sum(&self, operators: Vec<Operator>) -> Option<usize> {
        if !(self.numbers.len() == operators.len() + 1) {
            return None;
        }
        Some(self.numbers.iter().skip(1).zip(operators).fold(
            self.numbers[0],
            |n, (n_, op)| match op {
                Operator::Plus => n + n_,
                Operator::Mul => n * n_,
                Operator::Concat => n * 10_usize.pow(n_.ilog10()+1) + n_,
            },
        ))
    }
    fn possible_p1(&self, operators: Vec<Operator>) -> bool {
        if !(self.numbers.len() == operators.len() + 1) {
            let mut operators_plus = operators.clone();
            operators_plus.push(Operator::Plus);
            let mut operators_mul = operators.clone();
            operators_mul.push(Operator::Mul);
            return self.possible_p1(operators_mul) || self.possible_p1(operators_plus);
        }
        if let Some(s) = self.sum(operators) {
            return s == self.result;
        }
        false
    }
    fn possible_p2(&self, operators: Vec<Operator>) -> bool {
        if !(self.numbers.len() == operators.len() + 1) {
            let mut operators_plus = operators.clone();
            operators_plus.push(Operator::Plus);
            let mut operators_mul = operators.clone();
            operators_mul.push(Operator::Mul);
            let mut operators_con = operators.clone();
            operators_con.push(Operator::Concat);
            return self.possible_p2(operators_mul)
                || self.possible_p2(operators_plus)
                || self.possible_p2(operators_con);
        }
        if let Some(s) = self.sum(operators) {
            return s == self.result;
        }
        false
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let equations = inp
        .into_iter()
        .map(|l| {
            let s = l.split(":").collect::<Vec<&str>>();
            let result = s[0].parse::<usize>().expect("Result not a number");
            let numbers = s[1]
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect("Number not a number"))
                .collect::<Vec<usize>>();
            Equation { result, numbers }
        })
        .collect::<Vec<_>>();
    // dbg!(equations[0].sum(vec![Operator::Concat]));
    res.part_1 = equations
        .iter()
        .map(|e| if e.possible_p1(vec![]) { e.result } else { 0 })
        .sum();
    res.part_2 = equations
        .iter()
        .map(|e| if e.possible_p2(vec![]) { e.result } else { 0 })
        .sum();
}
