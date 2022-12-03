use std::collections::HashMap;
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
    part_1: i32,
    part_2: i32,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let scores: HashMap<char, i32> = HashMap::from([
        ('a', 1), ('b', 2), ('c', 3), ('d', 4),
        ('e', 5), ('f', 6), ('g', 7), ('h', 8),
        ('i', 9), ('j', 10), ('k', 11), ('l', 12),
        ('m', 13), ('n', 14), ('o', 15), ('p', 16),
        ('q', 17), ('r', 18), ('s', 19), ('t', 20),
        ('u', 21), ('v', 22), ('w', 23), ('x', 24),
        ('y', 25), ('z', 26), ('A', 27), ('B', 28),
        ('C', 29), ('D', 30), ('E', 31), ('F', 32),
        ('G', 33), ('H', 34), ('I', 35), ('J', 36),
        ('K', 37), ('L', 38), ('M', 39), ('N', 40),
        ('O', 41), ('P', 42), ('Q', 43), ('R', 44),
        ('S', 45), ('T', 46), ('U', 47), ('V', 48),
        ('W', 49), ('X', 50), ('Y', 51), ('Z', 52),
    ]);
    let mut common_items: Vec<i32> = Vec::new();
    let mut history = Vec::new();
    for line in inp {
        let len = line.chars().count();
        let comp_1: String = line.chars().take(len / 2).collect();
        let comp_2: String = line.chars().skip(len / 2).collect();
        let mut items_1: HashSet<i32> = HashSet::new();
        for c in comp_1.chars() {
            items_1.insert(scores.get(&c).unwrap().clone());
        }
        let mut items_2: HashSet<i32> = HashSet::new();
        for c in comp_2.chars() {
            items_2.insert(scores.get(&c).unwrap().clone());
        }
        let intersection = items_1.intersection(&items_2).next().unwrap();
        let union: HashSet<i32> = items_1.union(&items_2).map(|x| x.clone()).collect();
        common_items.push(intersection.clone());
        history.push(union.clone());
    }
    res.part_1 = common_items.into_iter().sum();
    for group in history.chunks(3) {
        let intersection = group.iter().skip(1).fold(group[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });
        res.part_2 += intersection.iter().next().unwrap();
    }
}
