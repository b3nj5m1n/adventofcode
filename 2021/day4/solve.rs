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
    part_1: u64,
    part_2: i32,
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut inp_split = inp.split(|line| line.is_empty());
    let draws = inp_split.next().unwrap()[0]
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    let mut boards: Vec<Board> = inp_split
        .map(|l| Board::new(l.to_vec().into_iter().map(|s| String::from(s)).collect()))
        .filter(|board| board.fields.len() > 0)
        .collect();
    for draw in draws {
        for board in &mut boards {
            board.draw(draw);
            if board.won() {
                res.part_1 = board.score() * u64::from(draw);
                return;
            }
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    NotDrawn,
    Drawn,
}

#[derive(Debug, Clone)]
struct Field {
    number: u8,
    state: State,
}

#[derive(Debug, Clone)]
struct Board {
    fields: Vec<Field>,
    won: bool,
    width: u8,
    height: u8,
}

impl Board {
    fn new(lines: Vec<String>) -> Self {
        let mut fields = Vec::new();
        for line in lines {
            fields.append(
                &mut line
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|number| Field {
                        number: number.parse::<u8>().unwrap(),
                        state: State::NotDrawn,
                    })
                    .collect(),
            );
        }
        Self {
            fields,
            won: false,
            width: 5,
            height: 5,
        }
    }
    fn score(&self) -> u64 {
        let mut result = 0;
        for field in &self.fields {
            if let State::NotDrawn = field.state {
                result += u64::from(field.number);
            }
        }
        result
    }
    fn draw(&mut self, num: u8) {
        for i in 0..self.width {
            for j in 0..self.height {
                if self.fields[usize::from(i * self.width + j)].number == num {
                    self.fields[usize::from(i * self.width + j)].state = State::Drawn;
                }
            }
        }
    }
    fn get_by_index(&self, x: u8, y: u8) -> &Field {
        &self.fields[usize::from(y * self.width + x)]
    }
    fn get_line(&self, index: u8) -> Vec<&Field> {
        let mut line = Vec::new();
        for i in 0..self.width {
            line.push(self.get_by_index(i, index))
        }
        line
    }
    fn get_column(&self, index: u8) -> Vec<&Field> {
        let mut column = Vec::new();
        for i in 0..self.height {
            column.push(self.get_by_index(index, i))
        }
        column
    }
    fn won(&self) -> bool {
        for i in 0..self.height {
            let drawn = self
                .get_line(i)
                .into_iter()
                .map(|field| {
                    if let State::Drawn = field.state {
                        true
                    } else {
                        false
                    }
                })
                .fold(true, |a, b| a & b);
            if drawn {
                return true;
            }
        }
        for i in 0..self.height {
            let drawn = self
                .get_column(i)
                .into_iter()
                .map(|field| {
                    if let State::Drawn = field.state {
                        true
                    } else {
                        false
                    }
                })
                .fold(true, |a, b| a & b);
            if drawn {
                return true;
            }
        }
        false
    }
}
