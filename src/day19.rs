use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, char, digit1, newline},
    combinator::{map, map_res, opt, value},
    multi::separated_list1,
    sequence::{delimited, terminated},
    IResult,
};

pub fn solve() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    // let input = std::fs::read_to_string("input/day19.txt").unwrap();

    println!("Day19 Part1: {}", part1(&input));
    println!("Day19 Part2: {}", part2(&input));
}

#[derive(Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

struct Rule {
    condition: Option<Condition>,
    name: String,
}

#[derive(Clone, Copy)]
enum Op {
    Less,
    Greater,
}

struct Condition {
    category: Category,
    op: Op,
    value: usize,
}

fn parse(input: &str) -> IResult<&str, System> {
    let (input, workflows) = map_res(take_until("\n\n"), |s| {
        separated_list1(newline, parse_workflow)(s)
            .map(|(_, w)| w.into_iter().map(|w| (w.name.clone(), w)).collect())
    })(input)?;
    let (input, _) = tag("\n\n")(input)?;
    let (input, parts) =
        separated_list1(newline, delimited(char('{'), parse_part, char('}')))(input)?;

    Ok((input, System { workflows, parts }))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, name) = map(take_until("{"), str::to_string)(input)?;
    let (input, rules) =
        delimited(char('{'), separated_list1(char(','), parse_rule), char('}'))(input)?;

    Ok((input, Workflow { name, rules }))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, condition) = opt(parse_condition)(input)?;
    let (input, name) = map(alpha1, str::to_string)(input)?;

    Ok((input, Rule { condition, name }))
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    let (input, category) = alt((
        value(Category::X, char('x')),
        value(Category::M, char('m')),
        value(Category::A, char('a')),
        value(Category::S, char('s')),
    ))(input)?;
    let (input, op) = alt((value(Op::Less, char('<')), value(Op::Greater, char('>'))))(input)?;
    let (input, value) = map_res(terminated(digit1, char(':')), str::parse::<usize>)(input)?;

    Ok((
        input,
        Condition {
            category,
            op,
            value,
        },
    ))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = map_res(digit1, str::parse::<usize>)(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = map_res(digit1, str::parse::<usize>)(input)?;

    Ok((input, Part { x, m, a, s }))
}

fn part1(input: &str) -> String {
    let (_, system) = parse(input).unwrap();

    let mut sum = 0;
    for part in system.parts {
        let mut name = "in";

        loop {
            if name == "A" {
                sum += part.x;
                sum += part.m;
                sum += part.a;
                sum += part.s;
                break;
            } else if name == "R" {
                break;
            }

            let current = &system.workflows[name];

            for rule in &current.rules {
                if let Some(condition) = &rule.condition {
                    let value = match condition.category {
                        Category::X => part.x,
                        Category::M => part.m,
                        Category::A => part.a,
                        Category::S => part.s,
                    };

                    match condition.op {
                        Op::Less if value < condition.value => {
                            name = &rule.name;
                            break;
                        }
                        Op::Greater if value > condition.value => {
                            name = &rule.name;
                            break;
                        }
                        _ => {}
                    }
                } else {
                    name = &rule.name;
                    break;
                }
            }
        }
    }

    sum.to_string()
}

fn part2(input: &str) -> String {
    let mut sum = 0;

    let (_, system) = parse(input).unwrap();

    let mut stack = Vec::new();
    stack.push(("in", 1..4001, 1..4001, 1..4001, 1..4001));

    let mut visited = HashSet::new();

    while let Some((node, mut x, mut m, mut a, mut s)) = stack.pop() {
        if node == "A" {
            sum += x.len() * m.len() * a.len() * s.len();
            continue;
        } else if node == "R" {
            continue;
        }

        if !visited.insert(node) {
            continue;
        }

        let current = &system.workflows[node];

        for rule in &current.rules {
            if let Some(condition) = &rule.condition {
                match condition.op {
                    Op::Less => {
                        let mut new_x = x.clone();
                        let mut new_m = m.clone();
                        let mut new_a = a.clone();
                        let mut new_s = s.clone();

                        match condition.category {
                            Category::X if new_x.end > condition.value => {
                                new_x = new_x.start..condition.value;
                                x = condition.value..x.end + 1;
                            }
                            Category::M if new_m.end > condition.value => {
                                new_m = new_m.start..condition.value;
                                m = condition.value..m.end + 1;
                            }
                            Category::A if new_a.end > condition.value => {
                                new_a = new_a.start..condition.value;
                                a = condition.value..a.end + 1;
                            }
                            Category::S if new_s.end > condition.value => {
                                new_s = new_s.start..condition.value;
                                s = condition.value..s.end + 1;
                            }
                            _ => {}
                        };

                        stack.push((&rule.name, new_x, new_m, new_a, new_s));
                    }
                    Op::Greater => {
                        let mut new_x = x.clone();
                        let mut new_m = m.clone();
                        let mut new_a = a.clone();
                        let mut new_s = s.clone();

                        match condition.category {
                            Category::X if new_x.start <= condition.value => {
                                new_x = condition.value + 1..new_x.end + 1;
                                x = x.start..condition.value;
                            }
                            Category::M if new_m.start <= condition.value => {
                                new_m = condition.value + 1..new_m.end + 1;
                                m = m.start..condition.value;
                            }
                            Category::A if new_a.start <= condition.value => {
                                new_a = condition.value + 1..new_a.end + 1;
                                a = a.start..condition.value;
                            }
                            Category::S if new_s.start <= condition.value => {
                                new_s = condition.value + 1..new_s.end + 1;
                                s = s.start..condition.value;
                            }
                            _ => {}
                        };

                        stack.push((&rule.name, new_x, new_m, new_a, new_s));
                    }
                }
            } else {
                stack.push((&rule.name, x, m, a, s));
                break;
            }
        }
    }

    sum.to_string()
}
