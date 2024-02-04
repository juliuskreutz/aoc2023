use pathfinding::directed::dijkstra::dijkstra;

pub fn solve() {
    let input = std::fs::read_to_string("input/day17.txt").unwrap();

    println!("Day17 Part1: {}", part1(&input));
    println!("Day17 Part2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    direction: usize,
    straight: usize,
}

fn heat_loss(grid: &[Vec<u32>], min: usize, max: usize) -> u32 {
    let indices = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let result = dijkstra(
        &Point {
            x: 0,
            y: 0,
            direction: 5,
            straight: 1,
        },
        |&Point {
             x,
             y,
             direction,
             straight,
         }| {
            let mut successors = Vec::new();

            let mut paths = Vec::new();
            if direction == 5 {
                paths.push((1, 1));
                paths.push((2, 1));
            } else {
                if straight < max {
                    paths.push((direction, straight + 1));
                }

                if straight >= min {
                    paths.push(((direction + 1) % 4, 1));
                    paths.push(((direction + 3) % 4, 1));
                }
            }

            for (direction, straight) in paths {
                let (dx, dy) = indices[direction];

                let x = x as i32 + dx;
                let y = y as i32 + dy;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as usize;
                let y = y as usize;

                if y >= grid.len() || x >= grid[y].len() {
                    continue;
                }

                successors.push((
                    Point {
                        x,
                        y,
                        direction,
                        straight,
                    },
                    grid[y][x],
                ));
            }

            successors
        },
        |&Point { x, y, straight, .. }| {
            x == grid[y].len() - 1 && y == grid.len() - 1 && straight >= min
        },
    );

    result.unwrap().1
}

fn part1(input: &str) -> String {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    heat_loss(&grid, 0, 3).to_string()
}

fn part2(input: &str) -> String {
    let grid = input
        .lines()
        .map(|l| l.chars().flat_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    heat_loss(&grid, 4, 10).to_string()
}
