use std::collections::{HashMap, HashSet};
use std::env;
use std::io::Read;

use anyhow;

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

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

impl TryFrom<&str> for Hand {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let (cards, bid) = if let Some((cards, bid)) = value.split_once(" ") {
            (cards, bid)
        } else {
            return Err(anyhow::anyhow!("Coudldn't parse hand"));
        };

        let bid = bid.parse::<usize>()?;

        Ok(Self {
            cards: cards.chars().collect(),
            bid,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    fn get_type(&self) -> anyhow::Result<HandType> {
        let mut map: HashMap<&char, usize> = HashMap::new();
        for c in self.cards.iter() {
            map.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut vals: Vec<usize> = map.values().map(|c| *c).collect();
        vals.sort(); //)
        vals.reverse();

        Ok(match vals.as_slice() {
            &[5] => HandType::FiveOfAKind,
            &[4, 1] => HandType::FourOfAKind,
            &[3, 2] => HandType::FullHouse,
            &[3, 1, 1] => HandType::ThreeOfAKind,
            &[2, 2, 1] => HandType::TwoPair,
            &[2, 1, 1, 1] => HandType::OnePair,
            &[1, 1, 1, 1, 1] => HandType::HighCard,
            _ => return anyhow::bail!("Fuck"),
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.get_type().ok()?.partial_cmp(&other.get_type().ok()?) {
            Some(s) => match s {
                std::cmp::Ordering::Less => {
                    return Some(std::cmp::Ordering::Less);
                }
                std::cmp::Ordering::Greater => {
                    return Some(std::cmp::Ordering::Greater);
                }
                std::cmp::Ordering::Equal => {}
            },
            None => return None,
        }
        // It's eq
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 == c2 {
                continue;
            }
            let c1 = match c1 {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => panic!("Unreachable"),
            };
            let c2 = match c2 {
                'A' => 13,
                'K' => 12,
                'Q' => 11,
                'J' => 10,
                'T' => 9,
                '9' => 8,
                '8' => 7,
                '7' => 6,
                '6' => 5,
                '5' => 4,
                '4' => 3,
                '3' => 2,
                '2' => 1,
                _ => panic!("Unreachable"),
            };
            return Some(c1.cmp(&c2));
        }
        None
        // return Some(self.cards.cmp(&other.cards).reverse());
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).expect("Should be unreachable")
    }
}

// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut hands: Vec<Hand> = inp
        .into_iter()
        .map(|line| Hand::try_from(line).expect("Parsing failed"))
        .collect();
    hands.sort(); //)
    let ranks: Vec<_> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1, hand))
        .collect();
    dbg!(&ranks);
    let winnings: Vec<_> = ranks.iter().map(|(i, hand)| i * hand.bid).collect();
    res.part_1 = winnings.iter().sum();
}
