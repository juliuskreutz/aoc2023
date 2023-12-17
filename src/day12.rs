use std::iter;

use nom::{
    branch::alt,
    character::complete::{char, digit1, newline, space1},
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day12.txt").unwrap();

    println!("Day12 Part1: {}", part1(&input));
    println!("Day12 Part2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Hotspring {
    conditions: Vec<Condition>,
    record: Vec<usize>,
}

fn parse(input: &str) -> IResult<&str, Vec<Hotspring>> {
    let (input, hotsprings) = separated_list1(newline, parse_hotspring)(input)?;
    Ok((input, hotsprings))
}

fn parse_hotspring(input: &str) -> IResult<&str, Hotspring> {
    let (input, conditions) = many1(alt((
        value(Condition::Operational, char('.')),
        value(Condition::Damaged, char('#')),
        value(Condition::Unknown, char('?')),
    )))(input)?;
    let (input, _) = space1(input)?;
    let (input, record) = separated_list1(char(','), map_res(digit1, str::parse))(input)?;
    Ok((input, Hotspring { conditions, record }))
}

fn permutations<T: Clone>(items: Vec<T>) -> Vec<Vec<T>>
where
    T: Ord,
{
    if items.len() == 1 {
        vec![items]
    } else {
        let mut output: Vec<Vec<T>> = vec![];

        let mut unique_items = items.clone();
        unique_items.sort();
        unique_items.dedup();
        for first in unique_items {
            let mut remaining_elements = items.clone();

            let index = remaining_elements.iter().position(|x| *x == first).unwrap();
            remaining_elements.remove(index);

            for mut permutation in permutations(remaining_elements) {
                permutation.insert(0, first.clone());
                output.push(permutation);
            }
        }
        output
    }
}

fn part1(input: &str) -> String {
    let (_, hotsprings) = parse(input).unwrap();

    let mut sum = 0;
    for hotspring in hotsprings {
        let unknown = hotspring
            .conditions
            .iter()
            .filter(|&&c| c == Condition::Unknown)
            .count();

        let expected_damaged: usize = hotspring.record.iter().sum();
        let known_damaged = hotspring
            .conditions
            .iter()
            .filter(|&&c| c == Condition::Damaged)
            .count();
        let unknown_damaged = expected_damaged - known_damaged;
        let unknown_operational = unknown - unknown_damaged;

        let v: Vec<_> = iter::repeat(Condition::Damaged)
            .take(unknown_damaged)
            .chain(iter::repeat(Condition::Operational).take(unknown_operational))
            .collect();

        'outer: for permutation in permutations(v) {
            let mut conditions = hotspring.conditions.clone();
            let mut i = 0;
            for condition in conditions.iter_mut() {
                if *condition == Condition::Unknown {
                    *condition = permutation[i];
                    i += 1;
                }
            }

            conditions.reverse();

            while let Some(Condition::Operational) = conditions.last() {
                conditions.pop();
            }

            for &record in &hotspring.record {
                for _ in 0..record {
                    match conditions.pop() {
                        Some(Condition::Damaged) => {}
                        _ => continue 'outer,
                    }
                }
                if let Some(Condition::Damaged) = conditions.pop() {
                    continue 'outer;
                }
                while let Some(Condition::Operational) = conditions.last() {
                    conditions.pop();
                }
            }

            if !conditions.is_empty() {
                continue;
            }

            sum += 1;
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let (_, hotsprings) = parse(input).unwrap();

    for mut hotspring in hotsprings {
        let mut conditions = hotspring.conditions.clone();
        conditions.insert(0, Condition::Unknown);
        for _ in 0..5 {
            hotspring.conditions.append(&mut conditions.clone());
        }

        let records = hotspring.record.clone();
        for _ in 0..5 {
            hotspring.record.append(&mut records.clone());
        }
    }

    "TODO".to_string()
}
