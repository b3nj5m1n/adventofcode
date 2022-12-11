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
    part_1: i32,
    part_2: i32,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let lines: Vec<Line> = inp.into_iter().map(|line| Line::new(line)).collect();
    let mut points_1 = HashMap::new();
    let mut points_2 = HashMap::new();
    for line in lines {
        for point in line.get_points_hori_verti() {
            let key = (point.x, point.y);
            match points_1.get(&key) {
                Some(count) => {
                    points_1.insert(key, count + 1);
                }
                None => {
                    points_1.insert(key, 1);
                }
            }
        }
        for point in line.get_points() {
            let key = (point.x, point.y);
            match points_2.get(&key) {
                Some(count) => {
                    points_2.insert(key, count + 1);
                }
                None => {
                    points_2.insert(key, 1);
                }
            }
        }
    }
    let mut part_1 = 0;
    for (coords, count) in points_1.iter() {
        if count > &1 {
            part_1 += 1;
        }
    }
    let mut part_2 = 0;
    for (coords, count) in points_2.iter() {
        if count > &1 {
            part_2 += 1;
        }
    }
    res.part_1 = part_1;
    res.part_2 = part_2;
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(s: &str) -> Self {
        let nums: Vec<u32> = s
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect();
        Self {
            x: nums[0],
            y: nums[1],
        }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(s: &str) -> Self {
        let mut points = s.split(" -> ").map(|point| Point::new(point));
        Self {
            start: points.next().unwrap(),
            end: points.next().unwrap(),
        }
    }
    fn get_points_hori_verti(&self) -> Vec<Point> {
        if self.start.x != self.end.x && self.start.y != self.end.y {
            return Vec::new();
        }
        self.get_points()
    }
    fn get_points(&self) -> Vec<Point> {
        let mut result = vec![Point {
            x: self.start.x,
            y: self.start.y,
        }];
        let mut current = Point {
            x: self.start.x,
            y: self.start.y,
        };
        /* println!( "{} == {} -> {:?}", current.x, self.end.x, current.x == self.end.x);
        println!( "{} == {} -> {:?}", current.y, self.end.y, current.y == self.end.y);
        println!( "{:?}", (current.x != self.end.x) || (current.y != self.end.y)); */
        while (current.x != self.end.x) || (current.y != self.end.y) {
            let mut x = current.x;
            let mut y = current.y;
            if x > self.end.x {
                x = current.x - 1;
            }
            if x < self.end.x {
                x = current.x + 1;
            }
            if y > self.end.y {
                y = current.y - 1;
            }
            if y < self.end.y {
                y = current.y + 1;
            }
            current = Point { x, y };
            result.push(Point { x, y });
        }
        result
    }
}
