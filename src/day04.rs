use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();

    println!("Day04 Part1: {}", part1(&input));
    println!("Day04 Part2: {}", part2(&input));
}

#[derive(Clone)]
struct Card {
    id: usize,
    winning: Vec<i32>,
    having: Vec<i32>,
}

fn parse(input: &str) -> IResult<&str, Vec<Card>> {
    let (input, cards) = separated_list1(newline, parse_card)(input)?;
    Ok((input, cards))
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = space1(input)?;
    let (input, winning) = separated_list1(space1, map_res(digit1, str::parse::<i32>))(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = char('|')(input)?;
    let (input, _) = space1(input)?;
    let (input, having) = separated_list1(space1, map_res(digit1, str::parse::<i32>))(input)?;
    Ok((
        input,
        Card {
            id,
            winning,
            having,
        },
    ))
}

fn part1(input: &str) -> String {
    let (_, cards) = parse(input).unwrap();

    let mut sum = 0;
    for card in cards {
        let mut product = 0;
        for having in card.having {
            if card.winning.contains(&having) {
                if product == 0 {
                    product = 1;
                } else {
                    product <<= 1;
                }
            }
        }
        sum += product;
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let (_, cards) = parse(input).unwrap();

    let mut card_stack = cards.clone();

    let mut sum = 0;
    while let Some(card) = card_stack.pop() {
        let mut count = 0;
        for having in card.having {
            if card.winning.contains(&having) {
                count += 1;
            }
        }

        for i in 0..count {
            card_stack.push(cards[card.id + i].clone());
        }

        sum += 1;
    }

    sum.to_string()
}
