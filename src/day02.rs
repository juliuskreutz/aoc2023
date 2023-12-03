use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{map_res, value},
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
    colors: Vec<Color>,
}

#[derive(Clone, Copy)]
enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
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
    Ok((input, Set { colors }))
}

fn parse_color(input: &str) -> IResult<&str, Color> {
    let (input, amount) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = alt((
        value(Color::Red(amount), tag("red")),
        value(Color::Green(amount), tag("green")),
        value(Color::Blue(amount), tag("blue")),
    ))(input)?;
    Ok((input, color))
}

fn part1(input: &str) -> String {
    let (_, games) = parse(input).unwrap();

    let mut count = 0;
    'outer: for game in games {
        for color in game.sets.iter().flat_map(|set| &set.colors).copied() {
            match color {
                Color::Red(amount) => {
                    if amount > 12 {
                        continue 'outer;
                    }
                }
                Color::Green(amount) => {
                    if amount > 13 {
                        continue 'outer;
                    }
                }
                Color::Blue(amount) => {
                    if amount > 14 {
                        continue 'outer;
                    }
                }
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
        let mut red = usize::MIN;
        let mut green = usize::MIN;
        let mut blue = usize::MIN;

        for color in game.sets.iter().flat_map(|set| &set.colors).copied() {
            match color {
                Color::Red(amount) => {
                    red = red.max(amount);
                }
                Color::Green(amount) => {
                    green = green.max(amount);
                }
                Color::Blue(amount) => {
                    blue = blue.max(amount);
                }
            }
        }

        count += red * green * blue;
    }

    count.to_string()
}
