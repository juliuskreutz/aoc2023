use std::{
    cmp::{Ordering, Reverse},
    collections::HashMap,
};

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::{map_res, value},
    multi::{count, separated_list1},
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day07.txt").unwrap();

    println!("Day07 Part1: {}", part1(&input));
    println!("Day07 Part2: {}", part2(&input));
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse(input: &str) -> IResult<&str, Vec<Hand>> {
    let (input, cards) = separated_list1(newline, parse_hand)(input)?;
    Ok((input, cards))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    let (input, cards) = count(parse_card, 5)(input)?;
    let (input, _) = char(' ')(input)?;
    let (input, bid) = map_res(digit1, str::parse::<usize>)(input)?;
    Ok((input, Hand { cards, bid }))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, card) = alt((
        value(Card::Ace, char('A')),
        value(Card::King, char('K')),
        value(Card::Queen, char('Q')),
        value(Card::Jack, char('J')),
        value(Card::Ten, char('T')),
        value(Card::Nine, char('9')),
        value(Card::Eight, char('8')),
        value(Card::Seven, char('7')),
        value(Card::Six, char('6')),
        value(Card::Five, char('5')),
        value(Card::Four, char('4')),
        value(Card::Three, char('3')),
        value(Card::Two, char('2')),
    ))(input)?;
    Ok((input, card))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_card_type(cards: &[Card], part2: bool) -> HandType {
    let mut counts = HashMap::new();

    let mut jokers = 0;
    for &c in cards {
        if part2 && c == Card::Jack {
            jokers += 1;
        } else {
            *counts.entry(c).or_insert(0) += 1;
        }
    }

    if part2 {
        if let Some(value) = counts.values_mut().max() {
            *value += jokers;
        } else {
            counts.insert(Card::Jack, jokers);
        }
    }

    let mut counts = counts.into_values().collect::<Vec<_>>();
    counts.sort_by_key(|&x| Reverse(x));

    match counts[..] {
        [5] => HandType::FiveOfAKind,
        [4, ..] => HandType::FourOfAKind,
        [3, 2] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn compare(x: &[Card], y: &[Card], part2: bool) -> Ordering {
    let x_type = get_card_type(x, part2);
    let y_type = get_card_type(y, part2);

    if x_type != y_type {
        return x_type.cmp(&y_type);
    }

    for (&c1, &c2) in x.iter().zip(y.iter()) {
        if c1 != c2 {
            if part2 {
                if c1 == Card::Jack {
                    return Ordering::Less;
                } else if c2 == Card::Jack {
                    return Ordering::Greater;
                }
            }
            return c1.cmp(&c2);
        }
    }

    Ordering::Equal
}

fn part1(input: &str) -> String {
    let (_, mut cards) = parse(input).unwrap();

    cards.sort_unstable_by(|x, y| compare(&x.cards, &y.cards, false));

    let mut sum = 0;
    for (i, hand) in cards.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let (_, mut cards) = parse(input).unwrap();

    cards.sort_unstable_by(|x, y| compare(&x.cards, &y.cards, true));

    let mut sum = 0;
    for (i, hand) in cards.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    sum.to_string()
}
