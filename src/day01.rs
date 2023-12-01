use std::{collections::HashMap, fs};

pub fn solve() {
    let input = fs::read_to_string("input/day01.txt").unwrap();

    println!("Day01 Part1: {}", part1(&input));
    println!("Day01 Part2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let nums = line
            .chars()
            .filter(|x| x.is_ascii_digit())
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        sum += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    let map: HashMap<&str, u32> = HashMap::from_iter([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in input.lines() {
        let mut first = None;
        let mut second = None;

        let mut line = line.to_string();

        loop {
            for (key, &value) in &map {
                if line.starts_with(key) {
                    first.get_or_insert(value);
                    second = Some(value);
                }
            }

            if let Some(c) = line.chars().next() {
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap();

                    first.get_or_insert(digit);
                    second = Some(digit);
                }
            } else {
                break;
            }

            line = line[1..].to_string();
        }

        sum += first.unwrap() * 10 + second.unwrap();
    }

    sum.to_string()
}
