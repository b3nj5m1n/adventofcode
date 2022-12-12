use std::collections::VecDeque;
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

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: u32,
    y: u32,
}

fn bfs(
    grid: &mut Vec<Vec<(u32, bool)>>,
    point_current: Point,
    point_end: Point,
    path: &mut Vec<Point>,
    parent: Point,
    // ) -> Option<Vec<Point>> {
) -> Option<usize> {
    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(point_current);
    let mut history: Vec<(Point, Point)> = Vec::new();
    while let Some(point_current) = queue.pop_front() {
        let neigh_t = if point_current.y >= 1
            && !(grid[(point_current.y as usize) - 1][point_current.x as usize].1)
            && !(point_current == parent)
            && (grid[(point_current.y as usize) - 1][point_current.x as usize].0)
                <= (grid[point_current.y as usize][point_current.x as usize].0 + 1)
        {
            Some(Point {
                x: point_current.x,
                y: point_current.y - 1,
            })
        } else {
            None
        };
        let neigh_b = if (point_current.y as usize) + 1 < grid.len()
            && !(grid[(point_current.y as usize) + 1][point_current.x as usize].1)
            && !(point_current == parent)
            && (grid[(point_current.y as usize) + 1][point_current.x as usize].0)
                <= (grid[point_current.y as usize][point_current.x as usize].0 + 1)
        {
            Some(Point {
                x: point_current.x,
                y: point_current.y + 1,
            })
        } else {
            None
        };
        let neigh_r = if (point_current.x as usize) + 1 < grid[0].len()
            && !(grid[(point_current.y as usize)][point_current.x as usize + 1].1)
            && !(point_current == parent)
            && (grid[point_current.y as usize][(point_current.x as usize) + 1].0)
                <= (grid[point_current.y as usize][point_current.x as usize].0 + 1)
        {
            Some(Point {
                x: point_current.x + 1,
                y: point_current.y,
            })
        } else {
            None
        };
        let neigh_l = if point_current.x >= 1
            && !(grid[(point_current.y as usize)][point_current.x as usize - 1].1)
            && !(point_current == parent)
            && (grid[point_current.y as usize][(point_current.x as usize) - 1].0)
                <= (grid[point_current.y as usize][point_current.x as usize].0 + 1)
        {
            Some(Point {
                x: point_current.x - 1,
                y: point_current.y,
            })
        } else {
            None
        };
        for n in [neigh_t, neigh_b, neigh_r, neigh_l] {
            if let Some(p) = n {
                grid[p.y as usize][p.x as usize].1 = true;
                queue.push_back(p);
                history.push((p, point_current));
                if p == point_end {
                    let mut c = p;
                    while let Some(x) = (history.iter().filter(|x| x.0 == c).next()) {
                        c = x.1;
                        path.push(x.1);
                    }
                    return Some(path.len());
                }
            }
        }
    }
    None
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut grid = Vec::new();
    let mut point_start = Point { x: 0, y: 0 };
    let mut point_end = Point { x: 0, y: 0 };
    let mut points_start_p2 = Vec::new();
    for (i, line) in inp.into_iter().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(j, mountain)| {
                    if mountain == 'S' {
                        point_start = Point {
                            x: j as u32,
                            y: i as u32,
                        };
                        (0, false)
                    } else if mountain == 'E' {
                        point_end = Point {
                            x: j as u32,
                            y: i as u32,
                        };
                        (25, false)
                    } else {
                        let val = ((mountain as u32) - ('a' as u32), false);
                        if val.0 == 0 {
                            points_start_p2.push(Point {
                                x: j as u32,
                                y: i as u32,
                            });
                        }
                        val
                    }
                })
                .collect::<Vec<(u32, bool)>>(),
        );
        // println!("{}", line)
    }
    let mut grid_p1 = grid.clone();
    grid_p1[point_start.y as usize][point_start.x as usize].1 = true;
    let solution = bfs(
        &mut grid_p1,
        point_start,
        point_end,
        &mut Vec::new(),
        point_end,
    )
    .unwrap();
    res.part_1 = solution;
    let mut fewest_steps = solution;
    for point_start in points_start_p2 {
        let mut grid = grid.clone();
        grid[point_start.y as usize][point_start.x as usize].1 = true;
        let solution = bfs(
            &mut grid,
            point_start,
            point_end,
            &mut Vec::new(),
            point_end,
        );
        if let Some(s) = solution {
            if s < fewest_steps {
                fewest_steps = s;
            }
        }
    }
    res.part_2 = fewest_steps;
}
