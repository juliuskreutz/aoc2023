use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();

    println!("Day01 Part1: {}", part1(&input));
    println!("Day01 Part2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let nums = line
            .chars()
            .flat_map(|x| x.to_digit(10))
            .collect::<Vec<_>>();

        sum += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
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

    let mut sum = 0;
    for line in input.lines() {
        let mut nums = vec![];

        let mut line = line.to_string();

        loop {
            for (key, &value) in &map {
                if line.starts_with(key) {
                    nums.push(value);
                }
            }

            if let Some(c) = line.chars().next() {
                if let Some(digit) = c.to_digit(10) {
                    nums.push(digit);
                }
            } else {
                break;
            }

            line = line[1..].to_string();
        }

        sum += nums.first().unwrap() * 10 + nums.last().unwrap();
    }

    sum.to_string()
}
