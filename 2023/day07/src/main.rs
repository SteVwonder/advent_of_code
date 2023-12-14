use std::env;
use std::cmp::Ordering;

use counter::Counter;

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Clone)]
enum HandClass {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug,Hash,PartialEq,Eq,Ord,PartialOrd,Clone)]
struct Card {
    value: u32,
}

impl Card {
    fn from_char(char: char, part2: bool) -> Card {
        if let Some(value) = char.to_digit(10) {
            return Card{value: value};
        }
        let value = match char {
            'T' => Some(10),
            'J' => {
                match part2 {
                    false => Some(11),
                    true => Some(1),
                }
            }
            'Q' => Some(12),
            'K' => Some(13),
            'A' => Some(14),
            _ => None
        };
        Card{value: value.unwrap()}
    }
}

trait Hand {
    fn cards(&self) -> std::slice::Iter<Card>;
    fn to_class(&self) -> HandClass;
}

#[derive(Debug)]
struct Hand1 {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand1 {
    fn from_line(line: &String) -> Hand1 {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        Hand1{
            cards: cards_str.chars().map(|x| {Card::from_char(x, false)}).collect(),
            bid: bid_str.parse::<u32>().unwrap(),
        }
    }
}

impl Hand for Hand1 {
    fn cards(&self) -> std::slice::Iter<Card> {
        return self.cards.iter();
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

#[derive(Debug)]
struct Hand2 {
    cards: Vec<Card>,
    bid: u32,
    class: HandClass,
}

impl Hand2 {
    fn from_line(line: &String) -> Hand2 {
        let (cards_str, bid_str) = line.split_once(" ").unwrap();
        let cards = cards_str.chars().map(|x| {Card::from_char(x, true)}).collect::<Vec<_>>();
        let common_counter = cards.iter().collect::<Counter<_>>();

        let most_common = common_counter.most_common();
        let class = match common_counter.get(&Card{value: 1}) {
            Some(5) | Some(4) => HandClass::FiveOfAKind,
            Some(3) => {
                match most_common.len() {
                    // Both other cards are the same thing
                    2 => HandClass::FiveOfAKind,
                    // Both other cards are different from each other
                    3 => HandClass::FourOfAKind,
                    _ => panic!("Wrong num of cards"),
                }
            }
            Some(2) => {
                match most_common.len() {
                    // All other cards are the same
                    2 => HandClass::FiveOfAKind,
                    // 2 of one type, 1 of another
                    3 => HandClass::FourOfAKind,
                    // All 3 non-jacks are different types
                    4 => HandClass::ThreeOfAKind,
                    _ => panic!("Wrong num of cards"),
                }
            },
            Some(1) => {
                match most_common.len() {
                    // All other cards are the same
                    2 => HandClass::FiveOfAKind,
                    3 => {
                        // 3 of one type, 1 of another
                        // 2 of one type, 2 of another
                        match most_common.get(0).unwrap().1 {
                            3 => HandClass::FourOfAKind,
                            2 => HandClass::FullHouse,
                            _ => panic!("Wrong num of cards"),
                        }
                    }
                    // 2 of one type, 1 of another, 1 of another
                    4 => HandClass::ThreeOfAKind,
                    // All 4 non-jacks are different types
                    5 => HandClass::OnePair,
                    _ => panic!("Wrong num of cards"),
                }
            },
            None => {
                Hand1{
                    bid: 0,
                    cards: cards.clone(),
                }.to_class()
            },
            Some(_) => panic!("Too many cards in hand"),
        };

        Hand2{
            cards,
            bid: bid_str.parse::<u32>().unwrap(),
            class,
        }
    }
}

impl Hand for Hand2 {
    fn cards(&self) -> std::slice::Iter<Card> {
        return self.cards.iter();
    }

    fn to_class(&self) -> HandClass {
        return self.class.clone()
    }
}

fn compare_hands<I>(hand1: &I, hand2: &I) -> Ordering
where
    I: Hand,
{
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
    let (diff1, diff2) = hand1.cards().zip(hand2.cards()).skip_while(|(x,y)| {x == y}).next().unwrap();
    return diff1.cmp(&diff2);
}

fn part1(lines: Vec<String>) {
    // Build a list of hands
    let mut hands = lines.iter().map(|x| {Hand1::from_line(x)}).collect::<Vec<Hand1>>();
    // Sort the list of hands based on rules
    hands.sort_unstable_by(compare_hands);
    // multiply bids by rank (lowest hand has rank 1), then return sum
    let score: u32 = hands.iter().enumerate().map(|(i, hand)| { (i+1) as u32 * hand.bid}).sum();
    println!("Part1: {}", score);
}

fn part2(lines: Vec<String>) {
    // Build a list of hands
    let mut hands = lines.iter().map(|x| {Hand2::from_line(x)}).collect::<Vec<Hand2>>();
    // Sort the list of hands based on rules
    hands.sort_unstable_by(compare_hands);
    //println!("Hands: {:#?}", hands);
    // multiply bids by rank (lowest hand has rank 1), then return sum
    let score: u32 = hands.iter().enumerate().map(|(i, hand)| { (i+1) as u32 * hand.bid}).sum();
    println!("Part2: {}", score);
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
