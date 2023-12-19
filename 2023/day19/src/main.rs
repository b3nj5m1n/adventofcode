use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::str::FromStr;

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
        .map(|l| l.split("\n").filter(|line| !line.is_empty()).collect())
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
    part_1: usize,
    part_2: usize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Attribute {
    X,
    M,
    A,
    S,
}
impl FromStr for Attribute {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(anyhow::anyhow!("Failed to parse Attribute")),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct RuleProper {
    target_attribute: Attribute,
    comparison: std::cmp::Ordering,
    value: u32,
    successor: String,
}
impl FromStr for RuleProper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let target_attribute = Attribute::from_str(&s[0..1])?;
        let comparison = match &s[1..2] {
            ">" => std::cmp::Ordering::Greater,
            "<" => std::cmp::Ordering::Less,
            _ => return Err(anyhow::anyhow!("Failed to parse comparison symbol")),
        };
        let (s_value, successor) = s[2..]
            .split_once(":")
            .ok_or(anyhow::anyhow!("Failed to parse rule count and successor"))?;
        let value = s_value.parse()?;
        Ok(Self {
            target_attribute,
            comparison,
            value,
            successor: successor.to_string(),
        })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum RuleResult {
    Accept,
    Reject,
    ConditionlessSuccessor(String),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Rule {
    RuleProper(RuleProper),
    Accept,
    Reject,
    ConditionlessSuccessor(String),
}
impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            s if !s.contains(":") => Ok(Self::ConditionlessSuccessor(s.into())),
            s => Ok(Self::RuleProper(RuleProper::from_str(s)?)),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl FromStr for Workflow {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let name: String = s.chars().take_while(|&c| c != '{').collect();
        let s_rules: String = s
            .chars()
            .skip_while(|&c| c != '{')
            .skip(1)
            .take_while(|&c| c != '}')
            .collect();
        let rules = s_rules
            .split(",")
            .map(|r| Rule::from_str(r))
            .collect::<anyhow::Result<Vec<Rule>>>()?;
        Ok(Self { name, rules })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}
impl Part {
    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s
            .strip_prefix("{")
            .map(|s| s.strip_suffix("}"))
            .flatten()
            .ok_or(anyhow::anyhow!("Unexpected format for Part"))?;
        let attributes = s
            .split(",")
            .map(|a| {
                a.split_once("=")
                    .expect("Unexpected format for Part")
                    .1
                    .parse::<u32>()
                    .expect("Couldn't parse attribute number")
            })
            .collect::<Vec<u32>>();
        Ok(Self {
            x: attributes[0],
            m: attributes[1],
            a: attributes[2],
            s: attributes[3],
        })
    }
}

// Function to solve both parts
fn solve(inp: Vec<Vec<&str>>, res: &mut Result) {
    let mut workflows = HashMap::new();
    for line in &inp[0] {
        let workflow = Workflow::from_str(line).expect("Failed to parse workflow");
        workflows.insert(workflow.name.clone(), workflow);
    }
    let parts = inp[1]
        .clone()
        .into_iter()
        .map(|l| Part::from_str(l).expect("Failed to parse Part"))
        .collect::<Vec<_>>();

    for part in &parts {
        if is_valid_part(part, &workflows) {
            res.part_1 += part.value() as usize;
        }
    }
}

fn is_valid_part(part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
    let mut current = run_workflow("in", part, &workflows);
    while let Ok(RuleResult::ConditionlessSuccessor(ref s)) = current {
        current = run_workflow(s, part, &workflows);
    }

    if let Ok(RuleResult::Accept) = current {
        return true;
    } else if let Ok(RuleResult::Reject) = current {
        return false;
    } else {
        dbg!(current);
        dbg!(part);
        unreachable!()
    }
}

fn run_workflow(
    workflow: &str,
    part: &Part,
    workflows: &HashMap<String, Workflow>,
) -> anyhow::Result<RuleResult> {
    let workflow = workflows
        .get(workflow)
        .ok_or(anyhow::anyhow!("Workflow not found"))?;
    for rule in &workflow.rules {
        match rule {
            Rule::RuleProper(r) => {
                let value = match r.target_attribute {
                    Attribute::X => part.x,
                    Attribute::M => part.m,
                    Attribute::A => part.a,
                    Attribute::S => part.s,
                };
                if value.cmp(&r.value) == r.comparison {
                    match r.successor.as_str() {
                        "A" => return Ok(RuleResult::Accept),
                        "R" => return Ok(RuleResult::Reject),
                        s => return Ok(RuleResult::ConditionlessSuccessor(s.to_string())),
                    }
                }
            }
            Rule::Accept => return Ok(RuleResult::Accept),
            Rule::Reject => return Ok(RuleResult::Reject),
            Rule::ConditionlessSuccessor(s) => {
                return Ok(RuleResult::ConditionlessSuccessor(s.clone()))
            }
        }
    }
    unreachable!()
}
