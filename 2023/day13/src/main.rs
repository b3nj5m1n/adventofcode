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
    let inp: Vec<&str> = inp.split("\n").collect();

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

/* fn check_vertical(inp: &Vec<&str>, idx: usize) -> Option<(usize, usize)> {
    let i = idx;
    let j = i + 1;
    // let range = (((idx as i64 + 1) - (inp.len() as i64) - 1_i64) as usize).min(idx);
    let range = (((inp[0].len() as i64) - (idx as i64) - 1_i64) as usize).min(idx);
    /* if i-range != 0 && j+range != inp[0].len() {
        println!("i: {i}, j: {j}, range: {range}, idx: {idx}, len: {}", inp[0].len());
        dbg!(inp);
    } */
    if inp
        .iter()
        .map(|line| line.chars().nth(i).expect("Unreachable"))
        .collect::<String>()
        != inp
            .iter()
            .map(|line| line.chars().nth(j).expect("Unreachable"))
            .collect::<String>()
    {
        return None;
    }
    for d in 0..range {
        /* println!(
            "[{}, {}] (range: {range}) (len: {})",
            i - d,
            j + d,
            inp.len()
        ); */
        let current = inp
            .iter()
            .map(|line| line.chars().nth(i - d).expect("Unreachable"))
            .collect::<String>();
        let next = inp
            .iter()
            .map(|line| line.chars().nth(j + d).expect("Unreachable"))
            .collect::<String>();

        if current != next {
            return None;
            // return Some((d, i - d + 1));
        }
    }

    return Some((range, i));
}
fn check_horizontal(inp: &Vec<&str>, idx: usize) -> Option<(usize, usize)> {
    let i = idx;
    let j = i + 1;
    let range = (((inp.len() as i64) - (idx as i64) - 1_i64) as usize).min(idx);
    if inp[i] != inp[j] {
        return None;
    }
    for d in 0..range {
        /* println!(
            "[{}, {}] (range: {range}) (len: {})",
            i - d,
            j + d,
            inp.len()
        ); */

        let current = inp[i - d];
        let next = inp[j + d];

        if current != next {
            return None;
            // return Some((d, i - d + 1));
        }
    }

    return Some((range, i));
}

fn score(inp: &Vec<&str>) -> usize {
    // dbg!(inp);
    if inp.len() == 0 {
        return 0;
    }
    let mut top_h = Vec::new();
    let mut top_v = Vec::new();
    for (x_1, x_2) in (0..(inp[0].len() - 1).max(inp.len() - 1))
        .zip((0..(inp[0].len() - 1).max(inp.len() - 1)).rev())
    {
        for x in [x_2, x_1] {
                println!("{x}");
            if x < inp.len() - 1 {
                if let Some((range, idk)) = check_horizontal(&inp, x) {
                    // println!("Mirrors horizontally between , starting from {y},{}", y + 1);
                    // dbg!(range, idk, y);
                    let score = (x - range + 1);
                    let score = x + 1;
                    // return score * 100;
                    top_h.push((range, score));
                    /* if score > top_h {
                        top_h = score;
                    } */
                    // println!("Score of {score}")
                }
            }
            if x < inp[0].len() - 1 {
                if let Some((range, idk)) = check_vertical(&inp, x) {
                    /* println!(
                        "Mirrors vertically between , starting from {x},{} with range {range}",
                        x + 1
                    ); */
                    let score = (x - range + 1);
                    let score = x + 1;
                    // return score;
                    top_v.push((range, score));
                    /* if score > top_v {
                        top_v = score;
                    } */
                    // println!("Score of {score}")
                }
            }
        }
        // }
    }
    for y in 0..inp.len() - 1 {}
    /* if top_v == 0 && top_h == 0 {
        println!("Zero score for input found");
        dbg!(inp);
    } */
    top_v.sort_by(|a, b| a.1.cmp(&b.1));
    top_v.reverse();
    top_h.sort_by(|a, b| a.1.cmp(&b.1));
    top_h.reverse();
    /* println!("{:?}", top_v[0]);
    println!("{:?}", top_h[0]); */
    /* assert!(top_v[0].0 + 1 == top_v[0].1);
    assert!(top_h[0].0 + 1 == top_h[0].1); */
    // assert!(top_v.len() == 0 || top_h.len() == 0);
    // dbg!(&top_v, &top_h);
    if top_h.len() == 0 {
        println!("Using vertical mirror with score {}", top_v[0].1);
        return (top_v[0].1);
    } else if top_v.len() == 0 {
        println!("Using horizontal mirror with score {}", (top_h[0].1));
        return (top_h[0].1) * 100;
    } else {
        if top_v[0] == top_h[0] {
            println!(
                "Scores are the same, using horizontal mirror with score {}",
                top_h[0].1
            );
            return (top_h[0].1 * 100);
        }
        if top_v[0].1 > top_h[0].1 {
            println!("Using vertical mirror with score {}", top_v[0].1);
            return (top_v[0].1);
        } else {
            println!("Using horizontal mirror with score {}", (top_h[0].1));
            return (top_h[0].1) * 100;
        }
    }
} */

fn check_horizontal(inp: &Vec<impl ToString>, i_orig: usize) -> bool {
    let mut i = i_orig;
    let mut j = i + 1;
    loop {
        if inp[i].to_string() != inp[j].to_string() {
            return false;
        }
        if i == 0 || j == inp.len() - 1 {
            return true;
        }
        i -= 1;
        j += 1;
        /* if !(inp.get(i).is_some() && inp.get(j).is_some()) {
            break;
        } */
    }
    true
}

fn score(inp: &Vec<&str>) -> Option<(usize, usize, u8)> {
    // dbg!(inp);
    if inp.len() == 0 {
        return Some((0, 0, 0));
    }
    let transposed: Vec<String> = (0..inp[0].len())
        .map(|col| {
            (0..inp.len())
                .rev()
                .map(|row| inp[row].chars().nth(col).expect("fuck").clone())
                .collect()
        })
        .collect();
    // assert!(transposed_orig == transposed);
    // dbg!(&transposed);
    for i in 0..(inp.len() - 1).max(transposed.len() - 1) {
        // println!("Checking {i}");
        if i < inp.len() - 1 && check_horizontal(inp, i) {
            let score = (i + 1) * 100;
            // println!("Score: {}", score);
            return Some((score, i, 1));
        }
        if i < transposed.len() - 1 && check_horizontal(&transposed, i) {
            let score = i + 1;
            // println!("Score: {}", score);
            return Some((score, i, 2));
        }
    }

    None
}

fn score_p2(
    inp: &Vec<&str>,
    (original_mirror_line_idx, original_mirror_line_type): (usize, u8),
) -> usize {
    if inp.len() == 0 {
        return 0;
    }
    let mut orig = None;
    for y in 0..inp.len() {
        for x in 0..inp[0].len() {
            let current = match inp[y].chars().nth(x) {
                Some('.') => '#',
                Some('#') => '.',
                _ => unreachable!(),
            };
            let mut new_grid = inp.clone();
            let chars = new_grid[y].chars();
            let chars_before: String = chars.clone().take(x).collect();
            let chars_after: String = chars.skip(x + 1).collect();
            new_grid[y] = (chars_before + &current.to_string() + &chars_after).leak();
            /* println!("Old: {}", inp[y]);
            println!("New: {}", new_grid[y]); */
            if let Some((score, i, mirror_type)) = score(&new_grid) {
                if i != original_mirror_line_idx && mirror_type != original_mirror_line_type {
                    // println!("Found new mirror when switching {y}, {x}. Score: {score}. I: {i}");
                    return score;
                } else if mirror_type != original_mirror_line_type {
                    return score;
                } else {
                    orig = Some(score);
                }
            }
        }
    }
    dbg!(orig);
    return orig.expect("Hopefully unreachable");
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut scores_p1 = HashMap::new();
    res.part_1 = inp
        .split(|line| line.is_empty())
        .enumerate()
        .map(|(idx, inp)| {
            let (score, i, mirror_type) =
                score(&inp.to_vec()).expect("Didn't find mirror for part 1");
            scores_p1.insert(idx, (i, mirror_type));
            score
        })
        .sum::<usize>();

    res.part_2 = inp
        .split(|line| line.is_empty())
        .enumerate()
        .map(|(idx, inp)| score_p2(&inp.to_vec(), *scores_p1.get(&idx).expect("fuck")))
        .sum::<usize>();
}
