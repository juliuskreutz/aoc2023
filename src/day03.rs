use std::collections::HashMap;

use regex::Regex;

pub fn solve() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();

    println!("Day03 Part1: {}", part1(&input));
    println!("Day03 Part2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for m in re.find_iter(&line.iter().collect::<String>()) {
            let y = y as i32;
            let x = m.start() as i32;

            let mut indices = vec![(y, x - 1), (y, m.end() as i32)];
            for offset_x in x - 1..=m.end() as i32 {
                indices.push((y - 1, offset_x));
                indices.push((y + 1, offset_x));
            }

            let mut valid = false;
            for (iy, ix) in indices {
                if iy < 0 || ix < 0 {
                    continue;
                }

                let ny = iy as usize;
                let nx = ix as usize;

                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }

                if grid[ny][nx] != '.' {
                    valid = true;
                    break;
                }
            }

            if valid {
                sum += m.as_str().parse::<i32>().unwrap();
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut map = HashMap::new();

    let re = Regex::new(r"\d+").unwrap();
    for (y, line) in grid.iter().enumerate() {
        for m in re.find_iter(&line.iter().collect::<String>()) {
            let y = y as i32;
            let x = m.start() as i32;

            let mut indices = vec![(y, x - 1), (y, m.end() as i32)];
            for offset_x in x - 1..=m.end() as i32 {
                indices.push((y - 1, offset_x));
                indices.push((y + 1, offset_x));
            }

            for (iy, ix) in indices {
                if iy < 0 || ix < 0 {
                    continue;
                }

                let ny = iy as usize;
                let nx = ix as usize;

                if ny >= grid.len() || nx >= grid[ny].len() {
                    continue;
                }

                map.entry((ny, nx))
                    .or_insert_with(Vec::new)
                    .push(m.as_str().parse::<i32>().unwrap());
            }
        }
    }

    let mut sum = 0;
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '*' {
                if let Some(values) = map.get(&(y, x)) {
                    if values.len() == 2 {
                        sum += values.iter().product::<i32>();
                    }
                }
            }
        }
    }

    sum.to_string()
}
