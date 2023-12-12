use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, newline},
    combinator::value,
    multi::{count, many0, separated_list0},
    sequence::{delimited, pair, separated_pair},
    IResult,
};
use num::Integer;

pub fn solve() {
    let input = std::fs::read_to_string("input/day08.txt").unwrap();

    println!("Day08 Part1: {}", part1(&input));
    println!("Day08 Part2: {}", part2(&input));
}

struct Navigation {
    directions: Vec<Direction>,
    nodes: HashMap<[char; 3], Node>,
}

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

struct Node {
    name: [char; 3],
    left: [char; 3],
    right: [char; 3],
}

fn parse(input: &str) -> IResult<&str, Navigation> {
    let (input, directions) = many0(alt((
        value(Direction::Left, char('L')),
        value(Direction::Right, char('R')),
    )))(input)?;
    let (input, _) = pair(newline, newline)(input)?;
    let (input, nodes) = separated_list0(newline, parse_node)(input)?;
    let nodes = nodes.into_iter().map(|node| (node.name, node)).collect();

    Ok((input, Navigation { directions, nodes }))
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    let (input, name) = count(anychar, 3)(input)?;
    let (input, _) = tag(" = ")(input)?;
    let (input, (left, right)) = delimited(
        char('('),
        separated_pair(count(anychar, 3), tag(", "), count(anychar, 3)),
        char(')'),
    )(input)?;

    Ok((
        input,
        Node {
            name: name.try_into().unwrap(),
            left: left.try_into().unwrap(),
            right: right.try_into().unwrap(),
        },
    ))
}

fn part1(input: &str) -> String {
    let (_, navigation) = parse(input).unwrap();

    let mut count = 0;
    let mut current = ['A', 'A', 'A'];
    loop {
        let node = &navigation.nodes[&current];

        let direction = navigation.directions[count % navigation.directions.len()];
        match direction {
            Direction::Left => current = node.left,
            Direction::Right => current = node.right,
        }

        count += 1;

        if current == ['Z', 'Z', 'Z'] {
            break;
        }
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let (_, navigation) = parse(input).unwrap();

    let mut counts = Vec::new();
    for current in navigation
        .nodes
        .keys()
        .copied()
        .filter(|&[_, _, c]| c == 'A')
    {
        let mut count = 0;
        let mut current = current;
        loop {
            let node = &navigation.nodes[&current];

            let direction = navigation.directions[count % navigation.directions.len()];
            match direction {
                Direction::Left => current = node.left,
                Direction::Right => current = node.right,
            }

            count += 1;

            if current[2] == 'Z' {
                break;
            }
        }
        counts.push(count);
    }

    let mut lcm = counts[0];
    for count in counts.into_iter().skip(1) {
        lcm = lcm.lcm(&count);
    }

    lcm.to_string()
}
