use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day05.txt").unwrap();

    println!("Day05 Part1: {}", part1(&input));
    println!("Day05 Part2: {}", part2(&input));
}

type Almanac = Vec<(Range<usize>, Range<usize>)>;

struct Garden {
    seeds: Vec<usize>,
    almanacs: Vec<Almanac>,
}

fn parse(input: &str) -> IResult<&str, Garden> {
    let mut parts = input.split("\n\n");

    let (_, seeds) = parse_seeds(parts.next().unwrap())?;

    let mut almanacs = Vec::new();
    for part in parts {
        let (_, a) = parse_almanac(part)?;
        almanacs.push(a);
    }

    Ok(("", Garden { seeds, almanacs }))
}

fn parse_seeds(input: &str) -> IResult<&str, Vec<usize>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) =
        separated_list1(space1, map_res(digit1, |s: &str| s.parse::<usize>()))(input)?;
    Ok((input, seeds))
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = newline(input)?;
    let (input, almanacs) = separated_list1(newline, parse_almanac_entry)(input)?;
    Ok((input, almanacs))
}

fn parse_almanac_entry(input: &str) -> IResult<&str, (Range<usize>, Range<usize>)> {
    let (input, destination_start) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = space1(input)?;
    let (input, source_start) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    let (input, _) = space1(input)?;
    let (input, len) = map_res(digit1, |s: &str| s.parse::<usize>())(input)?;
    Ok((
        input,
        (
            destination_start..destination_start + len,
            source_start..source_start + len,
        ),
    ))
}

fn part1(input: &str) -> String {
    let (_, garden) = parse(input).unwrap();

    let mut lowest = usize::MAX;
    for seed in garden.seeds {
        let mut location = seed;
        for almanac in &garden.almanacs {
            for (destination, source) in almanac {
                if source.contains(&location) {
                    location += destination.start - source.start;
                    break;
                }
            }
        }

        if location < lowest {
            lowest = location;
        }
    }

    lowest.to_string()
}

fn part2(input: &str) -> String {
    let (_, garden) = parse(input).unwrap();

    let mut seeds = garden
        .seeds
        .chunks(2)
        .map(|s| s[0]..s[0] + s[1])
        .collect::<Vec<_>>();

    for almanac in &garden.almanacs {
        let mut mapped_seeds = Vec::new();

        while let Some(seed) = seeds.pop() {
            let mut mapped = false;
            for (destination, source) in almanac {
                // Not in range
                if seed.end <= source.start || source.end <= seed.start {
                    continue;
                }

                let offset = destination.start - source.start;

                // Inside range
                mapped_seeds
                    .push(seed.start.max(source.start) + offset..seed.end.min(source.end) + offset);

                // Left range
                if seed.start < source.start {
                    seeds.push(seed.start..source.start);
                }

                // Right range
                if source.end < seed.end {
                    seeds.push(source.end..seed.end);
                }

                mapped = true;
            }

            if !mapped {
                mapped_seeds.push(seed);
            }
        }

        seeds = mapped_seeds;
    }

    seeds.iter().map(|s| s.start).min().unwrap().to_string()
}
