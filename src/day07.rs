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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
        [3, 2, ..] => HandType::FullHouse,
        [3, ..] => HandType::ThreeOfAKind,
        [2, 2, ..] => HandType::TwoPair,
        [2, ..] => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

fn compare(x: &[Card], y: &[Card], order: &HashMap<Card, usize>, part2: bool) -> Ordering {
    let x_type = get_card_type(x, part2);
    let y_type = get_card_type(y, part2);

    if x_type != y_type {
        return x_type.cmp(&y_type);
    }

    for (&c1, &c2) in x.iter().zip(y.iter()) {
        if c1 != c2 {
            return order[&c1].cmp(&order[&c2]);
        }
    }

    Ordering::Equal
}

fn part1(input: &str) -> String {
    let (_, mut cards) = parse(input).unwrap();

    let order = HashMap::from_iter([
        (Card::Two, 0),
        (Card::Three, 1),
        (Card::Four, 2),
        (Card::Five, 3),
        (Card::Six, 4),
        (Card::Seven, 5),
        (Card::Eight, 6),
        (Card::Nine, 7),
        (Card::Ten, 8),
        (Card::Jack, 9),
        (Card::Queen, 10),
        (Card::King, 11),
        (Card::Ace, 12),
    ]);
    cards.sort_unstable_by(|x, y| compare(&x.cards, &y.cards, &order, false));

    let mut sum = 0;
    for (i, hand) in cards.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let (_, mut cards) = parse(input).unwrap();

    let order = HashMap::from_iter([
        (Card::Jack, 0),
        (Card::Two, 1),
        (Card::Three, 2),
        (Card::Four, 3),
        (Card::Five, 4),
        (Card::Six, 5),
        (Card::Seven, 6),
        (Card::Eight, 7),
        (Card::Nine, 8),
        (Card::Ten, 9),
        (Card::Queen, 10),
        (Card::King, 11),
        (Card::Ace, 12),
    ]);
    cards.sort_unstable_by(|x, y| compare(&x.cards, &y.cards, &order, true));

    let mut sum = 0;
    for (i, hand) in cards.iter().enumerate() {
        sum += hand.bid * (i + 1);
    }

    sum.to_string()
}
