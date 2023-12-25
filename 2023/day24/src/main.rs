use std::io::Read;
use std::{collections::HashSet, env};

use itertools::Itertools;
use nalgebra::{DMatrix, DVector, Matrix2, Matrix3, Vector2, Vector3, VectorN, LU, U1, U3};

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

fn vec_parallel(v1: &Vector2<f32>, v2: &Vector2<f32>) -> bool {
    (v1.x / v2.x == v1.y / v2.y) || (v1.x == 0.0 && v2.x == 0.0) || (v1.y == 0.0 && v2.y == 0.0)
}

fn find_intersection_point(
    a_start: Vector2<f32>,
    a_dir: Vector2<f32>,
    b_start: Vector2<f32>,
    b_dir: Vector2<f32>,
    bound_lower: f32,
    bound_upper: f32,
) -> bool {
    if vec_parallel(&a_dir, &b_dir) {
        // println!("Vectors are parallel, no intersection");
        return false;
    }

    let dir_matrix = Matrix2::new(a_dir.x, -b_dir.x, a_dir.y, -b_dir.y);

    let start_diff = b_start - a_start;

    let lu = dir_matrix.clone().lu();
    if let Some(solution) = lu.solve(&start_diff) {
        let t = solution.x;
        let s = solution.y;

        if t < 0.0 && s < 0.0 {
            // println!("Vectors crossed in the path for both vectors");
            return false;
        }
        if t < 0.0 {
            // println!("Vectors crossed in the path for vector A");
            return false;
        }
        if s < 0.0 {
            // println!("Vectors crossed in the path for vector B");
            return false;
        }

        // Calculate the intersection point
        let intersection_point = a_start + t * a_dir;

        // println!("Intersection Point: {:?}", intersection_point);
        // println!("t: {}, s: {}", t, s);
        return intersection_point.x >= bound_lower
            && intersection_point.x <= bound_upper
            && intersection_point.y >= bound_lower
            && intersection_point.y <= bound_upper;
    }
    // println!("Vectors don't intersect");
    false
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let vecs = inp
        .into_iter()
        .map(|line| {
            let (start, dir) = line.split_once(" @ ").expect("Failed to parse input");
            let start = start
                .split(",")
                .map(|c| {
                    c.trim()
                        .parse::<f32>()
                        .expect("Failed to parse input as f32")
                })
                .collect::<Vec<_>>();
            let dir = dir
                .split(",")
                .map(|c| {
                    c.trim()
                        .parse::<f32>()
                        .expect("Failed to parse input as f32")
                })
                .collect::<Vec<_>>();
            let start = Vector3::new(start[0], start[1], start[2]);
            let dir = Vector3::new(dir[0], dir[1], dir[2]);
            (start, dir)
        })
        .collect::<Vec<(Vector3<f32>, Vector3<f32>)>>();

    let (bound_lower, bound_upper) = (7.0, 27.0);
    let (bound_lower, bound_upper) = (200000000000000.0, 400000000000000.0);

    for ab in vecs.clone().iter().combinations(2) {
        let ((a_start, a_dir), (b_start, b_dir)) = (ab[0], ab[1]);
        let a_start = Vector2::new(a_start.x, a_start.y);
        let a_dir = Vector2::new(a_dir.x, a_dir.y);
        let b_start = Vector2::new(b_start.x, b_start.y);
        let b_dir = Vector2::new(b_dir.x, b_dir.y);
        if find_intersection_point(a_start, a_dir, b_start, b_dir, bound_lower, bound_upper) {
            res.part_1 += 1;
        }
    }
}
