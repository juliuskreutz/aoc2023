use std::collections::HashMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day14.txt").unwrap();

    println!("Day14 Part1: {}", part1(&input));
    println!("Day14 Part2: {}", part2(&input));
}

fn tilt(grid: &mut Vec<Vec<char>>) {
    for x in 0..grid[0].len() {
        let mut start = 0;
        let mut rocks = 0;
        for y in 0..grid.len() {
            if grid[y][x] == 'O' {
                rocks += 1;
                grid[y][x] = '.';
            }

            if grid[y][x] == '#' || y == grid.len() - 1 {
                grid.iter_mut()
                    .skip(start)
                    .take(rocks)
                    .for_each(|row| row[x] = 'O');
                start = y + 1;
                rocks = 0;
            }
        }
    }
}

fn rotate(grid: &mut Vec<Vec<char>>) {
    let mut new_grid = vec![vec!['.'; grid.len()]; grid[0].len()];

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            new_grid[x][grid.len() - y - 1] = c;
        }
    }

    *grid = new_grid;
}

fn score(grid: &[Vec<char>]) -> usize {
    let mut sum = 0;

    for (y, row) in grid.iter().rev().enumerate() {
        for &c in row {
            if c == 'O' {
                sum += y + 1;
            }
        }
    }

    sum
}

fn part1(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    tilt(&mut grid);

    score(&grid).to_string()
}

fn part2(input: &str) -> String {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();

    for i in 0..1000000000 {
        if let Some(&index) = cache.get(&grid) {
            let cycle = i - index;
            let remaining = (1000000000 - i) % cycle;

            for _ in 0..remaining {
                for _ in 0..4 {
                    tilt(&mut grid);
                    rotate(&mut grid);
                }
            }

            break;
        }

        cache.insert(grid.clone(), i);

        for _ in 0..4 {
            tilt(&mut grid);
            rotate(&mut grid);
        }
    }

    score(&grid).to_string()
}
