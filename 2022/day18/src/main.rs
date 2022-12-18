// Part 1 was super easy, I initially wasn't sure what part 2 was asking, so but I kind of figured
// a floodfill would be the way to go, still had to go check the solution thread because of a very
// weird bug that took me _way_ too long to figure out.
use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
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
        part_1: String::from(""),
        part_2: String::from(""),
    };

    // Solve
    solve(inp, &mut result);
    // Output the solutions
    output(&result);
}

// Struct for solution values
struct Result {
    part_1: String,
    part_2: String,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

fn get_neighbours<'a>(cube: &Cube, grid: &'a HashSet<Cube>) -> HashSet<&'a Cube> {
    let mut potential_neighbours = HashSet::new();
    for x in [-1, 1] {
        potential_neighbours.insert(Cube {
            x: cube.x + x,
            y: cube.y,
            z: cube.z,
        });
    }
    for y in [-1, 1] {
        potential_neighbours.insert(Cube {
            x: cube.x,
            y: cube.y + y,
            z: cube.z,
        });
    }
    for z in [-1, 1] {
        potential_neighbours.insert(Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z + z,
        });
    }
    let mut neighbours = HashSet::new();
    for potential_neighbour in potential_neighbours.iter() {
        if grid.contains(&potential_neighbour) {
            neighbours.insert(grid.get(&potential_neighbour).unwrap());
        }
    }
    neighbours
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid = HashSet::new();
    let mut bounds_min = (0, 0, 0);
    let mut bounds_max = (0, 0, 0);
    for line in inp {
        let v = line
            .split(",")
            .take(3)
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        bounds_min = (
            min(bounds_min.0, v[0]),
            min(bounds_min.1, v[1]),
            min(bounds_min.2, v[2]),
        );
        bounds_max = (
            max(bounds_max.0, v[0]),
            max(bounds_max.1, v[1]),
            max(bounds_max.2, v[2]),
        );
        grid.insert(Cube {
            x: v[0],
            y: v[1],
            z: v[2],
        });
    }
    let mut r = 0;
    for cube in grid.iter() {
        let c = get_neighbours(&cube, &grid).len();
        r += 6 - c;
    }
    res.part_1 = r.to_string();
    let mut queue = VecDeque::new();
    queue.push_front(Cube {
        x: bounds_min.0 - 2,
        y: bounds_min.1 - 2,
        z: bounds_min.2 - 2,
    });
    let mut outside = HashSet::new();
    let mut visited = HashSet::new();
    let b = 2;
    while queue.len() > 0 {
        let curr = queue.pop_front().unwrap();
        visited.insert(curr);
        let neighbours = get_neighbours(&curr, &grid);
        for i in [-1, 1] {
            for c in [
                Cube {
                    x: curr.x + i,
                    y: curr.y,
                    z: curr.z,
                },
                Cube {
                    x: curr.x,
                    y: curr.y + i,
                    z: curr.z,
                },
                Cube {
                    x: curr.x,
                    y: curr.y,
                    z: curr.z + i,
                },
            ] {
                if c.x < bounds_min.0 - b
                    || c.x >= bounds_max.0 + b
                    || c.y < bounds_min.0 - b
                    || c.y >= bounds_max.0 + b
                    || c.z < bounds_min.0 - b
                    || c.z >= bounds_max.0 + b
                {
                    continue;
                }
                if visited.contains(&c) || queue.contains(&c) {
                    continue;
                }
                if neighbours.contains(&c) {
                    outside.insert(c);
                } else {
                    queue.push_back(c);
                }
            }
        }
    }
    let mut r2 = 0;
    for cube in outside.into_iter() {
        let c = get_neighbours(&cube, &visited).len();
        r2 += c;
    }
    res.part_2 = r2.to_string();
}
