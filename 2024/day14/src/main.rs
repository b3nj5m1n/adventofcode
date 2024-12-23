use std::collections::HashSet;
use std::env;
use std::io::Read;
use std::ops::{Add, Mul};

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

#[derive(Debug, Clone, Copy, Hash, PartialEq, PartialOrd, Ord, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

impl Robot {
    fn simulate(&self, steps: i64, width: i64, height: i64) -> Self {
        let res = self.pos + steps * self.vel;

        let pos = Vec2 {
            x: res.x.rem_euclid(width),
            y: res.y.rem_euclid(height),
        };

        Robot { pos, vel: self.vel }
    }

    fn quadrant(&self, width: i64, height: i64) -> u8 {
        if self.pos.x < (width - 1) / 2 && self.pos.y < (height - 1) / 2 {
            2
        } else if self.pos.x < (width - 1) / 2 && self.pos.y > (height - 1) / 2 {
            3
        } else if self.pos.x > (width - 1) / 2 && self.pos.y < (height - 1) / 2 {
            4
        } else if self.pos.x > (width - 1) / 2 && self.pos.y > (height - 1) / 2 {
            1
        } else {
            0
        }
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    /* for line in inp {
        println!("{}", line)
    } */
    let orig_robots = inp
        .into_iter()
        .map(|l| {
            let mut a = l.split_whitespace();
            let p = a
                .next()
                .expect("Didn't find position")
                .replace("p=", "")
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let v = a
                .next()
                .expect("Didn't find velocity")
                .replace("v=", "")
                .split(",")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            Robot {
                pos: Vec2 { x: p[0], y: p[1] },
                vel: Vec2 { x: v[0], y: v[1] },
            }
        })
        .collect::<Vec<_>>();

    let time = 100;
    /* // Test
    let width = 11;
    let height = 7; */
    // Input
    let width = 101;
    let height = 103;

    let robots = orig_robots
        .clone()
        .into_iter()
        .map(|r| r.simulate(time, width, height))
        .collect::<Vec<_>>();

    let robots = robots
        .into_iter()
        .map(|r| r.quadrant(width, height))
        .counts();

    res.part_1 = robots.get(&1).unwrap_or(&1)
        * robots.get(&2).unwrap_or(&1)
        * robots.get(&3).unwrap_or(&1)
        * robots.get(&4).unwrap_or(&1);

    /* let search_space: i64 = 10_i64.pow(10);
    let _ = (search_space..=search_space*10)
        .into_par_iter()
        .progress_count(( search_space*10 - search_space ) as u64)
        .map(|i| {
            /* if i % 10000000 == 0 {
                println!("{i}");
            } */
            if width as usize
                == orig_robots
                    .clone()
                    .into_iter()
                    .map(|r| r.simulate(i, width, height))
                    .map(|r| r.pos.x)
                    .filter(|x| *x == (width - 1) / 2 + 1)
                    .count()
            {
                // res.part_2 = i as usize;
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                println!("Solution: {i}");
                // return i;
            }
        })
        .count(); */
}

// 176 too low

impl Mul<Vec2> for i64 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
