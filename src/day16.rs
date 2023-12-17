use std::collections::{HashMap, HashSet};

pub fn solve() {
    let input = std::fs::read_to_string("input/day16.txt").unwrap();

    println!("Day16 Part1: {}", part1(&input));
    println!("Day16 Part2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn beam(grid: Vec<Vec<char>>, x: usize, y: usize, direction: Direction) -> usize {
    let mut tiles: HashMap<_, HashSet<_>> = HashMap::new();
    let mut stack = vec![(x, y, direction)];

    while let Some((x, y, direction)) = stack.pop() {
        if !tiles.entry((x, y)).or_default().insert(direction) {
            continue;
        }

        match direction {
            Direction::Up => {
                if y == 0 {
                    continue;
                }

                let y = y - 1;

                match grid[y][x] {
                    '\\' => stack.push((x, y, Direction::Left)),
                    '/' => stack.push((x, y, Direction::Right)),
                    '-' => {
                        stack.push((x, y, Direction::Left));
                        stack.push((x, y, Direction::Right));
                    }
                    _ => stack.push((x, y, Direction::Up)),
                }
            }
            Direction::Down => {
                if y == grid.len() - 1 {
                    continue;
                }

                let y = y + 1;

                match grid[y][x] {
                    '\\' => stack.push((x, y, Direction::Right)),
                    '/' => stack.push((x, y, Direction::Left)),
                    '-' => {
                        stack.push((x, y, Direction::Left));
                        stack.push((x, y, Direction::Right));
                    }
                    _ => stack.push((x, y, Direction::Down)),
                }
            }
            Direction::Left => {
                if x == 0 {
                    continue;
                }

                let x = x - 1;

                match grid[y][x] {
                    '\\' => stack.push((x, y, Direction::Up)),
                    '/' => stack.push((x, y, Direction::Down)),
                    '|' => {
                        stack.push((x, y, Direction::Up));
                        stack.push((x, y, Direction::Down));
                    }
                    _ => stack.push((x, y, Direction::Left)),
                }
            }
            Direction::Right => {
                if x == grid[0].len() - 1 {
                    continue;
                }

                let x = x + 1;

                match grid[y][x] {
                    '\\' => stack.push((x, y, Direction::Down)),
                    '/' => stack.push((x, y, Direction::Up)),
                    '|' => {
                        stack.push((x, y, Direction::Up));
                        stack.push((x, y, Direction::Down));
                    }
                    _ => stack.push((x, y, Direction::Right)),
                }
            }
        }
    }

    tiles.len()
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    beam(grid, 0, 0, Direction::Right).to_string()
}

fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max = 0;

    for y in 0..grid.len() {
        max = max.max(beam(grid.clone(), 0, y, Direction::Right));
        max = max.max(beam(grid.clone(), grid[y].len() - 1, y, Direction::Left));
    }

    for x in 0..grid[0].len() {
        max = max.max(beam(grid.clone(), x, 0, Direction::Down));
        max = max.max(beam(grid.clone(), x, grid.len() - 1, Direction::Up));
    }

    max.to_string()
}
