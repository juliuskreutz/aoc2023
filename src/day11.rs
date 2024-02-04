pub fn solve() {
    let input = std::fs::read_to_string("input/day11.txt").unwrap();

    println!("Day11 Part1: {}", part1(&input));
    println!("Day11 Part2: {}", part2(&input));
}

fn calculate(input: &str, extra_space: usize) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut galaxies = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut empty_rows = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.push(y);
        }
    }

    let mut empty_cols = Vec::new();
    for x in 0..grid[0].len() {
        if grid.iter().all(|row| row[x] == '.') {
            empty_cols.push(x);
        }
    }

    let mut sum = 0;
    for (i, &(x1, y1)) in galaxies.iter().enumerate() {
        for &(x2, y2) in &galaxies[i + 1..] {
            let start_x = x1.min(x2);
            let end_x = x1.max(x2);
            let start_y = y1.min(y2);
            let end_y = y1.max(y2);

            // No need for diagonal distance, just go on the axis
            let mut distance = end_x - start_x + end_y - start_y;

            for &empty_row in &empty_rows {
                if (start_y + 1..end_y).contains(&empty_row) {
                    distance += extra_space;
                }
            }

            for &empty_col in &empty_cols {
                if (start_x + 1..end_x).contains(&empty_col) {
                    distance += extra_space;
                }
            }

            sum += distance;
        }
    }

    sum
}

fn part1(input: &str) -> String {
    // Twice as big = 1 extra space
    calculate(input, 1).to_string()
}

fn part2(input: &str) -> String {
    // 1000000 times as big = 999999 extra spaces
    calculate(input, 999999).to_string()
}
