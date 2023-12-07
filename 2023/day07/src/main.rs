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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn get_value_card(c: &char, part_2: bool) -> u32 {
    if !part_2 {
        match c {
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
        }
    } else {
        match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => panic!("Unreachable"),
        }
    }
}

impl Hand {
    fn get_type(&self) -> anyhow::Result<(HandType, HandType)> {
        let joker_count = self.cards.iter().filter(|&c| *c == 'J').count();
        let mut map: HashMap<&char, usize> = HashMap::new();
        for c in self.cards.iter().filter(|&c| *c != 'J') {
            map.entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut vals: Vec<usize> = map.values().map(|c| *c).collect();

        // Part 1
        if joker_count > 0 {
            vals.push(joker_count);
        }
        vals.sort();
        vals.reverse();
        let p1 = match vals.as_slice() {
            &[5] => HandType::FiveOfAKind,
            &[4, 1] => HandType::FourOfAKind,
            &[3, 2] => HandType::FullHouse,
            &[3, 1, 1] => HandType::ThreeOfAKind,
            &[2, 2, 1] => HandType::TwoPair,
            &[2, 1, 1, 1] => HandType::OnePair,
            &[1, 1, 1, 1, 1] => HandType::HighCard,
            _ => return anyhow::bail!("Fuck"),
        };
        if joker_count > 0 {
            vals.remove(vals.iter().position(|x| *x == joker_count).expect("fuck"));
        }

        let p2 = if joker_count > 0 {
            // Part 2
            vals.sort();
            vals.reverse();
            // All jokers
            if vals.is_empty() {
                vals.push(5);
            } else {
                vals[0] += joker_count;
            }

            match vals.as_slice() {
                &[5] => HandType::FiveOfAKind,
                &[4, 1] => HandType::FourOfAKind,
                &[3, 2] => HandType::FullHouse,
                &[3, 1, 1] => HandType::ThreeOfAKind,
                &[2, 2, 1] => HandType::TwoPair,
                &[2, 1, 1, 1] => HandType::OnePair,
                &[1, 1, 1, 1, 1] => HandType::HighCard,
                _ => return anyhow::bail!("Fuck"),
            }
        } else {
            p1
        };
        Ok((p1, p2))
    }
    fn cmp(&self, other: &Self, part_2: bool) -> std::cmp::Ordering {
        let type_self = match part_2 {
            true => self.get_type().expect("").1,
            false => self.get_type().expect("").0,
        };
        let type_other = match part_2 {
            true => other.get_type().expect("").1,
            false => other.get_type().expect("").0,
        };
        match type_self.cmp(&type_other) {
            std::cmp::Ordering::Less => {
                return std::cmp::Ordering::Less;
            }
            std::cmp::Ordering::Greater => {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal => {}
        }
        // It's eq
        for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
            if c1 == c2 {
                continue;
            }
            let c1 = get_value_card(c1, part_2);
            let c2 = get_value_card(c2, part_2);
            return c1.cmp(&c2);
        }
        unreachable!()
    }
}

// TODO Solve part 1 again (b72fc878d13e81e41452f1657d829788838f12d7)
// Function to solve both parts
fn solve(inp: Vec<&str>, res: &mut Result) {
    let mut hands: Vec<Hand> = inp
        .into_iter()
        .map(|line| Hand::try_from(line).expect("Parsing failed"))
        .collect();
    // Part 1
    hands.sort_by(|a, b| a.cmp(b, false));
    let ranks: Vec<_> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1, hand))
        .collect();
    let winnings: Vec<_> = ranks.iter().map(|(i, hand)| i * hand.bid).collect();
    res.part_1 = winnings.iter().sum();
    hands.sort_by(|a, b| a.cmp(b, true));
    let ranks: Vec<_> = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1, hand))
        .collect();
    let winnings: Vec<_> = ranks.iter().map(|(i, hand)| i * hand.bid).collect();
    res.part_2 = winnings.iter().sum();
}
