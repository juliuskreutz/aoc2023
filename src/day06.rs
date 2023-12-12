use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day06.txt").unwrap();

    println!("Day06 Part1: {}", part1(&input));
    println!("Day06 Part2: {}", part2(&input));
}

struct Race {
    times: Vec<usize>,
    distances: Vec<usize>,
}

fn parse_part1(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, map_res(digit1, str::parse))(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, map_res(digit1, str::parse))(input)?;
    Ok((input, Race { times, distances }))
}

fn parse_part2(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, time) = map_res(separated_list1(space1, digit1), |v| {
        v.join("").parse::<usize>()
    })(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distance) = map_res(separated_list1(space1, digit1), |v| {
        v.join("").parse::<usize>()
    })(input)?;
    Ok((
        input,
        Race {
            times: vec![time],
            distances: vec![distance],
        },
    ))
}

fn part1(input: &str) -> String {
    let (_, race) = parse_part1(input).unwrap();

    let mut product = 1;
    for (&time, &distance) in race.times.iter().zip(race.distances.iter()) {
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
    let (_, race) = parse_part2(input).unwrap();

    let time = race.times[0];
    let distance = race.distances[0];

    let mut count = 0;
    for hold in 0..time {
        if (time - hold) * hold > distance {
            count += 1;
        }
    }

    count.to_string()
}
