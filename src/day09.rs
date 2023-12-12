use nom::{
    bytes::complete::take_while1,
    character::complete::{newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};

pub fn solve() {
    let input = std::fs::read_to_string("input/day09.txt").unwrap();

    println!("Day09 Part1: {}", part1(&input));
    println!("Day09 Part2: {}", part2(&input));
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i64>>> {
    let (input, data) = separated_list1(
        newline,
        separated_list1(
            space1,
            map_res(
                take_while1(|c: char| c.is_ascii_digit() || c == '-'),
                str::parse,
            ),
        ),
    )(input)?;
    Ok((input, data))
}

fn part1(input: &str) -> String {
    let (_, mut data) = parse(input).unwrap();

    let mut sum = 0;
    for history in data.iter_mut() {
        while !history.iter().all(|&i| i == 0) {
            for i in 0..(history.len() - 1) {
                history[i] = history[i + 1] - history[i];
            }
            sum += history.pop().unwrap();
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let (_, mut data) = parse(input).unwrap();

    let mut sum = 0;
    for history in data.iter_mut() {
        history.reverse();

        let mut sign = 1;
        while !history.iter().all(|&i| i == 0) {
            for i in 0..(history.len() - 1) {
                history[i] -= history[i + 1];
            }

            let x = history.pop().unwrap();

            sum += x * sign;
            sign *= -1;
        }
    }

    sum.to_string()
}
