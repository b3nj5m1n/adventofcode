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
    let inp: Vec<Vec<&str>> = inp
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .map(|x| {
            x.split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.trim())
                .collect::<Vec<&str>>()
        })
        .collect();

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
    part_1: i32,
    part_2: u128,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
struct Operation {
    operator: Operator,
    a: Option<u128>,
    b: Option<u128>,
}

impl Operation {
    fn apply(&self, old: u128, cap: u128) -> u128 {
        let x = match self.a {
            Some(x) => x,
            None => old,
        };
        let y = match self.b {
            Some(x) => x,
            None => old,
        };
        match self.operator {
            Operator::Add => x + y % cap,
            Operator::Sub => x - y % cap,
            Operator::Mul => x * y % cap,
            Operator::Div => x / y % cap,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    idx: u128,
    starting_items: Vec<u128>,
    operation: Operation,
    test_divisor: u128,
    target_true: u128,
    target_false: u128,
}

fn parse_monkey(monkey: Vec<&str>) -> Monkey {
    let idx = monkey[0].split(" ").collect::<Vec<&str>>()[1]
        .replace(":", "")
        .parse::<u128>()
        .unwrap();
    let list_starting_items = monkey[1].replace("Starting items: ", "");
    let starting_items = list_starting_items
        .split(", ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let r_operation = monkey[2].replace("Operation: new = ", "");
    let s_operation = r_operation.split(" ").collect::<Vec<&str>>();
    let a = match s_operation[0] {
        "old" => None,
        x => Some(x.parse::<u128>().unwrap()),
    };
    let b = match s_operation[2] {
        "old" => None,
        x => Some(x.parse::<u128>().unwrap()),
    };
    let operator = match s_operation[1] {
        "+" => Operator::Add,
        "-" => Operator::Sub,
        "*" => Operator::Mul,
        "/" => Operator::Div,
        _ => panic!(),
    };
    let operation = Operation { operator, a, b };
    let r_test_divisor = monkey[3].replace("Test: divisible by ", "");
    let test_divisor = r_test_divisor.parse::<u128>().unwrap();
    let r_target_true = monkey[4].replace("If true: throw to monkey ", "");
    let target_true = r_target_true.parse::<u128>().unwrap();
    let r_target_false = monkey[5].replace("If false: throw to monkey ", "");
    let target_false = r_target_false.parse::<u128>().unwrap();

    Monkey {
        idx,
        starting_items,
        operation,
        test_divisor,
        target_true,
        target_false,
    }
}

// https://gistlib.com/rust/find-the-least-common-multiple-of-a-list-of-numbers-in-rust
fn lcm(numbers: Vec<u128>) -> u128 {
    let mut lcm = 1;
    for number in numbers {
        lcm = lcm * number / gcd(lcm, number);
    }
    lcm
}
// https://gistlib.com/rust/find-the-least-common-multiple-of-a-list-of-numbers-in-rust
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to solve both parts
fn solve(inp: Vec<Vec<&str>>, res: &mut Result) {
    let mut monkeys = Vec::new();
    let mut items_p1 = Vec::new();
    let mut counter_p1 = Vec::new();
    let mut items_p2 = Vec::new();
    let mut counter_p2 = Vec::new();
    let mut divisors = Vec::new();
    for m in inp {
        let mut monkey = parse_monkey(m);
        items_p1.push(monkey.starting_items.clone());
        counter_p1.push(0);
        items_p2.push(monkey.starting_items.clone());
        counter_p2.push(0);
        divisors.push(monkey.test_divisor);
        monkeys.push(monkey);
    }
    let cap = lcm(divisors);
    for _ in 0..20 {
        for (i, monkey) in monkeys.clone().into_iter().enumerate() {
            for item in &items_p1[i].clone() {
                let worry_level = monkey.operation.apply(*item, cap) / 3;
                if worry_level % monkey.test_divisor == 0 {
                    items_p1[monkey.target_true as usize].push(worry_level);
                } else {
                    items_p1[monkey.target_false as usize].push(worry_level);
                }
                counter_p1[i] += 1;
            }
            items_p1[i] = Vec::new();
        }
    }
    counter_p1.sort();
    res.part_1 = counter_p1.iter().rev().take(2).fold(1, |acc, x| acc * x);

    for _ in 0..10000 {
        for (i, monkey) in monkeys.clone().into_iter().enumerate() {
            for item in &items_p2[i].clone() {
                let worry_level = monkey.operation.apply(*item, cap);
                if worry_level % monkey.test_divisor == 0 {
                    items_p2[monkey.target_true as usize].push(worry_level);
                } else {
                    items_p2[monkey.target_false as usize].push(worry_level);
                }
                counter_p2[i] += 1;
            }
            items_p2[i] = Vec::new();
        }
    }
    counter_p2.sort();
    res.part_2 = counter_p2.iter().rev().take(2).fold(1, |acc, x| acc * x);
}
