use itertools::iproduct;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res};
use nom::multi::separated_list;
use nom::sequence::{pair, separated_pair};
use std::cmp::min;
use std::io::{stdin, Read};
use std::str::FromStr;
use structopt::StructOpt;

// Types

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone)]
struct Shift {
    axis: Axis,
    distance: i64,
}

#[derive(Copy, Clone)]
struct Point {
    pub x: i64,
    pub y: i64,
}

#[derive(Copy, Clone)]
struct Line {
    axis: Axis,
    distance: i64,
    origin: Point,
}

// Parsing

fn axis_sign(input: &str) -> nom::IResult<&str, (Axis, i64)> {
    alt((
        map(tag("L"), |_| (Axis::X, -1)),
        map(tag("R"), |_| (Axis::X, 1)),
        map(tag("U"), |_| (Axis::Y, -1)),
        map(tag("D"), |_| (Axis::Y, 1)),
    ))(input)
}

fn shift(input: &str) -> nom::IResult<&str, Shift> {
    map(
        pair(axis_sign, map_res(digit1, i64::from_str)),
        |((axis, sign), distance)| Shift {
            axis,
            distance: distance * sign,
        },
    )(input)
}

fn shifts(input: &str) -> nom::IResult<&str, Vec<Shift>> {
    separated_list(tag(","), shift)(input)
}

fn shifts_pair(input: &str) -> Result<(Vec<Shift>, Vec<Shift>), failure::Error> {
    let result = separated_pair(shifts, newline, shifts)(input);

    result.map(|(_, pair)| pair).map_err(|err| match err {
        nom::Err::Error(e) => failure::err_msg(format!("{:?}", e)),
        nom::Err::Failure(e) => failure::err_msg(format!("{:?}", e)),
        nom::Err::Incomplete(needed) => failure::err_msg(format!("{:?}", needed)),
    })
}

// Solving

fn lines(shifts: &Vec<Shift>) -> Vec<Line> {
    shifts
        .iter()
        .scan(Point { x: 0, y: 0 }, |point, shift| {
            let origin = match shift.axis {
                Axis::X => {
                    let old = point.x;
                    point.x = old + shift.distance;
                    Point {
                        x: min(old, point.x),
                        y: point.y,
                    }
                }
                Axis::Y => {
                    let old = point.y;
                    point.y = old + shift.distance;
                    Point {
                        x: point.x,
                        y: min(old, point.y),
                    }
                }
            };

            Some(Line {
                axis: shift.axis,
                origin: origin,
                distance: shift.distance.abs(),
            })
        })
        .collect()
}

fn intersect(first: &Line, second: &Line) -> Option<Point> {
    match (first.axis, second.axis) {
        // these could overlap, but do not in the input
        (Axis::X, Axis::X) | (Axis::Y, Axis::Y) => None,
        (Axis::Y, Axis::X) => intersect(second, first),
        (Axis::X, Axis::Y) => {
            if first.origin.x <= second.origin.x
                && (first.origin.x + first.distance) >= second.origin.x
                && first.origin.y >= second.origin.y
                && first.origin.y <= (second.origin.y + second.distance)
            {
                Some(Point {
                    x: second.origin.x,
                    y: first.origin.y,
                })
            } else {
                None
            }
        }
    }
}

fn intersections(first: &Vec<Line>, second: &Vec<Line>) -> Vec<(Point, usize, usize)> {
    iproduct!(first.iter().enumerate(), second.iter().enumerate())
        .flat_map(|((i1, line1), (i2, line2))| intersect(line1, line2).map(|point| (point, i1, i2)))
        .collect()
}

fn minimum_manhattan(intersections: &Vec<(Point, usize, usize)>) -> Option<i64> {
    intersections
        .iter()
        .map(|(point, _, _)| (point.x.abs() + point.y.abs()))
        .filter(|distance| *distance > 0)
        .min()
}

fn delay(lines: &Vec<Line>, index: usize, point: Point) -> i64 {
    let initial_delay: i64 = lines[0..index].iter().map(|line| line.distance.abs()).sum();

    let last = &lines[index];
    let last_delay = match last.axis {
        Axis::X if last.distance < 0 => last.origin.x - last.distance - point.x,
        Axis::X => point.x - last.origin.x,
        Axis::Y if last.distance < 0 => last.origin.y - last.distance - point.y,
        Axis::Y => point.y - last.origin.y,
    };

    initial_delay + last_delay
}

fn minimum_delay(
    first: &Vec<Line>,
    second: &Vec<Line>,
    intersections: &Vec<(Point, usize, usize)>,
) -> Option<i64> {
    intersections
        .iter()
        .map(|(point, i1, i2)| delay(first, *i1, *point) + delay(second, *i2, *point))
        .filter(|distance| *distance > 0)
        .min()
}

// Main

#[derive(StructOpt)]
pub enum Options {
    A,
    B,
}

pub fn run(options: &Options) -> Result<(), failure::Error> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let instructions = shifts_pair(&input)?;
    let lines = (lines(&instructions.0), lines(&instructions.1));
    let intersections = intersections(&lines.0, &lines.1);

    let minimum = match options {
        Options::A => minimum_manhattan(&intersections),
        Options::B => minimum_delay(&lines.0, &lines.1, &intersections),
    }
    .ok_or_else(|| failure::err_msg("no intersections!"))?;

    println!("{}", minimum);

    Ok(())
}
