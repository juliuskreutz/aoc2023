pub fn solve() {
    let input = std::fs::read_to_string("input/day06.txt").unwrap();

    println!("Day06 Part1: {}", part1(&input));
    println!("Day06 Part2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();

    let times = lines.next().unwrap()[5..]
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>();

    let distances = lines.next().unwrap()[9..]
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .collect::<Vec<_>>();

    let mut product = 1;
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for hold in 0..time {
            if (time - hold) * hold > distance {
                count += 1;
            }
        }

        product *= count;
    }

    product.to_string()
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();

    let time = lines.next().unwrap()[5..]
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();
    let distance = lines.next().unwrap()[9..]
        .replace(' ', "")
        .parse::<usize>()
        .unwrap();

    let mut count = 0;
    for hold in 0..time {
        if (time - hold) * hold > distance {
            count += 1;
        }
    }

    count.to_string()
}
