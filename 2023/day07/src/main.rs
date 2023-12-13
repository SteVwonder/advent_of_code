use std::env;
use std::cmp::Ordering;

use counter::Counter;

#[derive(Default,Debug,Hash,PartialEq,Eq,Ord,PartialOrd)]
struct Card {
    value: u32,
}

impl Card {
    fn from_char(char: char) -> Card {
        if let Some(value) = char.to_digit(10) {
            return Card{value: value};
        }
        let value = match char {
            'T' => Some(10),
            'J' => Some(11),
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None
        };
        Card{value: value.unwrap()}
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandClass {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Default,Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from_line(line: &String) -> Hand {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        Hand{
            cards: cards_str.chars().map(|x| {Card::from_char(x)}).collect(),
            bid: bid_str.parse::<u32>().unwrap(),
        }
    }

    fn to_class(&self) -> HandClass {
        let common_vec = self.cards.iter().collect::<Counter<_>>().k_most_common_ordered(2);
        let most_common = common_vec[0].1;
        match most_common {
            5 => Some(HandClass::FiveOfAKind),
            4 => Some(HandClass::FourOfAKind),
            3 => {
                match common_vec[1].1 {
                    1 => Some(HandClass::ThreeOfAKind),
                    2 => Some(HandClass::FullHouse),
                    _ => None,
                }
            },
            2 => {
                match common_vec[1].1 {
                    1 => Some(HandClass::OnePair),
                    2 => Some(HandClass::TwoPair),
                    _ => None,
                }
            },
            1 => Some(HandClass::HighCard),
            _ => None
        }.unwrap()
    }
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    // Start by determining "class" of hands, which are ranked in the following
    // order:
    // - Five of a kind
    // - Four of a kind
    // - Full house
    // - Three of a kind
    // - Two pair
    // - One pair
    // - High card
    let class1 = hand1.to_class();
    let class2 = hand2.to_class();
    if class1 != class2 {
        return class1.cmp(&class2);
    }
    // If two hands have the same class, then start looking at the cards in
    // order.  For the first card that doesn't match between the two hands, the
    // higher card wins.
    let (diff1, diff2) = hand1.cards.iter().zip(hand2.cards.iter()).skip_while(|(x,y)| {x == y}).next().unwrap();
    return diff1.cmp(&diff2);
}

fn part1(lines: Vec<String>) {
    // Build a list of hands
    let mut hands = lines.iter().map(|x| {Hand::from_line(x)}).collect::<Vec<Hand>>();
    // Sort the list of hands based on rules
    hands.sort_unstable_by(compare_hands);
    // multiply bids by rank (lowest hand has rank 1), then return sum
    let score: u32 = hands.iter().enumerate().map(|(i, hand)| { (i+1) as u32 * hand.bid}).sum();
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) {
}

fn read_file_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in std::fs::read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];

    let lines = read_file_lines(filename);
    part1(lines.clone());
    part2(lines.clone());
}
