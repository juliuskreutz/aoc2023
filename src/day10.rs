use std::collections::HashSet;

pub fn solve() {
    let input = std::fs::read_to_string("input/day10.txt").unwrap();

    println!("Day10 Part1: {}", part1(&input));
    println!("Day10 Part2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Pipe {
    start_x: usize,
    start_y: usize,
    start: char,
    pipe: HashSet<(usize, usize)>,
}

fn get_pipe(grid: &[Vec<char>]) -> Pipe {
    let (mut start_x, mut start_y) = (0, 0);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                (start_x, start_y) = (x, y);
                break 'outer;
            }
        }
    }

    let directions = {
        let mut directions = Vec::new();

        if start_y != 0 {
            if let '|' | '7' | 'F' = grid[start_y - 1][start_x] {
                directions.push(Direction::Up)
            }
        }
        if start_x != grid[start_y].len() - 1 {
            if let '-' | 'J' | '7' = grid[start_y][start_x + 1] {
                directions.push(Direction::Right)
            }
        }
        if start_y != grid.len() - 1 {
            if let '|' | 'L' | 'J' = grid[start_y + 1][start_x] {
                directions.push(Direction::Down)
            }
        }
        if start_x != 0 {
            if let '-' | 'L' | 'F' = grid[start_y][start_x - 1] {
                directions.push(Direction::Left)
            }
        }

        directions
    };

    let start = match directions[..] {
        [Direction::Up, Direction::Down] => '|',
        [Direction::Right, Direction::Left] => '-',
        [Direction::Up, Direction::Right] => 'L',
        [Direction::Up, Direction::Left] => 'J',
        [Direction::Down, Direction::Left] => '7',
        [Direction::Right, Direction::Down] => 'F',
        _ => unreachable!(),
    };

    let mut direction = directions[0];
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

    Pipe {
        start_x,
        start_y,
        start,
        pipe,
    }
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    (get_pipe(&grid).pipe.len() / 2).to_string()
}

fn part2(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pipe = get_pipe(&grid);

    grid[pipe.start_y][pipe.start_x] = pipe.start;

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        let mut inside = false;
        for (x, &c) in row.iter().enumerate() {
            if pipe.pipe.contains(&(x, y)) {
                if let '|' | 'L' | 'J' = c {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }

    count.to_string()
}
