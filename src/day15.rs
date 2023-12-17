use indexmap::IndexMap;

pub fn solve() {
    let input = std::fs::read_to_string("input/day15.txt").unwrap();

    println!("Day15 Part1: {}", part1(&input));
    println!("Day15 Part2: {}", part2(&input));
}

fn hash(input: &str) -> usize {
    let mut sum = 0;

    for step in input.split(',') {
        let mut value = 0usize;

        for c in step.chars() {
            value += c as usize;
            value *= 17;
            value %= 256;
        }

        sum += value;
    }

    sum
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    for step in input.trim_end().split(',') {
        sum += hash(step);
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut boxes = vec![IndexMap::new(); 256];

    for step in input.trim_end().split(',') {
        if let Some(label) = step.strip_suffix('-') {
            let hash = hash(label);

            boxes[hash].shift_remove(label);
        } else {
            let mut split = step.split('=');

            let label = split.next().unwrap();
            let value = split.next().unwrap().parse::<usize>().unwrap();

            let hash = hash(label);

            boxes[hash].insert(label.to_string(), value);
        }
    }

    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (j, &v) in b.values().enumerate() {
            sum += (i + 1) * (j + 1) * v;
        }
    }

    sum.to_string()
}
