use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();

    println!("Day02 Part1: {}", part1(&input));
    println!("Day02 Part2: {}", part2(&input));
}

struct Game {
    id: i32,
    sets: Vec<Set>,
}

struct Set {
    red: i32,
    green: i32,
    blue: i32,
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(newline, parse_game)(input)?;
    Ok((input, games))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(digit1, str::parse::<i32>)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;
    Ok((input, Game { id, sets }))
}

fn parse_set(input: &str) -> IResult<&str, Set> {
    let (input, colors) = separated_list1(tag(", "), parse_color)(input)?;

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for (color, amount) in colors {
        match color {
            "red" => red = amount,
            "green" => green = amount,
            "blue" => blue = amount,
            _ => unreachable!(),
        }
    }

    Ok((input, Set { red, green, blue }))
}

fn parse_color(input: &str) -> IResult<&str, (&str, i32)> {
    let (input, amount) = map_res(digit1, str::parse::<i32>)(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((tag("red"), tag("green"), tag("blue")))(input)?;
    Ok((input, (color, amount)))
}

fn part1(input: &str) -> String {
    let (_, games) = parse(input).unwrap();

    let mut count = 0;
    'outer: for game in games {
        for set in game.sets {
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                continue 'outer;
            }
        }

        count += game.id;
    }

    count.to_string()
}

fn part2(input: &str) -> String {
    let (_, games) = parse(input).unwrap();

    let mut count = 0;
    for game in games {
        let mut red = i32::MIN;
        let mut green = i32::MIN;
        let mut blue = i32::MIN;

        for set in game.sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }

        count += red * green * blue;
    }

    count.to_string()
}
