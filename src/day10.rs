use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();

    println!("Day10 Part1: {}", part1(&input));
    println!("Day10 Part2: {}", part2(&input));
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn get_pipe(grid: &[Vec<char>]) -> HashSet<(usize, usize)> {
    let (mut start_x, mut start_y) = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                (start_x, start_y) = (x, y);
                break 'outer;
            }
        }
    }

    let mut direction = 'direction: {
        if start_y != 0 {
            match grid[start_y - 1][start_x] {
                '|' | '7' | 'F' => break 'direction Direction::Up,
                _ => {}
            }
        }
        if start_x != grid[start_y].len() - 1 {
            match grid[start_y][start_x + 1] {
                '-' | 'J' | '7' => break 'direction Direction::Right,
                _ => {}
            }
        }
        if start_y != grid.len() - 1 {
            match grid[start_y + 1][start_x] {
                '|' | 'L' | 'J' => break 'direction Direction::Down,
                _ => {}
            }
        }
        if start_x != 0 {
            match grid[start_y][start_x - 1] {
                '-' | 'L' | 'F' => break 'direction Direction::Left,
                _ => {}
            }
        }

        unreachable!()
    };

    let (mut current_x, mut current_y) = (start_x, start_y);
    let mut pipe = HashSet::new();

    loop {
        if !pipe.insert((current_x, current_y)) {
            break;
        }

        match direction {
            Direction::Up => {
                current_y -= 1;

                match grid[current_y][current_x] {
                    '7' => direction = Direction::Left,
                    'F' => direction = Direction::Right,
                    _ => {}
                }
            }
            Direction::Right => {
                current_x += 1;

                match grid[current_y][current_x] {
                    'J' => direction = Direction::Up,
                    '7' => direction = Direction::Down,
                    _ => {}
                }
            }
            Direction::Down => {
                current_y += 1;

                match grid[current_y][current_x] {
                    'L' => direction = Direction::Right,
                    'J' => direction = Direction::Left,
                    _ => {}
                }
            }
            Direction::Left => {
                current_x -= 1;

                match grid[current_y][current_x] {
                    'L' => direction = Direction::Up,
                    'F' => direction = Direction::Down,
                    _ => {}
                }
            }
        }
    }

    pipe
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (get_pipe(&grid).len() / 2).to_string()
}

fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pipe = get_pipe(&grid);

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut inside = false;
        for (x, &c) in row.iter().enumerate() {
            if pipe.contains(&(x, y)) {
                match c {
                    'S' if x != row.len() - 1 && row[x + 1] != '-' => {
                        inside = !inside;
                    }
                    '|' | '7' | 'F' => {
                        inside = !inside;
                    }
                    _ => {}
                }
            } else if inside {
                count += 1;
            }
        }
    }

    count.to_string()
}
