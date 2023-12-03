use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day01.txt").unwrap();

    println!("Day01 Part1: {}", part1(&input));
    println!("Day01 Part2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for line in input.lines() {
        for i in 0..line.len() {
            if let Ok(num) = line[i..i + 1].parse::<u32>() {
                sum += 10 * num;
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if let Ok(num) = line[i..i + 1].parse::<u32>() {
                sum += num;
                break;
            }
        }
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
        'outer: for i in 0..line.len() {
            if let Ok(num) = line[i..i + 1].parse::<u32>() {
                sum += 10 * num;
                break;
            }

            for (key, &num) in &map {
                if line[i..].starts_with(key) {
                    sum += 10 * num;
                    break 'outer;
                }
            }
        }

        'outer: for i in (0..line.len()).rev() {
            if let Ok(num) = line[i..i + 1].parse::<u32>() {
                sum += num;
                break;
            }

            for (key, &num) in &map {
                if line[i..].starts_with(key) {
                    sum += num;
                    break 'outer;
                }
            }
        }
    }

    sum.to_string()
}
